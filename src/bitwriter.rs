use crate::error::BitWriterError;

#[allow(dead_code)]
#[derive(Default)]
pub struct BitWriter {
    buffer: Vec<u8>,
    current_byte: u8,
    bit_pos: u8,
}

impl BitWriter {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            current_byte: 0,
            bit_pos: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            current_byte: 9,
            bit_pos: 0,
        }
    }

    pub fn write_bits(&mut self, _value: u32, _num_bits: u8) -> Result<(), BitWriterError> {
        todo!()
    }

    pub fn flush(&mut self) -> Result<(), BitWriterError> {
        todo!()
    }

    pub fn into_inner(mut self) -> Vec<u8> {
        let _ = self.flush();
        self.buffer
    }
}
