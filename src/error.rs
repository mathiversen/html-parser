use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("{0}")]
    Parsing(String),
    #[error("{0}")]
    Cli(String),
    #[error("{0}")]
    IO(#[from] std::io::Error),
    #[error("{0}")]
    Serde(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
