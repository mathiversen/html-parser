use super::*;
use anyhow::Result;

#[test]
fn it_works() {
    assert!(true)
}

#[test]
fn it_can_parse() -> Result<()> {
    let result = HtmlParser::parse("hello world")?;
    dbg!(result);
    assert!(false);
    Ok(())
}
