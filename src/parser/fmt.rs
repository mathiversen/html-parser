use super::Rule;
use crate::error::Error;
use anyhow::Result;
use pest::{error::Error as PestError, RuleType};

/// This function abstracts the formatting of errors away from the core logic inside parser,
/// so that the file is easier to read.
pub fn error_msg(error: PestError<Rule>) -> Result<()> {
    let message = error.renamed_rules(|rule| match *rule {
        Rule::EOI => "end of input".to_string(),
        // TODO: For a better experience the "x" case should be removed
        // an the match should be exhaustive
        x => format!("{:?} [ Default error ]", x),
    });
    return Err(Error::Parsing(message.to_string()).into());
}
