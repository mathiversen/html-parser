use super::element::Element;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Node {
    Text(String),
    Element(Element),
    Comment(String),
}
