use crate::error::MetaBlockError;
use crate::{BitReader, HuffmanTree};

#[allow(dead_code)]
pub struct MetaBlockHeader {
    pub is_last: bool,
    pub length: u32,
    pub is_uncompressed: bool,
}

#[allow(dead_code)]
pub struct MetaBlock {
    pub header: MetaBlockHeader,
    pub literal_huffman: Option<HuffmanTree>,
    // Future: insert/copy, distance trees
    pub data: Vec<u8>,
}

impl MetaBlock {
    /// Decode a single meta-block from the stream.
    pub fn decode(reader: &mut BitReader) -> Result<Self, MetaBlockError> {
        let header = MetaBlock::parse_header(reader)?;

        if header.is_uncompressed {
            todo!("Uncompressed meta-blocks not yet supported");
        }

        // For now, assume a fixed Huffman tree or use a stub tree
        let literal_huffman = Some(HuffmanTree::from_code_lengths(&[2, 2, 2, 2])?);

        let mut data = Vec::new();
        for _ in 0..header.length {
            let symbol = literal_huffman.as_ref().unwrap().decode_symbol(reader)?;
            data.push(symbol as u8);
        }

        Ok(MetaBlock {
            header,
            literal_huffman,
            data,
        })
    }

    /// Parse the header of a meta-block (is_last, length, is_uncompressed).
    fn parse_header(reader: &mut BitReader) -> Result<MetaBlockHeader, MetaBlockError> {
        let is_last = reader.read_bits(1)? != 0;
        let length_nbits = reader.read_bits(2)? + 4;
        let length = reader.read_bits(length_nbits as u8)?;
        let is_uncompressed = reader.read_bits(1)? != 0;

        Ok(MetaBlockHeader {
            is_last,
            length,
            is_uncompressed,
        })
    }
}
