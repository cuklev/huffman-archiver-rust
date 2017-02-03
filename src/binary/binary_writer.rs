use std::io::{Write, BufWriter};

pub struct BinaryWriter<T: Write> {
    buffer: u8,
    index: i8,
    writer: BufWriter<T>,
}

impl<T: Write> BinaryWriter<T> {
    pub fn new(f: T) -> BinaryWriter<T> {
        BinaryWriter {
            buffer: 0,
            index: 8,
            writer: BufWriter::new(f),
        }
    }

    pub fn put_bit(&mut self, x: u8) {
        self.index -= 1;
        self.buffer |= x << self.index;
        if self.index == 0 {
            self.writer.write(&[self.buffer]);
            self.index = 8;
            self.buffer = 0;
        }
    }

    pub fn put_u8(&mut self, x: u8) {
        self.writer.write(&[self.buffer | (x >> (8 - self.index))]);
        self.buffer = x.wrapping_shl(self.index as u32);
    }

    pub fn put_u64(&mut self, x: u64) {
        self.writer.write(&[self.buffer | (x >> (64 - self.index)) as u8]);
        let mut i = 56 - self.index;
        while i >= 0 {
            self.writer.write(&[(x >> i) as u8]);
            i -= 8;
        }
        self.buffer = (x << self.index) as u8;
    }
}

impl<T: Write> Drop for BinaryWriter<T> {
    fn drop(&mut self) {
        if self.index < 8 {
            self.writer.write(&[self.buffer]);
        }
    }
}
