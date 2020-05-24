use anyhow::Result;
use pest::{iterators::Pairs, Parser as PestParser};
use serde::Serialize;
use std::default::Default;

use crate::error::Error;
use crate::parser::Parser;
use crate::Rule;

pub mod element;
pub mod formatting;
pub mod node;

use element::{Element, ElementVariant};
use node::Node;

/// These are all of the types that the parsed html tree can have.
#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DomVariant {
    /// This means that the parsed html had the representation that of an html document. The doctype is optional but a document should only have one root node with the name of html.
    Document,
    /// A document fragment means that the parsed html did not have the representation of a document. A fragment can have multiple root nodes of any name except html, body or head.
    DocumentFragment,
    /// An empty dom means that no normal or text elements where found inside of the parsed html string
    Empty,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Dom {
    /// The type of the tree that was parsed
    pub tree_type: DomVariant,

    /// All of the root nodes in the tree
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<Node>,
}

impl Default for Dom {
    fn default() -> Self {
        Self {
            tree_type: DomVariant::Empty,
            nodes: vec![],
        }
    }
}

impl Dom {
    pub fn parse(input: &str) -> Result<Self> {
        let pairs = match Parser::parse(Rule::html, input) {
            Ok(pairs) => pairs,
            Err(error) => return formatting::error_msg(error),
        };
        Self::build_dom(pairs)
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    pub fn to_json_pretty(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    fn build_dom(pairs: Pairs<Rule>) -> Result<Self> {
        let mut dom = Self::default();
        for pair in pairs {
            match pair.as_rule() {
                Rule::doctype => {
                    dom.tree_type = DomVariant::DocumentFragment;
                }
                Rule::node_element => {
                    if let Some(node) = Self::build_node_element(pair.into_inner())? {
                        dom.nodes.push(node);
                    }
                }
                Rule::node_text => {
                    dom.nodes.push(Node::Text(pair.as_str().to_string()));
                }
                Rule::EOI => break,
                _ => unreachable!("[build dom] unknown rule: {:?}", pair.as_rule()),
            };
        }

        // TODO: This needs to be cleaned up
        // What logic should apply when parsing fragment vs document?
        // I had some of this logic inside the grammar before, but i thought it would be a bit clearer
        // to just have everyting here when we construct the dom
        match dom.nodes.len() {
            0 => {
                dom.tree_type = DomVariant::Empty;
                Ok(dom)
            }
            1 => match dom.nodes[0] {
                Node::Element(ref el) => {
                    let name = el.name.to_lowercase();
                    if name == "html" {
                        dom.tree_type = DomVariant::Document;
                        Ok(dom)
                    } else if dom.tree_type == DomVariant::Document && name != "html" {
                        Err(
                            Error::Parsing("A document can only have html as root".to_string())
                                .into(),
                        )
                    } else {
                        dom.tree_type = DomVariant::DocumentFragment;
                        Ok(dom)
                    }
                }
                _ => {
                    dom.tree_type = DomVariant::DocumentFragment;
                    Ok(dom)
                }
            },
            _ => {
                dom.tree_type = DomVariant::DocumentFragment;
                for node in &dom.nodes {
                    if let Node::Element(ref el) = node {
                        let name = el.name.clone().to_lowercase();
                        if name == "html" || name == "body" || name == "head" {
                            return Err(Error::Parsing(format!(
                                "A document fragment should not include {}",
                                name
                            ))
                            .into());
                        }
                    }
                }
                Ok(dom)
            }
        }
    }

    fn build_node_element(pairs: Pairs<Rule>) -> Result<Option<Node>> {
        let mut element = Element::default();
        for pair in pairs {
            match pair.as_rule() {
                Rule::node_element | Rule::el_raw_text => {
                    if let Some(child_element) = Self::build_node_element(pair.into_inner())? {
                        element.nodes.push(child_element)
                    }
                }
                Rule::node_text | Rule::el_raw_text_content => {
                    element.nodes.push(Node::Text(pair.as_str().to_string()));
                }
                Rule::el_name | Rule::el_void_name | Rule::el_raw_text_name => {
                    element.name = pair.as_str().to_string();
                }
                Rule::attr => {
                    let new_attribute = Self::build_attribute(pair.into_inner())?;
                    match new_attribute.0.as_str() {
                        "id" => element.id = new_attribute.1,
                        "class" => {
                            if let Some(classes) = new_attribute.1 {
                                let classes =
                                    classes.split_whitespace().into_iter().collect::<Vec<_>>();
                                for class in classes {
                                    element.classes.push(class.to_string());
                                }
                            }
                        }
                        _ => {
                            element.attributes.insert(new_attribute.0, new_attribute.1);
                        }
                    };
                }
                Rule::el_normal_end => {
                    element.variant = ElementVariant::Normal;
                    break;
                }
                Rule::el_dangling => (),
                Rule::EOI => (),
                _ => unreachable!("unknown tpl rule: {:?}", pair.as_rule()),
            }
        }
        if element.name != "" {
            Ok(Some(Node::Element(element)))
        } else {
            Ok(None)
        }
    }

    fn build_attribute(pairs: Pairs<Rule>) -> Result<(String, Option<String>)> {
        let mut attribute = ("".to_string(), None);
        for pair in pairs {
            match pair.as_rule() {
                Rule::attr_key => {
                    attribute.0 = pair.as_str().to_string();
                }
                Rule::attr_value | Rule::attr_non_quoted => {
                    attribute.1 = Some(pair.as_str().to_string());
                }
                _ => unreachable!("unknown tpl rule: {:?}", pair.as_rule()),
            }
        }
        Ok(attribute)
    }
}
