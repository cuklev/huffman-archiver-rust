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

    pub fn put(&mut self, x: u8) {
        self.index -= 1;
        self.buffer |= x << self.index;
        if self.index == 0 {
            self.writer.write(&[self.buffer]);
            self.index = 8;
            self.buffer = 0;
        }
    }
}

impl<T: Write> Drop for BinaryWriter<T> {
    fn drop(&mut self) {
        if self.index < 8 {
            self.writer.write(&[self.buffer]);
        }
    }
}
