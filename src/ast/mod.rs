use anyhow::Result;
use pest::{iterators::Pairs, Parser as PestParser};
use serde::Serialize;
use std::default::Default;

use crate::error::Error;
use crate::parser::Parser;
use crate::Rule;

mod formatting;
pub mod node;
use node::{Element, ElementVariant, Node};

// TODO: Parse doctype attribute
#[derive(Debug, PartialEq, Serialize)]
pub enum AstType {
    Document,
    DocumentFragment,
    Empty,
}

#[derive(Debug, Serialize)]
pub struct Ast {
    pub tree_type: AstType,
    pub nodes: Vec<Node>,
}

impl Default for Ast {
    fn default() -> Self {
        Self {
            tree_type: AstType::Empty,
            nodes: vec![],
        }
    }
}

impl Ast {
    pub fn parse(input: &str) -> Result<Self> {
        let pairs = match Parser::parse(Rule::html, input) {
            Ok(pairs) => pairs,
            Err(error) => return formatting::error_msg(error),
        };
        Self::build_ast(pairs)
    }

    fn build_ast(pairs: Pairs<Rule>) -> Result<Self> {
        let mut ast = Self::default();
        for pair in pairs {
            match pair.as_rule() {
                Rule::doctype => {
                    ast.tree_type = AstType::DocumentFragment;
                }
                Rule::node_element => {
                    let node = Self::build_node_element(pair.into_inner())?;
                    ast.nodes.push(node);
                }
                Rule::node_text => {
                    ast.nodes.push(Node::Text(pair.as_str().to_string()));
                }
                Rule::EOI => break,
                _ => unreachable!("[build ast] unknown rule: {:?}", pair.as_rule()),
            };
        }

        // TODO: This needs to be cleaned up / what logic should apply when parsing fragment vs document?
        match ast.nodes.len() {
            0 => {
                ast.tree_type = AstType::Empty;
                Ok(ast)
            }
            1 => match ast.nodes[0] {
                Node::Element(ref el) => {
                    let name = el.name.to_lowercase();
                    if name == "html" {
                        ast.tree_type = AstType::Document;
                        Ok(ast)
                    } else if ast.tree_type == AstType::Document && name != "html" {
                        Err(
                            Error::Parsing("A document can only have html as root".to_string())
                                .into(),
                        )
                    } else {
                        ast.tree_type = AstType::DocumentFragment;
                        Ok(ast)
                    }
                }
                _ => {
                    ast.tree_type = AstType::DocumentFragment;
                    Ok(ast)
                }
            },
            _ => {
                ast.tree_type = AstType::DocumentFragment;
                for node in &ast.nodes {
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
                Ok(ast)
            }
        }
    }

    fn build_node_element(pairs: Pairs<Rule>) -> Result<Node> {
        let mut element = Element::default();
        for pair in pairs {
            match pair.as_rule() {
                Rule::node_element | Rule::el_raw_text => {
                    let child_element = Self::build_node_element(pair.into_inner())?;
                    element.nodes.push(child_element)
                }
                Rule::node_text | Rule::el_raw_text_content => {
                    element.nodes.push(Node::Text(pair.as_str().to_string()));
                }
                Rule::el_name | Rule::el_void_name | Rule::el_raw_text_name => {
                    element.name = pair.as_str().to_string();
                }
                Rule::attr => {
                    let new_attribute = Self::build_attribute(pair.into_inner())?;
                    element.attributes.insert(new_attribute.0, new_attribute.1);
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
        Ok(Node::Element(element))
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
