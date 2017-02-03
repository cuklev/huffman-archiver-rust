use std::io::{Read, BufReader, Bytes};

pub struct BinaryReader<T> {
    buffer: u8,
    index: i8,
    reader: Bytes<BufReader<T>>,
}

impl<T: Read> BinaryReader<T> {
    pub fn new(f: T) -> BinaryReader<T> {
        BinaryReader {
            buffer: 0,
            index: 8,
            reader: BufReader::new(f).bytes(),
        }
    }

    pub fn get_bit(&mut self) -> u8 {
        self.index -= 1;
        let bit = (self.buffer >> self.index) & 1;
        if self.index == 0 {
            self.index = 8;
            if let Some(byte) = self.reader.next() {
                self.buffer = byte.unwrap();
            }
        }
        bit
    }

    pub fn get_u8(&mut self) -> u8 {
        let mut x = self.buffer << (8 - self.index);
        if let Some(byte) = self.reader.next() {
            self.buffer = byte.unwrap();
        }
        x | (self.buffer >> self.index)
    }

    pub fn get_u64(&mut self) -> u64 {
        let mut x = 0;
        let mut i = 64 - self.index;
        while i >= 0 {
            x |= (self.buffer as u64) << i;
            if let Some(byte) = self.reader.next() {
                self.buffer = byte.unwrap();
            }
            i -= 8;
        }
        x | ((self.buffer as u64) >> self.index)
    }
}
