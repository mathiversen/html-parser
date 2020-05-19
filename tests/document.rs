use html_parser::{HtmlParser, Result};
use indoc::indoc;
use insta::assert_debug_snapshot;

#[test]
fn it_can_parse_minimal_document() -> Result<()> {
    let markup = "<!DOCTYPE html><html></html>";
    let ast = HtmlParser::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_document_with_comments() -> Result<()> {
    let markup = indoc!(
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
    let ast = HtmlParser::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
#[test]
fn it_error_when_doctype_and_multiple_html() {
    let markup = "<!DOCTYPE html><html></html><html></html>";
    assert!(HtmlParser::parse(markup).is_err());
}
