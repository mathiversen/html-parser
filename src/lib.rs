//! [![github]](https://github.com/mathiversen/html-parser)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//!
//! # Html parser
//!
//! **WIP - work in progress, use at your own risk**
//!
//! A simple and general purpose html parser, using [Pest](https://pest.rs/).
//!
//! ## What is it not
//!
//! - It's not a high-performance browser-grade parser
//! - It's not 100% complient with html
//! - It's not a parser that includes node selection and dom manipulation
//!
//! If your requirements matches any of the above, then you're most likely looking for crates described below:
//!
//! - [html5ever](https://crates.io/crates/html5ever)
//! - [kuchiki](https://crates.io/crates/kuchiki)
//! - [scraper](https://crates.io/crates/scraper)
//! - or other crates using the `html5ever` parser
//!
//! ## Features
//! - Parse html document
//! - Parse html fragments
//! - Parse custom, non-standard, elements
//! - Doesn't include comments in the AST
//! - Removed dangling elements
//!
//! ## Examples
//! #### Parse html document
//!
//! ```rust
//!     use html_parser::HtmlParser;
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
//!                     </h1> <!-- dangling nodes are removed -->
//!                 </body>
//!             </html>"#;
//!
//!         assert!(HtmlParser::parse(html).is_ok());
//!     }
//! ```
//!
//! #### Parse html fragment
//!
//! ```rust
//!     use html_parser::HtmlParser;
//!
//!     fn main() {
//!         let html = "<div id=cat />";
//!         assert!(HtmlParser::parse(html).is_ok());
//!     }
//! ```
//!
//! #### Print to json
//!
//! ```rust
//!     use html_parser::{HtmlParser, Result};
//!
//!     fn main() -> Result<()> {
//!         let html = "<div id=cat />";
//!         let json = HtmlParser::parse(html)?.to_json_pretty()?;
//!         println!("{}", json);
//!         Ok(())
//!     }
//! ```
//!
//! ## Contributions
//! I would love to get some feedback if you find my little project useful. Please feel free to highlight issues with my code or submit a PR in case you want to improve it.

#![allow(clippy::needless_doctest_main)]

mod ast;
mod error;
mod parser;

use parser::Rule;

pub use crate::ast::element::{Element, ElementVariant};
pub use crate::ast::node::Node;
pub use crate::ast::Ast as HtmlParser;
pub use crate::ast::AstVariant;
pub use anyhow::Result;
