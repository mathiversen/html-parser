use html_parser::{HtmlParser, Result};
use insta::assert_debug_snapshot;

#[test]
fn it_can_parse_empty_document() -> Result<()> {
    let markup = "";
    let ast = HtmlParser::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
