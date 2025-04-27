use crate::error::BitReaderError;

/// BitReader reads individual bits and bit sequences from a byte array.
#[derive(Clone)]
pub struct BitReader<'a> {
    /// The byte slice to read from.
    data: &'a [u8],
    /// The current position in the byte slice.
    byte_pos: usize,
    /// The current bit position within the current byte.
    bit_pos: u8,
}

impl<'a> BitReader<'a> {
    /// Creates a new BitReader.
    ///
    /// # Arguments
    ///
    /// * `data` - A byte slice to read from.
    ///
    /// # Returns
    ///
    /// A new instance of BitReader.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brotschneider::BitReader;
    ///
    /// let data = [0b11001100, 0b10101010];
    /// let mut reader = BitReader::new(&data);
    /// let bits = reader.read_bits(4).unwrap();
    ///
    /// assert_eq!(bits, 0b1100);
    /// ```
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            byte_pos: 0,
            bit_pos: 0,
        }
    }

    /// Read `n` bits and advance the position.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of bits to read (1-32).
    ///
    /// # Returns
    ///
    /// * `Ok(u32)` - The read bits as a u32.
    /// * `Err(String)` - An error message if the read fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brotschneider::BitReader;
    ///
    /// let data = [0b11001100, 0b10101010];
    /// let mut reader = BitReader::new(&data);
    /// let bits = reader.read_bits(4).unwrap();
    ///
    /// assert_eq!(bits, 0b1100);
    /// ```
    pub fn read_bits(&mut self, n: u8) -> Result<u32, BitReaderError> {
        if n == 0 || n > 32 {
            return Err(BitReaderError::InvalidBitCount(n));
        }

        let mut bits_left = n;
        let mut result = 0u32;

        while bits_left > 0 {
            if self.byte_pos >= self.data.len() {
                return Err(BitReaderError::UnexpectedEndOfInput);
            }

            let current_byte = self.data[self.byte_pos];
            let available_bits = 8 - self.bit_pos;
            let bits_to_take = bits_left.min(available_bits);

            let shift = available_bits - bits_to_take;
            let mask = ((1 << bits_to_take) - 1) as u8;
            let bits = (current_byte >> shift) & mask;

            result = (result << bits_to_take) | (bits as u32);

            self.bit_pos += bits_to_take;
            if self.bit_pos == 8 {
                self.byte_pos += 1;
                self.bit_pos = 0;
            }

            bits_left -= bits_to_take;
        }

        Ok(result)
    }

    /// Peek `n` bits without advancing the position.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of bits to peek (1-32).
    ///
    /// # Returns
    ///
    /// * `Ok(u32)` - The peeked bits as a u32.
    /// * `Err(String)` - An error message if the peek fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brotschneider::BitReader;
    ///
    /// let data = [0b11001100, 0b10101010];
    /// let mut reader = BitReader::new(&data);
    /// let bits = reader.peek_bits(4).unwrap();
    ///
    /// assert_eq!(bits, 0b1100);
    /// ```
    pub fn peek_bits(&self, n: u8) -> Result<u32, BitReaderError> {
        let mut clone = self.clone();
        clone.read_bits(n)
    }

    /// Align to the next byte boundary.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brotschneider::BitReader;
    ///
    /// let data = [0b11001100, 0b10101010];
    /// let mut reader = BitReader::new(&data);
    ///
    /// reader.read_bits(4).unwrap();
    /// reader.align_to_byte();
    ///
    /// assert_eq!(reader.read_bits(4).unwrap(), 0b1010);
    /// ```
    pub fn align_to_byte(&mut self) {
        if self.bit_pos != 0 {
            self.byte_pos += 1;
            self.bit_pos = 0;
        }
    }

    /// Check if all input has been consumed.
    ///
    /// # Returns
    ///
    /// * `true` if all input has been consumed, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brotschneider::BitReader;
    ///
    /// let data = [0b11001100, 0b10101010];
    /// let mut reader = BitReader::new(&data);
    ///
    /// reader.read_bits(16).unwrap();
    /// assert_eq!(reader.is_empty(), true);
    /// ```
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.byte_pos >= self.data.len() && self.bit_pos == 0
    }
}
