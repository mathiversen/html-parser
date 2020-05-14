use super::*;
use anyhow::Result;
use indoc::indoc;

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

#[test]
fn it_can_parse_a_document_fragment() {
    let markup = indoc!(
        r#"
        <rlx:div test="abc def" hejsan="abc">
            <div>hej</div>
        </rlx:div>
        <img test="hejsan" />
    "#
    );
    let result = HtmlParser::parse(markup);
    dbg!(&result);
    assert!(result.is_ok());
}
