use html_parser::{Dom, Result};
use insta::assert_json_snapshot;

#[test]
fn it_can_parse_empty_document() -> Result<()> {
    let markup = "";
    let dom = Dom::parse(markup)?;
    assert_json_snapshot!(dom);
    Ok(())
}
