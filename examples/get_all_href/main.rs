use html_parser::{Dom, Node, Result};

// This example illustrates how to use the library to get all of the anchor-hrefs from a document.

fn main() -> Result<()> {
    let html = include_str!("./index.html");
    let dom = Dom::parse(html)?;
    let iter = dom.children.get(0).unwrap().into_iter();

    let hrefs = iter.fold(Vec::new(), |mut hrefs, node| match node {
        Node::Element(ref element) => {
            if element.name == "a" {
                hrefs.push(element.attributes["href"].clone().unwrap_or_default());
                hrefs
            } else {
                hrefs
            }
        }
        _ => hrefs,
    });

    println!("\nThe following links where found:");
    for (index, href) in hrefs.iter().enumerate() {
        println!("{}: {}", index + 1, href)
    }

    Ok(())
}
