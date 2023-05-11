use html_parser::{Dom, Result};
use indoc::indoc;
use insta::assert_debug_snapshot;

#[test]
fn it_can_generate_source_span() -> Result<()> {
    let html = indoc! {"
            <template>
                <h1>Header</h1>
                <p>Paragraph</p>
            </template>
        "};
    let dom = Dom::parse(html)?;
    assert_debug_snapshot!(dom);
    Ok(())
}
