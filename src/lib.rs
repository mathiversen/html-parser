//! [![github]](https://github.com/mathiversen/html-parser)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//!
//! # Html parser
//!
//! A simple and general purpose html/xhtml parser lib/bin, using [Pest](https://pest.rs/).
//!
//! ## Features
//! - Parse html & xhtml (not xml processing instructions)
//! - Parse html-documents
//! - Parse html-fragments
//! - Parse empty documents
//! - Parse with the same api for both documents and fragments
//! - Parse custom, non-standard, elements; `<cat/>`, `<Cat/>` and `<C4-t/>`
//! - Removes comments
//! - Removes dangling elements
//! - Iterate over all nodes in the dom tree
//!
//! ## What is it not
//!
//! - It's not a high-performance browser-grade parser
//! - It's not suitable for html validation
//! - It's not a parser that includes element selection or dom manipulation
//!
//! If your requirements matches any of the above, then you're most likely looking for one of the crates below:
//!
//! - [html5ever](https://crates.io/crates/html5ever)
//! - [kuchiki](https://crates.io/crates/kuchiki)
//! - [scraper](https://crates.io/crates/scraper)
//! - or other crates using the `html5ever` parser
//!
//! ## Examples bin
//!
//! Parse html file
//!
//! ```shell
//! html_parser index.html
//!
//! ```
//!
//! Parse stdin with pretty output
//!
//! ```shell
//! curl <website> | html_parser -p
//! ```
//!
//! ## Examples lib
//!
//! Parse html document
//!
//! ```rust
//!     use html_parser::Dom;
//!
//!     fn main() {
//!         let html = r#"
//!             <!doctype html>
//!             <html lang="en">
//!                 <head>
//!                     <meta charset="utf-8">
//!                     <title>Html parser</title>
//!                 </head>
//!                 <body>
//!                     <h1 id="a" class="b c">Hello world</h1>
//!                     </h1> <!-- comments & dangling elements are ignored -->
//!                 </body>
//!             </html>"#;
//!
//!         assert!(Dom::parse(html).is_ok());
//!     }
//! ```
//!
//! Parse html fragment
//!
//! ```rust
//!     use html_parser::Dom;
//!
//!     fn main() {
//!         let html = "<div id=cat />";
//!         assert!(Dom::parse(html).is_ok());
//!     }
//! ```
//!
//! Print to json
//!
//! ```rust
//!     use html_parser::{Dom, Result};
//!
//!     fn main() -> Result<()> {
//!         let html = "<div id=cat />";
//!         let json = Dom::parse(html)?.to_json_pretty()?;
//!         println!("{}", json);
//!         Ok(())
//!     }
//! ```

#![allow(clippy::needless_doctest_main)]

mod dom;
mod error;
mod grammar;

use grammar::Rule;

pub use crate::dom::element::{Element, ElementVariant};
pub use crate::dom::node::Node;
pub use crate::dom::Dom;
pub use crate::dom::DomVariant;
pub use crate::error::Error;
pub use crate::error::Result;
