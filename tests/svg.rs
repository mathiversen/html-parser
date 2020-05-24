use html_parser::{Dom, Result};
use indoc::indoc;
use insta::assert_json_snapshot;

#[test]
fn it_can_parse_svg() -> Result<()> {
    let html = indoc!(
        r#"
        <svg  xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
            <rect x="10" y="10" height="100" width="100" style="stroke:#ff0000; fill: #0000ff"/>
        </svg>
    "#
    );
    let dom = Dom::parse(html)?;
    assert_json_snapshot!(dom);
    Ok(())
}

#[test]
fn it_can_parse_complex_svg() {
    let svg = indoc!(
        r#"
        <svg width="600" height="600">
            <rect id="rec" x="300" y="100" width="300" height="100" style="fill:lime"> 
            <animate attributeName="x" attributeType="XML" begin="0s" dur="6s" fill="freeze" from="300" to="0" /> 
            <animate attributeName="y" attributeType="XML" begin="0s" dur="6s" fill="freeze" from="100" to="0" /> 
            <animate attributeName="width" attributeType="XML" begin="0s" dur="6s" fill="freeze" from="300" to="800" /> 
            <animate attributeName="height" attributeType="XML" begin="0s" dur="6s" fill="freeze" from="100" to="300" /> 
            <animate attributeName="fill" attributeType="CSS" from="lime" to="red" begin="2s" dur="4s" fill="freeze" />
            </rect>
            <g transform="translate(100,100)"> 
            <text id="TextElement" x="0" y="0" style="font-family:Verdana;font-size:24; visibility:hidden"> It's SVG!
                <set attributeName="visibility" attributeType="CSS" to="visible" begin="1s" dur="5s" fill="freeze" />
                <animateMotion path="M 0 0 L 100 100" begin="1s" dur="5s" fill="freeze" />
                <animate attributeName="fill" attributeType="CSS" from="red" to="blue" begin="1s" dur="5s" fill="freeze" /> 
                <animateTransform attributeName="transform" attributeType="XML" type="rotate" from="-30" to="0" begin="1s" dur="5s" fill="freeze" /> 
                <animateTransform attributeName="transform" attributeType="XML" type="scale" from="1" to="3" additive="sum" begin="1s" dur="5s" fill="freeze" /> 
            </text> 
            </g>
            Sorry, your browser does not support inline SVG.
        </svg>
    "#
    );
    assert!(Dom::parse(&svg).is_ok());
}
