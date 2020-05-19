use html_parser::prelude::*;
use indoc::indoc;
use insta::assert_debug_snapshot;

#[test]
fn it_can_parse_empty_document() -> Result<()> {
    let markup = "";
    let ast = Ast::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
