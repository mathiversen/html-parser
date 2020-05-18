use anyhow::Result;
use pest::{iterators::Pair, iterators::Pairs, Parser};
use pest_derive::Parser;

use crate::error::Error;

mod fmt;
#[cfg(test)]
mod tests;

#[derive(Parser)]
#[grammar = "parser/html.pest"]
pub struct PestRules;

pub struct HtmlParser {}

impl HtmlParser {
    pub fn parse(input: &str, debug: bool) -> Result<()> {
        let pairs = match PestRules::parse(Rule::html, input) {
            Ok(pairs) => pairs,
            Err(error) => return fmt::error_msg(error),
        };
        if debug {
            dbg!(&pairs);
            let mut nodes = Vec::new();
            Self::node_builder(pairs, &mut nodes)?;
            dbg!(nodes);
        }

        Ok(())
    }

    pub fn node_builder(pairs: Pairs<Rule>, collector: &mut Vec<String>) -> Result<()> {
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
                Rule::EOI => (),
                _ => unreachable!("unknown tpl rule: {:?}", pair.as_rule()),
            };
        }
        Ok(())
    }
}
