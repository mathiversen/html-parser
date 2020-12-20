use html_parser::{Dom, Node, Result};

// This example illustrates how to use the library to get all of the anchor-links from within a document. Please see the index.html file for the html that gets parsed in this example.

fn main() -> Result<()> {
    let html = include_str!("./index.html");
    let dom = Dom::parse(html)?;
    let iter = dom.children.get(0).unwrap().into_iter();

    let links = iter.fold(Vec::new(), |mut acc, node| match node {
        Node::Element(ref element) => {
            if element.name == "a" {
                acc.push(element.attributes["href"].clone().unwrap_or_default());
                acc
            } else {
                acc
            }
        }
        _ => acc,
    });

    println!("\nThe following links where found:");
    for (index, link) in links.iter().enumerate() {
        println!("{}: {}", index + 1, link)
    }

    Ok(())
}
