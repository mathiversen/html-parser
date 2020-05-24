use html_parser::{Dom, Result};
use indoc::indoc;
use insta::assert_json_snapshot;

#[test]
fn it_can_parse_minimal_document() -> Result<()> {
    let html = "<!DOCTYPE html><html></html>";
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_can_parse_document_with_comments() -> Result<()> {
    let html = indoc!(
        r#"
        <!-- comment -->
        <!-- comment -->
        <!DOCTYPE html>
        <!-- comment -->
        <!-- comment -->
        <html>
        <!-- comment -->
        </html>
        <!-- comment -->
        <!-- comment -->
    "#
    );
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
#[test]
fn it_error_when_doctype_and_multiple_html() {
    let html = "<!DOCTYPE html><html></html><html></html>";
    assert!(Dom::parse(html).is_err());
}
