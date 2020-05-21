use super::node::Node;
use anyhow::Result;
use serde::{Serialize, Serializer};
use std::collections::{BTreeMap, HashMap};
use std::default::Default;

type AttributesMap = HashMap<String, Option<String>>;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ElementVariant {
    Normal,
    Void,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    pub name: String,
    pub variant: ElementVariant,
    #[serde(serialize_with = "ordered_map")]
    pub attributes: AttributesMap,
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

fn ordered_map<S: Serializer>(value: &AttributesMap, serializer: S) -> Result<S::Ok, S::Error> {
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}
