use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/rules.pest"]
pub struct Grammar;
