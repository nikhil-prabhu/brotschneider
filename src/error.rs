use thiserror::Error;

/// Errors that can occur while reading bits.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum BitReaderError {
    /// Requested an invalid number of bits (must be between 1 and 32).
    #[error("Invalid number of bits requested: {0}")]
    InvalidBitCount(u8),

    /// Attempted to read past the end of the data.
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
}

/// Errors that can occur while writing bits.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum BitWriterError {
    #[error("attempted to write too many bits ({0})")]
    TooManyBits(u8),
}
