//! [![github]](https://github.com/mathiversen/html-parser)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//!
//! # The HTML Parser
//!
//! Example:
//!
//! ```
//!     use html_parser::HtmlParser;
//!     
//!     fn main() {
//!         let markup = r"
//!             <html>
//!                 <head></head>
//!                 <body>
//!                     <h1>Hello world</h1>
//!                 </body>
//!             </html>";
//!
//!         assert!(HtmlParser::parse(markup).is_ok())
//!     }
//!```

mod ast;
mod error;
mod parser;

use parser::Rule;

pub use crate::ast::node::*;
pub use crate::ast::Ast as HtmlParser;
pub use crate::ast::AstType;
pub use anyhow::Result;
