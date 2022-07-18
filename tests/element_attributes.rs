use html_parser::{Dom, Result};
use insta::assert_json_snapshot;

#[test]
fn it_can_parse_double_quote() -> Result<()> {
    let html = "<div id=\"one\"></div>";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_single_quote() -> Result<()> {
    let html = "<div id='one'></div>";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_no_quote() -> Result<()> {
    let html = "<div id=one></div>";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_attribute_key_mixed_case_symbols() -> Result<()> {
    let html = "<div data-cat='morris'></div>";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_multiple_attributes_single_quote() -> Result<()> {
    let html = "<div cat='mjau' dog='woff' ape=oh></div>";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_multiple_attributes_where_whitespace_does_not_matter_for_keys() -> Result<()> {
    let html = "<div    cat   =  \"mjau\" dog ='  woff  'ape = oh ></div>";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_multiple_attributes_double_quote() -> Result<()> {
    let html = "<div cat=\"mjau\" dog=\"woff\" ape=\"oh\"></div>";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_multiple_attributes_no_quote() -> Result<()> {
    let html = "<div cat=mjau dog=woff ape=oh></div>";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_attribute_multiple_values_single_quote() -> Result<()> {
    let html = "<div cat='mjau mjau' />";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_attribute_multiple_values_double_quote() -> Result<()> {
    let html = "<div cat=\"mjau mjau\" />";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_attribute_with_empty_value() -> Result<()> {
    let html = "<img hidden/>";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}

#[test]
fn it_can_parse_id() -> Result<()> {
    let html = "<img id=a/>";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_classes() -> Result<()> {
    let html = "<img class='a b c'/>";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_keeps_spaces_for_non_classes() -> Result<()> {
    let html = "<img attr=' a b     \n\t'/>";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
