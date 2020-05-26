use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("{0}")]
    Parsing(String),
    #[error("{0}")]
    Cli(String),
}
