use html_parser::prelude::*;
use indoc::indoc;
use insta::assert_debug_snapshot;

#[test]
fn it_can_parse_document_with_just_one_comment() -> Result<()> {
    let markup = "<!-- hello !\"#/()= -->";
    let ast = Ast::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_document_with_just_comments() -> Result<()> {
    let markup = "<!--x--><!--y--><!--z-->";
    let ast = Ast::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
