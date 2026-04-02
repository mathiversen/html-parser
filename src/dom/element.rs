use super::node::{Node, write_html_list};
#[cfg(feature = "source-span")]
use super::span::SourceSpan;
use crate::dom::vecmap::VecMap;
use crate::for_each::ForEach;
use crate::{Attribute, Text, VecSet};
use html_escape::decode_html_entities;
use ownable::{IntoOwned, ToBorrowed, ToOwned};
#[cfg(feature = "test")]
use serde::Serialize;
use std::borrow::Cow;
use std::default::Default;

/// Normal: `<div></div>` or Void: `<meta/>`and `<meta>`
#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "test", derive(Debug, Serialize))]
#[cfg_attr(feature = "test", serde(rename_all = "camelCase"))]
// TODO: Align with: https://html.spec.whatwg.org/multipage/syntax.html#elements-2
pub enum ElementVariant {
    /// A normal element can have children, ex: <div></div>.
    Normal,
    /// A void element can't have children, ex: <meta /> and <meta>
    Void,
}

pub type Attributes<'a> = VecMap<Cow<'a, str>, Option<Attribute<'a>>>;

/// Most of the parsed html nodes are elements, except for text
#[derive(PartialEq, IntoOwned, ToBorrowed, ToOwned)]
#[cfg_attr(feature = "test", derive(Debug, Serialize))]
#[cfg_attr(feature = "test", serde(rename_all = "camelCase"))]
pub struct Element<'a> {
    /// The id of the element
    #[cfg_attr(feature = "test", serde(skip_serializing_if = "Option::is_none"))]
    pub id: Option<Attribute<'a>>,

    /// The name / tag of the element
    pub name: Cow<'a, str>,

    /// The element variant, if it is of type void or not
    #[ownable(clone)]
    pub variant: ElementVariant,

    /// All of the elements attributes, except id and class
    #[cfg_attr(feature = "test", serde(skip_serializing_if = "VecMap::is_empty"))]
    pub attributes: Attributes<'a>,

    /// All of the elements classes
    #[cfg_attr(feature = "test", serde(skip_serializing_if = "VecSet::is_empty"))]
    pub classes: VecSet<Attribute<'a>>,

    /// All of the elements child nodes
    #[cfg_attr(feature = "test", serde(skip_serializing_if = "Vec::is_empty"))]
    pub children: Vec<Node<'a>>,

    #[cfg(feature = "source-span")]
    /// Span of the element in the parsed source
    #[cfg_attr(feature = "test", serde(skip))]
    pub source_span: SourceSpan<'a>,
}

impl Default for Element<'static> {
    fn default() -> Self {
        Self {
            id: None,
            name: "".into(),
            variant: ElementVariant::Void,
            classes: Default::default(),
            attributes: Default::default(),
            children: vec![],
            #[cfg(feature = "source-span")]
            source_span: SourceSpan::default(),
        }
    }
}

impl<'a> ForEach<'a> for Element<'a> {
    #[inline]
    fn root(&self) -> &[Node<'a>] {
        self.children.as_slice()
    }

    #[inline]
    fn root_mut(&mut self) -> &mut [Node<'a>] {
        self.children.as_mut_slice()
    }
}

impl Element<'_> {
    #[inline]
    pub fn has_class(&self, class: &str) -> bool {
        self.classes.contains_key(class)
    }

    #[inline(always)]
    pub fn to_html(&self) -> String {
        let mut result = String::new();
        self.write_html(&mut result);
        result
    }

    pub fn write_html(&self, writer: &mut String) {
        let e = self;

        writer.push('<');
        writer.push_str(&e.name);
        if !e.classes.is_empty() {
            writer.push_str(" class=\"");
            writer.push_str(
                &e.classes
                    .iter()
                    .map(|c| c.as_str())
                    .collect::<Vec<_>>()
                    .join(" "),
            );
            writer.push('"');
        }
        if let Some(id) = &e.id {
            writer.push(' ');
            writer.push_str("id");
            writer.push('=');
            writer.push('"');
            writer.push_str(id);
            writer.push('"');
        }
        for (k, v) in e.attributes.iter() {
            writer.push(' ');
            writer.push_str(k);
            if let Some(v) = v {
                writer.push('=');
                writer.push('"');
                writer.push_str(v);
                writer.push('"');
            }
        }
        if e.variant == ElementVariant::Normal {
            writer.push('>');
            write_html_list(writer, &e.children);
            writer.push('<');
            writer.push('/');
            writer.push_str(&e.name);
            writer.push('>');
        } else {
            writer.push('/');
            writer.push('>');
        }
    }

    /// Strip all tags.
    ///
    /// Note that html entities are not removed, only tags.
    /// Use [`Self::to_text`], or [`Text::decode`] on the result.
    pub fn strip_tags(&self) -> Text<'_> {
        if let &[Node::Text(t)] = &self.children.as_slice() {
            t.to_borrowed()
        } else {
            let mut result = String::new();
            self.write_strip_tags(&mut result);
            Text(result.into())
        }
    }

    fn write_strip_tags(&self, writer: &mut String) {
        for c in &self.children {
            match c {
                Node::Text(t) => writer.push_str(t),
                Node::Element(e) => e.write_strip_tags(writer),
                Node::Comment(_) => (),
            }
        }
    }

    /// Strip tags and decode html-entities.
    pub fn to_text(&self) -> Cow<'_, str> {
        let t = self.strip_tags().0;
        match decode_html_entities(&t) {
            Cow::Borrowed(_) => t, // it's only returned as borrowed if the whole input is passed though
            Cow::Owned(o) => Cow::Owned(o),
        }
    }
}
