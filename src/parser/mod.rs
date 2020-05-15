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
        let mut pairs = match PestRules::parse(Rule::root, input) {
            Ok(pairs) => pairs,
            Err(error) => return fmt::error_msg(error),
        };
        if debug {
            let mut element_open_start = 0;
            let mut element_open_end = 0;
            let mut element_void = 0;
            let mut attribute_key = 0;
            let mut attribute_value = 0;
            let mut node_text = 0;

            for (i, pair) in pairs.enumerate() {
                match pair.as_rule() {
                    Rule::element_open_start => {
                        element_open_start = element_open_start + 1;
                        for p in pair.into_inner() {
                            match p.as_rule() {
                                Rule::attribute_key => attribute_key = attribute_key + 1,
                                Rule::attribute_value => attribute_value = attribute_value + 1,
                                _ => continue,
                            }
                        }
                    }
                    Rule::element_open_end => element_open_end = element_open_end + 1,
                    Rule::element_void => element_void = element_void + 1,
                    Rule::node_text => node_text = node_text + 1,
                    _ => continue,
                }
            }

            println!("element_open_start {}", element_open_start);
            println!("element_open_end {}", element_open_end);
            println!("element_void {}", element_void);
            println!("attribute_key {}", attribute_key);
            println!("attribute_value {}", attribute_value);
            println!("node_text {}", node_text);
        }

        Ok(())
    }
}
