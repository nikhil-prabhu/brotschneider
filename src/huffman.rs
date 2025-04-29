use crate::BitReader;
use crate::error::HuffmanError;

#[allow(dead_code)]
pub struct HuffmanTree {
    /// Map from Huffman code to symbol.
    pub lookup: Vec<u16>,
    /// Number of bits in the longest code.
    pub max_bits: u8,
}

impl HuffmanTree {
    /// Construct a canonical Huffman tree from code lengths.
    /// Each entry in `code_lengths` is the code length (in bits) for the symbol with that index.
    ///
    /// # Arguments
    ///
    /// * `code_lengths` - A slice of u8 representing the code lengths for each symbol.
    ///
    /// # Returns
    ///
    /// * A Result containing the constructed HuffmanTree or an error if the tree cannot be constructed.
    ///
    /// # Errors
    ///
    /// * Returns `HuffmanError::OverfullTree` if the tree is overfull.
    /// * Returns `HuffmanError::IncompleteTree` if the tree is incomplete.
    /// * Returns `HuffmanError::BitReaderError` if there is an error reading bits.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brotschneider::HuffmanTree;
    ///
    /// let code_lengths = [2, 2, 2, 2];
    /// let tree = HuffmanTree::from_code_lengths(&code_lengths).unwrap();
    ///
    /// assert_eq!(tree.max_bits, 2);
    /// assert_eq!(tree.lookup.len(), 4);
    /// ```
    pub fn from_code_lengths(code_lengths: &[u8]) -> Result<Self, HuffmanError> {
        let mut max_bits = 0u8;
        for &len in code_lengths {
            if len > max_bits {
                max_bits = len;
            }
        }

        if max_bits == 0 {
            return Err(HuffmanError::IncompleteTree);
        }

        let mut bl_count = vec![0u16; (max_bits + 1) as usize];
        for &len in code_lengths {
            if len != 0 {
                bl_count[len as usize] += 1;
            }
        }

        let mut code = 0u32;
        let mut next_code = vec![0u32; (max_bits + 1) as usize];

        for bits in 1..=max_bits {
            code = (code + bl_count[(bits - 1) as usize] as u32) << 1;
            next_code[bits as usize] = code;
        }

        let mut total_codes = 0u32;

        for bits in 1..=max_bits {
            total_codes = (total_codes << 1) + bl_count[bits as usize] as u32;
        }

        if total_codes != (1u32 << max_bits) {
            if total_codes > (1u32 << max_bits) {
                return Err(HuffmanError::OverfullTree(max_bits));
            }

            return Err(HuffmanError::IncompleteTree);
        }

        let table_size = 1 << max_bits;
        let mut lookup = vec![0xffffu16; table_size];

        for (symbol, &len) in code_lengths.iter().enumerate() {
            if len != 0 {
                let code_val = next_code[len as usize];
                next_code[len as usize] += 1;

                let prefix = code_val << (max_bits - len);
                let fill_count = 1 << (max_bits - len);

                for i in 0..fill_count {
                    let idx = (prefix | i) as usize;
                    lookup[idx] = symbol as u16;
                }
            }
        }

        Ok(HuffmanTree { lookup, max_bits })
    }

    /// Decode a symbol from the bitstream using the lookup table.
    ///
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a `BitReader` instance.
    ///
    /// # Returns
    ///
    /// * A Result containing the decoded symbol (u16) or an error if decoding fails.
    ///
    /// # Errors
    ///
    /// * Returns `HuffmanError::IncompleteTree` if the tree is incomplete.
    /// * Returns `HuffmanError::BitReaderError` if there is an error reading bits.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brotschneider::{BitReader, HuffmanTree};
    ///
    /// let code_lengths = [2, 2, 2, 2]; // Complete tree
    /// let tree = HuffmanTree::from_code_lengths(&code_lengths).unwrap();
    ///
    /// let mut reader = BitReader::new(&[0b00000000]);
    /// let symbol = tree.decode_symbol(&mut reader).unwrap();
    /// assert_eq!(symbol, 0);
    /// ```
    pub fn decode_symbol(&self, reader: &mut BitReader) -> Result<u16, HuffmanError> {
        let bits = reader.peek_bits(self.max_bits)? as usize;
        let symbol = self.lookup[bits];

        if symbol == 0xffff {
            return Err(HuffmanError::IncompleteTree);
        }

        let mut code_len = 1;
        while code_len <= self.max_bits {
            let idx = (bits >> (self.max_bits - code_len)) << (self.max_bits - code_len);
            let fill_count = 1 << (self.max_bits - code_len);
            let mut match_all = true;

            for i in 0..fill_count {
                if self.lookup[idx | i] != symbol {
                    match_all = false;
                    break;
                }
            }

            if match_all {
                reader.skip_bits(code_len as usize)?;
                return Ok(symbol);
            }

            code_len += 1;
        }

        Err(HuffmanError::IncompleteTree)
    }
}
