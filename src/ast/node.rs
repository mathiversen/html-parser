use serde::Serialize;
use std::collections::HashMap;
use std::default::Default;

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Node {
    Text(String),
    Element(Element),
    Comment(String),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ElementVariant {
    Normal,
    Void,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Element {
    pub name: String,
    pub variant: ElementVariant,
    pub attributes: HashMap<String, Option<String>>,
    pub nodes: Vec<Node>,
}

impl Default for Element {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            variant: ElementVariant::Void,
            attributes: HashMap::new(),
            nodes: vec![],
        }
    }
}
