use crate::BitReader;
use crate::error::HuffmanError;

#[allow(dead_code)]
pub struct HuffmanTree {
    /// Map from Huffman code to symbol.
    lookup: Vec<u16>,
    /// Number of bits in the longest code.
    max_bits: u8,
}

impl HuffmanTree {
    pub fn from_code_lengths(_code_lengths: &[u8]) -> Result<Self, HuffmanError> {
        todo!()
    }

    pub fn decode_symbol(&self, _reader: &mut BitReader) -> Result<u16, HuffmanError> {
        todo!()
    }
}
