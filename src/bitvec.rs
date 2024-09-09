use std::fs;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Read;
#[derive(Debug)]
pub struct Write;

#[derive(Debug)]
pub struct BitVec<T> {
    data: Vec<u8>,
    remaining_bits: u8,
    byte_pos: usize,
    _type: PhantomData<T>,
}
impl<T> Default for BitVec<T> {
    fn default() -> Self {
        Self {
            data: vec![0],
            remaining_bits: 8,
            byte_pos: 0,
            _type: PhantomData,
        }
    }
}
impl<T> BitVec<T> {
    pub const fn bits(&self) -> usize {
        self.byte_pos * 8 + (8 - self.remaining_bits) as usize
    }
    pub fn reset(&mut self) {
        self.byte_pos = 0;
        self.remaining_bits = 8;
    }
    pub fn data(self) -> Vec<u8> {
        self.data
    }
}
impl BitVec<Read> {
    pub fn from_file(file: &str) -> Self {
        let data = fs::read(file).unwrap();
        Self {
            data,
            ..Default::default()
        }
    }
    pub fn from_data(data: Vec<u8>) -> Self {
        Self {
            data,
            ..Default::default()
        }
    }
    pub fn read(&mut self, len: u8) -> Option<usize> {
        if self.byte_pos >= self.data.len() {
            return None;
        }
        debug_assert!(len < usize::BITS as u8);

        if self.remaining_bits >= len {
            // The rest of the function only needs the adjusted value
            self.remaining_bits -= len;

            // Align the data to the right
            let aligned_data = self.data[self.byte_pos] >> self.remaining_bits;

            // Construct a bitmask and zero bits outside of len
            let mask = (1 << len) - 1;
            Some(aligned_data as usize & mask)
        } else {
            let raw_byte = self.data[self.byte_pos];

            // Construct a bitmask and zero non-remaining bits
            // Handle edge-case of the full byte being remaining by just returning the byte
            let masked_data = if self.remaining_bits == 8 {
                raw_byte
            } else {
                let mask = (1 << self.remaining_bits) - 1;
                raw_byte & mask
            };

            // The amount of bits to read outside the current byte
            let remaining_len = len - self.remaining_bits;

            // Left shift the data to the correct position
            let positioned_data = (masked_data as usize) << remaining_len;

            self.next_byte();

            // Add (OR) any remaining data recursively
            Some(positioned_data | self.read(remaining_len)?)
        }
    }
    fn next_byte(&mut self) {
        self.remaining_bits = 8;
        self.byte_pos += 1;
    }
}
impl BitVec<Write> {
    #[expect(clippy::cast_possible_truncation)]
    pub fn write(&mut self, data: usize, len: u8) {
        if self.remaining_bits >= len {
            // The rest of the function only needs the adjusted value
            self.remaining_bits -= len;

            // Left shift the data to the correct position
            let positioned_data = data << self.remaining_bits;

            // Add (OR) the positioned data to the current byte
            self.data[self.byte_pos] |= positioned_data as u8;
        } else {
            // The amount of bits to write outside the current byte
            let remaining_len = len - self.remaining_bits;

            // Right shift the data so that it goes into the remaining bits
            let positioned_data = data >> remaining_len;

            // Add (OR) the positioned data to the current byte
            self.data[self.byte_pos] |= positioned_data as u8;

            self.next_byte();

            // Construct a bitmask and zero already written bits
            let remaining_data_mask = (1 << remaining_len) - 1;
            let remaining_data = data & remaining_data_mask;

            // Recursively write the rest of the data
            self.write(remaining_data, remaining_len);
        }
    }
    fn next_byte(&mut self) {
        self.data.push(0);
        self.remaining_bits = 8;
        self.byte_pos += 1;
    }
}
