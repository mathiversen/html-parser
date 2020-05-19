use html_parser::prelude::*;
use insta::assert_debug_snapshot;

#[test]
fn it_can_parse_double_quote() -> Result<()> {
    let markup = "<div id=\"one\"></div>";
    let ast = Ast::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_single_quote() -> Result<()> {
    let markup = "<div id='one'></div>";
    let ast = Ast::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_no_quote() -> Result<()> {
    let markup = "<div id=one></div>";
    let ast = Ast::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_attribute_key_mixed_case_symbols() -> Result<()> {
    let markup = "<div data-cat='morris'></div>";
    let ast = Ast::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
#[ignore]
#[test]
fn it_can_parse_multiple_attributes_single_quote() -> Result<()> {
    let markup = "<div cat='mjau' dog='woff'></div>";
    let ast = Ast::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
#[ignore]
#[test]
fn it_can_parse_multiple_attributes_double_quote() -> Result<()> {
    let markup = "<div cat=\"mjau\" dog=\"woff\"></div>";
    let ast = Ast::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
#[ignore]
#[test]
fn it_can_parse_multiple_attributes_no_quote() -> Result<()> {
    let markup = "<div cat=mjau dog=woff></div>";
    let ast = Ast::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_attribute_multiple_values_single_quote() -> Result<()> {
    let markup = "<div cat='mjau mjau' />";
    let ast = Ast::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_attribute_multiple_values_double_quote() -> Result<()> {
    let markup = "<div cat=\"mjau mjau\" />";
    let ast = Ast::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_attribute_with_empty_value() -> Result<()> {
    let markup = "<img hidden/>";
    let ast = Ast::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
