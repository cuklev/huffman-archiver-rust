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
            index: 0,
            reader: BufReader::new(f).bytes(),
        }
    }

    pub fn get(&mut self) -> u8 {
        if self.index == 0 {
            self.index = 7;
            if let Some(byte) = self.reader.next() {
                self.buffer = byte.unwrap();
            }
        } else {
            self.index -= 1;
        }

        (self.buffer >> self.index) & 1
    }
}
