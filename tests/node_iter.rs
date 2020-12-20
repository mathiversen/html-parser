use html_parser::{Dom, Node, Result};
use indoc::indoc;

#[test]
fn it_can_iter_1() -> Result<()> {
    let html = indoc! {"
        <html>
            <head>
                <title>title</title>
            </head>
            <body>
                <ul>
                    <li></li>
                    <li></li>
                    <li></li>
                </ul>
            </body>
        </html>
    "};
    let dom = Dom::parse(&html)?;
    let root = dom.children.get(0).unwrap().into_iter();
    let num_li = root.into_iter().fold(0, |mut acc, curr| match curr {
        Node::Element(ref e) => {
            if e.name == "li" {
                acc += 1;
            }
            acc
        }
        _ => acc,
    });
    assert_eq!(num_li, 3);
    Ok(())
}
