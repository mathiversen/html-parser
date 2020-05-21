use html_parser::{HtmlParser, Result};
use insta::assert_json_snapshot;

#[test]
fn it_can_parse_double_quote() -> Result<()> {
    let markup = "<div id=\"one\"></div>";
    let ast = HtmlParser::parse(markup)?;
    assert_json_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_single_quote() -> Result<()> {
    let markup = "<div id='one'></div>";
    let ast = HtmlParser::parse(markup)?;
    assert_json_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_no_quote() -> Result<()> {
    let markup = "<div id=one></div>";
    let ast = HtmlParser::parse(markup)?;
    assert_json_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_attribute_key_mixed_case_symbols() -> Result<()> {
    let markup = "<div data-cat='morris'></div>";
    let ast = HtmlParser::parse(markup)?;
    assert_json_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_multiple_attributes_single_quote() -> Result<()> {
    let markup = "<div cat='mjau' dog='woff' ape=oh></div>";
    let ast = HtmlParser::parse(markup)?;
    assert_json_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_multiple_attributes_double_quote() -> Result<()> {
    let markup = "<div cat=\"mjau\" dog=\"woff\" ape=\"oh\"></div>";
    let ast = HtmlParser::parse(markup)?;
    assert_json_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_multiple_attributes_no_quote() -> Result<()> {
    let markup = "<div cat=mjau dog=woff ape=oh></div>";
    let ast = HtmlParser::parse(markup)?;
    assert_json_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_attribute_multiple_values_single_quote() -> Result<()> {
    let markup = "<div cat='mjau mjau' />";
    let ast = HtmlParser::parse(markup)?;
    assert_json_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_attribute_multiple_values_double_quote() -> Result<()> {
    let markup = "<div cat=\"mjau mjau\" />";
    let ast = HtmlParser::parse(markup)?;
    assert_json_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_attribute_with_empty_value() -> Result<()> {
    let markup = "<img hidden/>";
    let ast = HtmlParser::parse(markup)?;
    assert_json_snapshot!(ast);
    Ok(())
}

#[test]
fn it_can_parse_id() -> Result<()> {
    let markup = "<img id=a/>";
    let ast = HtmlParser::parse(markup)?;
    assert_json_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_classes() -> Result<()> {
    let markup = "<img class='a b c'/>";
    let ast = HtmlParser::parse(markup)?;
    assert_json_snapshot!(ast);
    Ok(())
}
