use crate::error::BitWriterError;

/// BitWriter writes individual bits and bit sequences to a byte array.
#[derive(Default)]
pub struct BitWriter {
    /// The byte buffer to write to.
    buffer: Vec<u8>,
    /// The current byte being written to.
    current_byte: u8,
    /// The current bit position within the current byte.
    bit_pos: u8,
}

impl BitWriter {
    /// Creates a new BitWriter with an empty buffer.
    ///
    /// # Returns
    ///
    /// * A new instance of BitWriter.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brotschneider::BitWriter;
    ///
    /// let writer = BitWriter::new();
    /// assert_eq!(writer.into_inner(), vec![]);
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            current_byte: 0,
            bit_pos: 0,
        }
    }

    /// Creates a new BitWriter with a specified initial capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The initial capacity of the buffer.
    ///
    /// # Returns
    ///
    /// * A new instance of BitWriter with the specified capacity.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brotschneider::BitWriter;
    ///
    /// let writer = BitWriter::with_capacity(10);
    /// assert_eq!(writer.into_inner(), vec![]);
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            current_byte: 9,
            bit_pos: 0,
        }
    }

    /// Writes a specified number of bits to the buffer.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to write.
    /// * `num_bits` - The number of bits to write (must be between 1 and 32).
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the bits were written successfully.
    /// * `Err(BitWriterError)` if the number of bits is invalid or if an error occurs.
    ///
    /// # Panics
    ///
    /// * Panics if `num_bits` is greater than 32.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brotschneider::BitWriter;
    ///
    /// let mut writer = BitWriter::new();
    /// writer.write_bits(0b10101010, 8).unwrap();
    /// let writer_bytes = writer.into_inner();
    /// assert_eq!(writer_bytes, vec![0b10101010]);
    /// ```
    pub fn write_bits(&mut self, value: u32, mut num_bits: u8) -> Result<(), BitWriterError> {
        if num_bits > 32 {
            return Err(BitWriterError::TooManyBits(num_bits));
        }

        while num_bits > 0 {
            let available_bits = 8 - self.bit_pos;
            let bits_to_write = num_bits.min(available_bits);

            let bits = (value >> (num_bits - bits_to_write)) & ((1 << bits_to_write) - 1);

            self.current_byte |= (bits as u8) << (available_bits - bits_to_write);
            self.bit_pos += bits_to_write;
            num_bits -= bits_to_write;

            if self.bit_pos == 8 {
                self.flush()?
            }
        }

        Ok(())
    }

    /// Flushes the current byte to the buffer if there are any bits left.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the flush was successful.
    /// * `Err(BitWriterError)` if an error occurs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brotschneider::BitWriter;
    ///
    /// let mut writer = BitWriter::new();
    /// writer.write_bits(0b10101010, 8).unwrap();
    /// writer.flush().unwrap();
    /// ```
    #[inline]
    pub fn flush(&mut self) -> Result<(), BitWriterError> {
        if self.bit_pos > 0 {
            self.buffer.push(self.current_byte);
            self.current_byte = 0;
            self.bit_pos = 0;
        }

        Ok(())
    }

    /// Returns the current buffer as a byte vector.
    ///
    /// # Returns
    ///
    /// * A vector of bytes representing the written bits.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brotschneider::BitWriter;
    ///
    /// let mut writer = BitWriter::new();
    /// writer.write_bits(0b10101010, 8).unwrap();
    /// let buffer = writer.into_inner();
    /// assert_eq!(buffer, vec![0b10101010]);
    /// ```
    #[inline]
    pub fn into_inner(mut self) -> Vec<u8> {
        let _ = self.flush();
        self.buffer
    }
}
