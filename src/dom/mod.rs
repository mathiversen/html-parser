use ownable::{IntoOwned, ToBorrowed, ToOwned};
use pest::{Parser, iterators::Pair, iterators::Pairs};
#[cfg(feature = "test")]
use serde::Serialize;
use std::borrow::Cow;
use std::default::Default;

use crate::Rule;
use crate::error::Error;
use crate::grammar::Grammar;
use crate::{Attribute, Result, Text};

pub mod element;
pub mod formatting;
pub mod html;
pub mod node;
#[cfg(feature = "source-span")]
pub mod span;
pub mod vecmap;
pub mod vecset;

use crate::dom::node::write_html_list;
#[cfg(feature = "source-span")]
use crate::dom::span::SourceSpan;
use crate::for_each::ForEach;
use element::{Element, ElementVariant};
use node::Node;

/// Document, DocumentFragment or Empty
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "test", derive(Debug, Serialize))]
#[cfg_attr(feature = "test", serde(rename_all = "camelCase"))]
pub enum DomVariant {
    /// This means that the parsed html had the representation of an html document. The doctype is optional but a document should only have one root node with the name of html.
    /// Example:
    /// ```text
    /// <!doctype html>
    /// <html>
    ///     <head></head>
    ///     <body>
    ///         <h1>Hello world</h1>
    ///     </body>
    /// </html>
    /// ```
    Document,
    /// A document fragment means that the parsed html did not have the representation of a document. A fragment can have multiple root children of any name except html, body or head.
    /// Example:
    /// ```text
    /// <h1>Hello world</h1>
    /// ```
    DocumentFragment,
    /// An empty dom means that the input was empty
    Empty,
}

/// **The main struct** & the result of the parsed html
#[derive(PartialEq, ToBorrowed, ToOwned, IntoOwned)]
#[cfg_attr(feature = "test", derive(Debug, Serialize))]
#[cfg_attr(feature = "test", serde(rename_all = "camelCase"))]
pub struct Dom<'a> {
    /// The type of the tree that was parsed
    #[ownable(clone)]
    pub tree_type: DomVariant,

    /// All of the root children in the tree
    #[cfg_attr(feature = "test", serde(skip_serializing_if = "Vec::is_empty"))]
    pub children: Vec<Node<'a>>,

    /// A collection of all errors during parsing
    #[cfg_attr(feature = "test", serde(skip_serializing))]
    #[ownable(clone)]
    pub errors: Vec<String>,
}

impl<'a> Default for Dom<'a> {
    fn default() -> Self {
        Self {
            tree_type: DomVariant::Empty,
            children: vec![],
            errors: vec![],
        }
    }
}

impl<'a> Dom<'a> {
    pub fn parse(input: &'a str) -> Result<Self> {
        let pairs = match Grammar::parse(Rule::html, input) {
            Ok(pairs) => pairs,
            Err(error) => return formatting::error_msg(error),
        };
        Self::build_dom(pairs)
    }

    #[cfg(feature = "test")]
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    #[cfg(feature = "test")]
    pub fn to_json_pretty(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    fn build_dom(pairs: Pairs<'a, Rule>) -> Result<Self> {
        let mut dom = Self::default();

        // NOTE: The logic is roughly as follows:
        // 1) A document containing nothing but comments is DomVariant::Empty even though it will have
        //    children in this first pass.  We fix this in the next section.  This allows us to use
        //    DomVariant::Empty to indicate "we haven't decided the type yet".
        // 2) If the type is DomVariant::Empty _so far_, then it can be changed to DomVariant::Document
        //    or DomVariant::DocumentFragment.  DomVariant is only selected in this stage if we see a
        //    DOCTYPE tag.  Comments do not change the type.
        // 3) If the type is non-empty, we don't re-set the type.  We do look for conflicts between
        //    the type and the tokens in the next stage.
        for pair in pairs {
            match pair.as_rule() {
                // A <!DOCTYPE> tag means a full-fledged document.  Note that because of the way
                // the grammar is written, we will only get this token if the <!DOCTYPE> occurs
                // before any other tag; otherwise it will be parsed as a custom tag.
                Rule::doctype => {
                    if dom.tree_type == DomVariant::Empty {
                        dom.tree_type = DomVariant::Document;
                    }
                }

                // If we see an element, build the sub-tree and add it as a child.  If we don't
                // have a document type yet (i.e. "empty"), select DocumentFragment
                Rule::node_element => match Self::build_node_element(pair, &mut dom) {
                    Ok(el) => {
                        if let Some(node) = el {
                            if dom.tree_type == DomVariant::Empty {
                                dom.tree_type = DomVariant::DocumentFragment;
                            };
                            dom.children.push(node);
                        }
                    }
                    Err(error) => {
                        dom.errors.push(format!("{}", error));
                    }
                },

                // Similar to an element, we add it as a child and select DocumentFragment if we
                // don't already have a document type.
                Rule::node_text => {
                    if dom.tree_type == DomVariant::Empty {
                        dom.tree_type = DomVariant::DocumentFragment;
                    }
                    let text = pair.as_str();
                    if !text.trim().is_empty() {
                        dom.children.push(Node::Text(Text(text.into())));
                    }
                }

                // Store comments as a child, but it doesn't affect the document type selection
                // until the next phase (validation).
                Rule::node_comment => {
                    dom.children
                        .push(Node::Comment(Text(pair.into_inner().as_str().into())));
                }

                // Ignore 'end of input', which then allows the catch-all unreachable!() arm to
                // function properly.
                Rule::EOI => (),

                // This should be unreachable, due to the way the grammar is written
                _ => unreachable!("[build dom] unknown rule: {:?}", pair.as_rule()),
            };
        }

        // Implement some checks on the generated dom's data and initial type.  The type may be
        // modified in this section.
        match dom.tree_type {
            // A DomVariant::Empty can only have comments. Anything else is an error.
            DomVariant::Empty => {
                for node in &dom.children {
                    if let Node::Comment(_) = node {
                        // An "empty" document, but it has comments - this is where we cleanup the
                        // earlier assumption that a document with only comments is "empty".
                        // Really, it is a "fragment".
                        dom.tree_type = DomVariant::DocumentFragment
                    } else {
                        // Anything else (i.e. Text() or Element() ) can't happen at the top level;
                        // if we had seen one, we would have set the document type above
                        unreachable!(
                            "[build dom] empty document with an Element {:?}",
                            node.to_html()
                        )
                    }
                }
            }

            // A DomVariant::Document can only have comments and an <HTML> node at the top level.
            // Only one <HTML> tag is permitted.
            DomVariant::Document => {
                if dom
                    .children
                    .iter()
                    .filter(|x| matches!(x, Node::Element(el) if el.name.to_lowercase() == "html"))
                    .count()
                    > 1
                {
                    return Err(Error::Parsing(
                        "Document with multiple HTML tags".to_string(),
                    ));
                }
            }

            // A DomVariant::DocumentFragment should not have <HEAD>, or <BODY> tags at the
            // top-level.  If we find an <HTML> tag, then we consider this a Document instead (if
            // it comes before any other elements, and if there is only one <HTML> tag).
            DomVariant::DocumentFragment => {
                let mut seen_html = false;
                let mut seen_elements = false;

                for node in &dom.children {
                    match node {
                        // Nodes other than <HTML> - reject <HEAD> and <BODY>
                        Node::Element(el) if el.name.clone().to_lowercase() != "html" => {
                            if el.name == "head" || el.name == "body" {
                                return Err(Error::Parsing(format!(
                                    "A document fragment should not include {}",
                                    el.name
                                )));
                            }
                            seen_elements = true;
                        }
                        // <HTML> Nodes - one (before any other elements) is okay
                        Node::Element(el) if el.name.clone().to_lowercase() == "html" => {
                            if seen_html || seen_elements {
                                return Err(Error::Parsing(format!(
                                    "A document fragment should not include {}",
                                    el.name
                                )));
                            };

                            // A fragment with just an <HTML> tag is a document
                            dom.tree_type = DomVariant::Document;
                            seen_html = true;
                        }
                        // Comment() and Text() nodes are permitted at the top-level of a
                        // DocumentFragment
                        _ => (),
                    }
                }
            }
        }

        // The result is the validated tree
        Ok(dom)
    }

    fn build_node_element(pair: Pair<'a, Rule>, dom: &mut Dom) -> Result<Option<Node<'a>>> {
        let mut element = Element {
            #[cfg(feature = "source-span")]
            source_span: {
                let pair_span = pair.as_span();
                let (start_line, start_column) = pair_span.start_pos().line_col();
                let (end_line, end_column) = pair_span.end_pos().line_col();

                SourceSpan::new(
                    pair_span.as_str(),
                    start_line,
                    end_line,
                    start_column,
                    end_column,
                )
            },
            ..Element::default()
        };

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::node_element | Rule::el_raw_text => {
                    match Self::build_node_element(pair, dom) {
                        Ok(el) => {
                            if let Some(child_element) = el {
                                element.children.push(child_element)
                            }
                        }
                        Err(error) => {
                            dom.errors.push(format!("{}", error));
                        }
                    }
                }
                Rule::node_text | Rule::el_raw_text_content => {
                    let text = pair.as_str();
                    if !text.trim().is_empty() {
                        element.children.push(Node::Text(Text(text.into())));
                    }
                }
                Rule::node_comment => {
                    element
                        .children
                        .push(Node::Comment(Text(pair.into_inner().as_str().into())));
                }
                // TODO: To enable some kind of validation we should probably align this with
                // https://html.spec.whatwg.org/multipage/syntax.html#elements-2
                // Also see element variants
                Rule::el_name | Rule::el_void_name | Rule::el_raw_text_name => {
                    element.name = pair.as_str().into();
                }
                Rule::attr => match Self::build_attribute(pair.into_inner()) {
                    Ok((attr_key, attr_value)) => {
                        match attr_key {
                            "id" => element.id = attr_value.map(|s| Attribute(s.into())),
                            "class" => {
                                if let Some(classes) = attr_value {
                                    let classes = classes.split_whitespace().collect::<Vec<_>>();
                                    for class in classes {
                                        element.classes.insert(Attribute(class.into()));
                                    }
                                }
                            }
                            _ => {
                                element.attributes.insert(
                                    Cow::Borrowed(attr_key),
                                    attr_value.map(|s| Attribute(s.into())),
                                );
                            }
                        };
                    }
                    Err(error) => {
                        dom.errors.push(format!("{}", error));
                    }
                },
                Rule::el_normal_end | Rule::el_raw_text_end => {
                    element.variant = ElementVariant::Normal;
                    break;
                }
                Rule::el_dangling => (),
                Rule::EOI => (),
                _ => {
                    return Err(Error::Parsing(format!(
                        "Failed to create element at rule: {:?}",
                        pair.as_rule()
                    )));
                }
            }
        }
        if !element.name.is_empty() {
            Ok(Some(Node::Element(element)))
        } else {
            Ok(None)
        }
    }

    fn build_attribute(pairs: Pairs<'_, Rule>) -> Result<(&str, Option<&str>)> {
        let mut attribute = ("", None);
        for pair in pairs {
            match pair.as_rule() {
                Rule::attr_key => {
                    attribute.0 = pair.as_str().trim();
                }
                Rule::attr_non_quoted => {
                    attribute.1 = Some(pair.as_str().trim());
                }
                Rule::attr_quoted => {
                    let inner_pair = pair.into_inner().next().expect("attribute value");

                    match inner_pair.as_rule() {
                        Rule::attr_value => attribute.1 = Some(inner_pair.as_str()),
                        _ => {
                            return Err(Error::Parsing(format!(
                                "Failed to parse attr value: {:?}",
                                inner_pair.as_rule()
                            )));
                        }
                    }
                }
                _ => {
                    return Err(Error::Parsing(format!(
                        "Failed to parse attr: {:?}",
                        pair.as_rule()
                    )));
                }
            }
        }
        Ok(attribute)
    }

    #[inline(always)]
    pub fn to_html(&self) -> String {
        let mut result = String::new();
        self.write_html(&mut result);
        result
    }

    #[inline(always)]
    pub fn write_html(&self, writer: &mut String) {
        write_html_list(writer, self.children.as_slice());
    }
}

impl<'a> ForEach<'a> for Dom<'a> {
    #[inline]
    fn root(&self) -> &[Node<'a>] {
        self.children.as_slice()
    }

    #[inline]
    fn root_mut(&mut self) -> &mut [Node<'a>] {
        self.children.as_mut_slice()
    }
}
