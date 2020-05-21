use super::node::Node;
use anyhow::Result;
use serde::{Serialize, Serializer};
use std::collections::{BTreeMap, HashMap};
use std::default::Default;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementVariant {
    Normal,
    Void,
}

#[derive(Debug, Serialize)]
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
    pub attributes: HashMap<String, Option<String>>,

    /// All of the elements classes
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub classes: Vec<String>,

    /// All of the elements child nodes
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<Node>,
}

impl Default for Element {
    fn default() -> Self {
        Self {
            id: None,
            name: "".to_string(),
            variant: ElementVariant::Void,
            classes: vec![],
            attributes: HashMap::new(),
            nodes: vec![],
        }
    }
}

fn ordered_map<S: Serializer>(
    value: &HashMap<String, Option<String>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}
