use anyhow::Result;
use pest::{iterators::Pair, iterators::Pairs, Parser as PestParser};

use crate::error::Error;
use crate::parser::Parser;
use crate::Rule;

mod format_error_msg;
use format_error_msg::format_error_msg;

#[derive(Debug)]
pub enum AstType {
    Document,
    DocumentFragment,
}

#[derive(Debug)]
pub struct Ast {
    pub ast_type: AstType,
    pub nodes: Vec<String>,
}

impl Ast {
    pub fn parse(input: &str, debug: bool) -> Result<Self> {
        let pairs = match Parser::parse(Rule::html, input) {
            Ok(pairs) => pairs,
            Err(error) => return format_error_msg(error),
        };
        if debug {
            dbg!(&pairs);
        }

        let mut ast = Self {
            ast_type: AstType::Document,
            nodes: Vec::new(),
        };

        Self::node_builder(pairs, &mut ast.nodes)?;

        Ok(ast)
    }

    fn node_builder(pairs: Pairs<Rule>, collector: &mut Vec<String>) -> Result<()> {
        for pair in pairs {
            match pair.as_rule() {
                Rule::node_element => {
                    Self::node_builder(pair.into_inner(), collector)?;
                }
                Rule::node_text => collector.push("text".to_string()),
                Rule::node_comment => collector.push("comment".to_string()),
                Rule::el_name | Rule::el_void_name => {
                    collector.push(pair.as_str().to_string());
                }
                Rule::attr => {
                    Self::node_builder(pair.into_inner(), collector)?;
                }
                Rule::attr_key | Rule::attr_value => {
                    collector.push(format!("{}", pair.as_str().to_string()))
                }
                Rule::comment_tag_start => (),
                Rule::comment_tag_end => (),
                Rule::el_normal_end => collector.push(format!("{}", pair.as_str().to_string())),
                Rule::el_dangling => (),
                Rule::EOI => (),
                _ => unreachable!("unknown tpl rule: {:?}", pair.as_rule()),
            };
        }
        Ok(())
    }
}
