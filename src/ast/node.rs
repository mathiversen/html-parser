use super::element::Element;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Node {
    Text(String),
    Element(Element),
    Comment(String),
}
