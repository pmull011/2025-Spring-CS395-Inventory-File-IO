use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub struct ErrorWithValue<E: std::error::Error, V> {
    #[source]
    pub the_error: E,
    pub the_value: V,
}

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("{0:?}")]
    FileNotFound(#[from] std::io::Error),
}
