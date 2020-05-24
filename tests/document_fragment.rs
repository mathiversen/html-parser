use html_parser::{Dom, Result};
use insta::assert_json_snapshot;

#[test]
fn it_can_parse_single_div_as_fragment() -> Result<()> {
    let html = "<div/>";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_single_text_as_fragment() -> Result<()> {
    let html = "hello";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_text_comment_element_as_fragment() -> Result<()> {
    let html = "hello<!--world?--><div/>";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_error_when_body_is_used_in_fragment_root() {
    let html = "<div></div><body></body>";
    assert!(Dom::parse(html).is_err());
}
#[test]
fn it_error_when_head_is_used_in_fragment_root() {
    let html = "<div></div><head></head>";
    assert!(Dom::parse(html).is_err());
}
#[test]
fn it_error_when_html_is_used_in_fragment_root() {
    let html = "<div></div><html></html>";
    assert!(Dom::parse(html).is_err());
}
