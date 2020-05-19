use anyhow::Result;
use pest::{iterators::Pair, iterators::Pairs, Parser as PestParser};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::error::Error;
use crate::parser::Parser;
use crate::Rule;

mod formatting;
mod node;
use node::{Element, ElementVariant, Node};

// TODO: Parse doctype attribute
#[derive(Debug)]
pub enum AstType {
    Document,
    DocumentFragment,
}

#[derive(Debug)]
pub struct Ast {
    pub time_parsing: u128,
    pub tree_type: Option<AstType>,
    pub nodes: Vec<Node>,
}

impl Ast {
    pub fn parse(input: &str, debug: bool) -> Result<Self> {
        let now = Instant::now();
        let pairs = match Parser::parse(Rule::html, input) {
            Ok(pairs) => pairs,
            Err(error) => return formatting::error_msg(error),
        };

        let mut ast = Self {
            time_parsing: now.elapsed().as_millis(),
            tree_type: None,
            nodes: vec![],
        };

        Self::build_root(&mut ast, pairs)?;

        if debug {
            dbg!(&ast);
        }

        Ok(ast)
    }

    fn build_root(self: &mut Self, pairs: Pairs<Rule>) -> Result<()> {
        for pair in pairs {
            match pair.as_rule() {
                Rule::document => {
                    self.tree_type = Some(AstType::Document);
                    self.nodes
                        .extend(Self::build_document(pair.into_inner(), true)?);
                }
                Rule::document_fragment => {
                    self.tree_type = Some(AstType::DocumentFragment);
                    self.nodes
                        .extend(Self::build_document(pair.into_inner(), false)?);
                }
                Rule::EOI => (),
                _ => unreachable!("[build root] unknown rule: {:?}", pair.as_rule()),
            };
        }
        Ok(())
    }

    fn build_document(pairs: Pairs<Rule>) -> Result<Vec<Node>> {
        let mut nodes = vec![];
        for pair in pairs {
            match pair.as_rule() {
                Rule::doctype => (),
                Rule::node_element => {
                    let node = Self::build_node_element(pair.into_inner())?;
                    nodes.push(node);
                }
                _ => unreachable!("[build document] unknown rule: {:?}", pair.as_rule()),
            };
        }
        Ok(nodes)
    }

    fn build_node_element(pairs: Pairs<Rule>) -> Result<Node> {
        let mut element = Element::default();
        for pair in pairs {
            match pair.as_rule() {
                Rule::node_element | Rule::el_raw_text => {
                    let child_element = Self::build_node_element(pair.into_inner())?;
                    element.nodes.push(child_element)
                }
                Rule::node_text => {
                    element.nodes.push(Node::Text(pair.as_str().to_string()));
                }
                Rule::el_name | Rule::el_void_name | Rule::el_raw_text_name => {
                    element.name = pair.as_str().to_string();
                }
                Rule::attr => {
                    let new_attribute = Self::build_attribute(pair.into_inner())?;
                    element.attributes.insert(new_attribute.0, new_attribute.1);
                }
                Rule::el_normal_end => break,
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
                Rule::attr_value => {
                    attribute.1 = Some(pair.as_str().to_string());
                }
                _ => unreachable!("unknown tpl rule: {:?}", pair.as_rule()),
            }
        }
        assert_ne!(attribute.0, "");
        Ok(attribute)
    }
}
