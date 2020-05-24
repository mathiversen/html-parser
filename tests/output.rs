use html_parser::{Dom, Result};
use indoc::indoc;
use insta::assert_json_snapshot;

#[test]
fn it_can_output_json() -> Result<()> {
    assert!(Dom::parse("<div/>")?.to_json().is_ok());
    Ok(())
}

#[test]
fn it_can_output_json_pretty() -> Result<()> {
    assert!(Dom::parse("<div/>")?.to_json_pretty().is_ok());
    Ok(())
}

#[test]
fn it_can_output_complex_html_as_json() -> Result<()> {
    let html = indoc!(
        "<html lang=\"sv\">
        <head>
            <title>Här kan man va</title>
        </head>
            <body>
                <h1>Tjena världen!</h1>
                <p>Tänkte bara informera om att Sverige är bättre än Finland i ishockey.</p>
            </body>
        </html>"
    );
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}
