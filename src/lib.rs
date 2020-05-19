mod ast;
mod error;
mod parser;

use parser::Rule;

pub use crate::ast::node::*;
pub use crate::ast::Ast as HtmlParser;
pub use crate::ast::AstType;
pub use anyhow::Result;
