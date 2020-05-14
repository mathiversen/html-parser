use anyhow::Result;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[cfg(test)]
mod tests;

#[derive(Parser)]
#[grammar = "parser/rules.pest"]
pub struct PestRules;

pub struct HtmlParser {}

impl HtmlParser {
    pub fn parse(input: &str) -> Result<()> {
        let result = PestRules::parse(Rule::root, input)?;

        dbg!(result);

        Ok(())
    }
}
