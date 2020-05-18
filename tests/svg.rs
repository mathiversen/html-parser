use html_parser::prelude::*;
use indoc::indoc;

#[test]
fn it_can_parse_svg() -> Result<()> {
    let svg = indoc!(
        r#"
        <svg  xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
            <rect x="10" y="10" height="100" width="100" style="stroke:#ff0000; fill: #0000ff"/>
        </svg>
    "#
    );
    assert_eq!((), Ast::parse(&svg, false)?);
    Ok(())
}
