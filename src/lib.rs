mod ast;
mod error;
mod parser;

use parser::Rule;

pub mod prelude {
    pub use crate::ast::Ast;
    pub use anyhow::Result;
}
