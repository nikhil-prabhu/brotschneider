use thiserror::Error;

/// Errors that can occur while reading bits.
#[non_exhaustive]
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
#[non_exhaustive]
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum BitWriterError {
    #[error("attempted to write too many bits ({0})")]
    TooManyBits(u8),
}

/// Errors that can occur while decoding Huffman codes.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum HuffmanError {
    #[error("Too many codes for bit length {0}")]
    OverfullTree(u8),

    #[error("Incomplete Huffman tree")]
    IncompleteTree,

    #[error("Read error: {0}")]
    BitReaderError(#[from] BitReaderError),
}

#[derive(Debug, Error)]
pub enum MetaBlockError {
    #[error("Bit reading error: {0}")]
    BitReader(#[from] BitReaderError),

    #[error("Huffman decoding error: {0}")]
    Huffman(#[from] HuffmanError),

    #[error("Unsupported feature in meta-block")]
    Unsupported,
}
