use html_parser::{Dom, Node, Result};
use ownable::traits::ToBorrowed;
use std::ops::Deref;

// This example illustrates how to use the library to get all of the anchor-hrefs from a document.

fn main() -> Result<()> {
    let html = include_str!("./index.html");
    let dom = Dom::parse(html)?;
    let iter = dom.children.first().unwrap().into_iter();

    let hrefs = iter.filter_map(|item| match item {
        Node::Element(element) if element.name == "a" => element.attributes["href"].to_borrowed(),
        _ => None,
    });

    println!("\nThe following links where found:");
    for (index, href) in hrefs.enumerate() {
        println!("{}: {}", index + 1, href.deref())
    }

    Ok(())
}
