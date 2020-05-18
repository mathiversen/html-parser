use crate::parser::HtmlParser;
use anyhow::Result;

#[test]
fn it_can_parse_double_quote() -> Result<()> {
    assert_eq!((), HtmlParser::parse("<div id=\"one\"></div>", false)?);
    Ok(())
}
#[test]
fn it_can_parse_single_quote() -> Result<()> {
    assert_eq!((), HtmlParser::parse("<div id='one'></div>", false)?);
    Ok(())
}
#[test]
fn it_can_parse_no_quote_no_space() -> Result<()> {
    assert_eq!((), HtmlParser::parse("<div id=one></div>", false)?);
    Ok(())
}
#[test]
fn it_can_parse_no_quote_space() -> Result<()> {
    assert_eq!((), HtmlParser::parse("<div id=one ></div>", false)?);
    Ok(())
}
#[test]
fn it_can_parse_closed_element_attribute_double_quote() -> Result<()> {
    assert_eq!((), HtmlParser::parse("<img alt=\"cat\" />", false)?);
    Ok(())
}
#[test]
fn it_can_parse_closed_element_attribute_single_quote() -> Result<()> {
    assert_eq!((), HtmlParser::parse("<img alt='cat'/>", false)?);
    Ok(())
}
#[test]
fn it_can_parse_attribute_key_mixed_case_symbols() -> Result<()> {
    assert_eq!((), HtmlParser::parse("<img data-cat='morris'/>", false)?);
    Ok(())
}
#[test]
fn it_can_parse_multiple_attributes_single_quote() -> Result<()> {
    assert_eq!(
        (),
        HtmlParser::parse("<img alt='cat' title='morris'/>", false)?
    );
    Ok(())
}
#[test]
fn it_can_parse_multiple_attributes_single_quote_multiple_spaces() -> Result<()> {
    assert_eq!(
        (),
        HtmlParser::parse("<img alt='cat'   title='morris'  />", false)?
    );
    Ok(())
}
#[test]
fn it_can_parse_multiple_attributes_double_quote() -> Result<()> {
    assert_eq!(
        (),
        HtmlParser::parse("<img alt=\"cat\" title=\"morris\"/>", false)?
    );
    Ok(())
}

#[test]
fn it_can_parse_multiple_attribute_values_single_quote() -> Result<()> {
    assert_eq!((), HtmlParser::parse("<img alt='cat dog'/>", false)?);
    Ok(())
}
#[test]
fn it_can_parse_multiple_attribute_values_double_quote() -> Result<()> {
    assert_eq!((), HtmlParser::parse("<img alt=\"cat dog\"/>", false)?);
    Ok(())
}
#[test]
fn it_can_parse_empty_attributes() -> Result<()> {
    assert_eq!((), HtmlParser::parse("<img hidden/>", false)?);
    Ok(())
}
