use super::*;
use anyhow::Result;
use indoc::indoc;

#[ignore]
#[test]
fn it_works() {
    assert!(true)
}
#[ignore]
#[test]
fn it_can_parse_a_document() {
    let markup = indoc!(
        r#"
        <!-- comment -->
        <!doctype html>
        <!-- comment -->
        <html></html>
        <!-- comment -->
    "#
    );
    let result = HtmlParser::parse(markup);
    assert!(result.is_ok());
}
#[ignore]
#[test]
fn it_can_parse_document_without_doctype() {
    let markup = indoc!(
        r#"
        <!-- comment -->
        <!-- comment -->
        <html></html>
        <!-- comment -->
    "#
    );
    let result = HtmlParser::parse(markup);
    assert!(result.is_ok());
}

#[ignore]
#[test]
fn it_can_parse_a_document_fragment() {
    let markup = indoc!(
        r#"
        <div id="test" class="kalle 123"></div>
        <img width="123" height="321" />
    "#
    );
    let result = HtmlParser::parse(markup);
    dbg!(&result);
    assert!(result.is_ok());
}

#[test]
fn it_can_parse_nested_elements() {
    let markup = indoc!(
        r#"
        <div class="abc" style="abc"><div class="bcd" /></div>
    "#
    );
    let result = HtmlParser::parse(markup);
    dbg!(&result);
    assert!(result.is_ok());
}
