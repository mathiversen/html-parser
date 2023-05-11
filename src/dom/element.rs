use super::node::Node;
use super::span::SourceSpan;
use serde::{Serialize, Serializer};
use std::collections::{BTreeMap, HashMap};
use std::default::Default;
use std::result::Result;

/// Normal: `<div></div>` or Void: `<meta/>`and `<meta>`
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
// TODO: Align with: https://html.spec.whatwg.org/multipage/syntax.html#elements-2
pub enum ElementVariant {
    /// A normal element can have children, ex: <div></div>.
    Normal,
    /// A void element can't have children, ex: <meta /> and <meta>
    Void,
}

pub type Attributes = HashMap<String, Option<String>>;

/// Most of the parsed html nodes are elements, except for text
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    /// The id of the element
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name / tag of the element
    pub name: String,

    /// The element variant, if it is of type void or not
    pub variant: ElementVariant,

    /// All of the elements attributes, except id and class
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(serialize_with = "ordered_map")]
    pub attributes: Attributes,

    /// All of the elements classes
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub classes: Vec<String>,

    /// All of the elements child nodes
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Node>,

    /// Span of the element in the parsed source
    #[serde(skip)]
    pub source_span: SourceSpan,
}

impl Element {
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        match self.variant {
            ElementVariant::Normal => {
                html.push_str("<");
                html.push_str(&self.name);
                html.push_str(&Self::class_to_html(&self.classes));
                html.push_str(&Self::attr_to_html(&self.attributes));
                html.push_str(">");

                for child in self.children.iter() {
                    html.push_str(&child.to_html());
                }
                html.push_str(format!("</{}>", self.name).as_str())
            }
            ElementVariant::Void => {
                html.push_str("<");
                html.push_str(&self.name);
                html.push_str(&Self::class_to_html(&self.classes));
                html.push_str(&Self::attr_to_html(&self.attributes));
                html.push_str("/>");
            }
        }

        html
    }

    fn class_to_html(classes: &Vec<String>) -> String {
        let mut html = String::new();

        let classes_len = classes.len();
        for (index, class) in classes.iter().enumerate() {
            if index == 0 {
                html.push_str(&format!(" class=\"{}", class));
            } else {
                html.push_str(&format!(" {}", class));
            }

            if classes_len == index + 1 {
                html.push_str("\"");
            }
        }

        html
    }

    fn attr_to_html(attributes: &Attributes) -> String {
        let mut html = String::new();

        for attr in attributes.iter() {
            if let Some(value) = attr.1.as_ref() {
                html.push_str(&format!(" {}=\"{}\"", attr.0, value));
            } else {
                html.push_str(&format!(" {}", attr.0));
            }
        }

        html
    }
}

impl Default for Element {
    fn default() -> Self {
        Self {
            id: None,
            name: "".to_string(),
            variant: ElementVariant::Void,
            classes: vec![],
            attributes: HashMap::new(),
            children: vec![],
            source_span: SourceSpan::default(),
        }
    }
}

fn ordered_map<S: Serializer>(value: &Attributes, serializer: S) -> Result<S::Ok, S::Error> {
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}
