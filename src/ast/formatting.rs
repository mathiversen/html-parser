use crate::error::Error;
use crate::Rule;
use anyhow::Result;
use pest::{error::Error as PestError, RuleType};

/// This function abstracts the formatting of errors away from the core logic inside parser,
/// so that the file is easier to read.
pub fn error_msg(error: PestError<Rule>) -> Result<super::Ast> {
    let message = error.renamed_rules(|rule| match *rule {
        Rule::EOI => "end of input".to_string(),
        // TODO: For a better experience the "x" case should be removed
        // an the match should be exhaustive
        x => format!("{:?} [ Default ]", x),
    });
    Err(Error::Parsing(message.to_string()).into())
}
