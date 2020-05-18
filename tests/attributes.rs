use html_parser::prelude::*;

#[test]
fn it_can_parse_double_quote() {
    let markup = "<div id=\"one\"></div>";
    assert!(Ast::parse(markup, false).is_ok());
}
#[test]
fn it_can_parse_single_quote() {
    let markup = "<div id='one'></div>";
    assert!(Ast::parse(markup, false).is_ok());
}
#[test]
fn it_can_parse_no_quote_no_space() {
    let markup = "<div id=one></div>";
    assert!(Ast::parse(markup, false).is_ok());
}
#[test]
fn it_can_parse_no_quote_space() {
    let markup = "<div id=one ></div>";
    assert!(Ast::parse(markup, false).is_ok());
}
#[test]
fn it_can_parse_closed_element_attribute_double_quote() {
    let markup = "<img alt=\"cat\" />";
    assert!(Ast::parse(markup, false).is_ok());
}
#[test]
fn it_can_parse_closed_element_attribute_single_quote() {
    let markup = "<img alt='cat'/>";
    assert!(Ast::parse(markup, false).is_ok());
}
#[test]
fn it_can_parse_attribute_key_mixed_case_symbols() {
    let markup = "<img data-cat='morris'/>";
    assert!(Ast::parse(markup, false).is_ok());
}
#[test]
fn it_can_parse_multiple_attributes_single_quote() {
    let markup = "<img alt='cat' title='morris'/>";
    assert!(Ast::parse(markup, false).is_ok());
}
#[test]
fn it_can_parse_multiple_attributes_single_quote_multiple_spaces() {
    let markup = "<img alt='cat'   title='morris'  />";
    assert!(Ast::parse(markup, false).is_ok());
}
#[test]
fn it_can_parse_multiple_attributes_double_quote() {
    let markup = "<img alt=\"cat\" title=\"morris\"/>";
    assert!(Ast::parse(markup, false).is_ok());
}

#[test]
fn it_can_parse_multiple_attribute_values_single_quote() {
    let markup = "<img alt='cat dog'/>";
    assert!(Ast::parse(markup, false).is_ok());
}
#[test]
fn it_can_parse_multiple_attribute_values_double_quote() {
    let markup = "<img alt=\"cat dog\"/>";
    assert!(Ast::parse(markup, false).is_ok());
}
#[test]
fn it_can_parse_empty_attributes() {
    let markup = "<img hidden/>";
    assert!(Ast::parse(markup, false).is_ok());
}
