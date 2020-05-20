use html_parser::{HtmlParser, Result};
use insta::assert_json_snapshot;

#[test]
fn it_can_parse_document_with_just_one_comment() -> Result<()> {
    let markup = "<!-- hello !\"#/()= -->";
    let ast = HtmlParser::parse(markup)?;
    assert_json_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_document_with_just_comments() -> Result<()> {
    let markup = "<!--x--><!--y--><!--z-->";
    let ast = HtmlParser::parse(markup)?;
    assert_json_snapshot!(ast);
    Ok(())
}
