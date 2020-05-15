mod error;
mod parser;

pub mod prelude {
    pub use crate::parser::HtmlParser as Parser;
    pub use anyhow::Result;
}
