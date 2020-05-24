use html_parser::{HtmlParser, Result};
use insta::assert_json_snapshot;

#[test]
fn it_can_parse_single_div_as_fragment() -> Result<()> {
    let markup = "<div/>";
    let dom = HtmlParser::parse(markup)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_single_text_as_fragment() -> Result<()> {
    let markup = "hello";
    let dom = HtmlParser::parse(markup)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_text_comment_element_as_fragment() -> Result<()> {
    let markup = "hello<!--world?--><div/>";
    let dom = HtmlParser::parse(markup)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_error_when_body_is_used_in_fragment_root() {
    let markup = "<div></div><body></body>";
    assert!(HtmlParser::parse(markup).is_err());
}
#[test]
fn it_error_when_head_is_used_in_fragment_root() {
    let markup = "<div></div><head></head>";
    assert!(HtmlParser::parse(markup).is_err());
}
#[test]
fn it_error_when_html_is_used_in_fragment_root() {
    let markup = "<div></div><html></html>";
    assert!(HtmlParser::parse(markup).is_err());
}
