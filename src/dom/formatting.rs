use crate::error::Error;
use crate::Result;
use crate::Rule;
use pest::error::Error as PestError;

/// This function abstracts the formatting of errors away from the core logic inside parser,
/// so that the file is easier to read.
pub fn error_msg(error: PestError<Rule>) -> Result<super::Dom> {
    let message = error.renamed_rules(|rule| match *rule {
        Rule::EOI => "end of input".to_string(),
        Rule::doctype => "doctype element".to_string(),
        Rule::node_text => "text node".to_string(),
        Rule::node_element => "element node".to_string(),
        Rule::el_void => "void element".to_string(),
        Rule::el_void_xml => "void element with xml ending (/>)".to_string(),
        Rule::el_process_instruct => "xml processing instruction".to_string(),
        Rule::el_raw_text => "element with raw text (style or script)".to_string(),
        Rule::el_normal => "normal element".to_string(),
        Rule::el_dangling => "".to_string(),
        Rule::attr => "attribute (key=\"value\")".to_string(),
        Rule::attr_key => "attribute key".to_string(),
        Rule::attr_value => "attribute value".to_string(),
        Rule::el_name => "element name".to_string(),
        Rule::el_void_name_html => "void element name".to_string(),
        // TODO: Continue with this
        x => format!("{:?} ", x),
    });
    Err(Error::Parsing(message.to_string()))
}
