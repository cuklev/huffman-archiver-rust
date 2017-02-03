use std::io::{BufReader, BufWriter, Write, Read, Bytes};
use std::fs::File;

use huffman::compress::{compress, read_freq_table};
use binary::binary_writer::BinaryWriter;

pub fn compress_file_to_file(input_file: &str, output_file: &str) {
    if let Ok(f) = File::create(output_file) {
        let mut output_stream = BufWriter::new(f);
        compress_file_to_stream(input_file, &mut output_stream);
    }
}

pub fn compress_file_to_stream<W: Write>(input_file: &str, output_stream: &mut W) {
    let freq_table = {
        if let Ok(f) = File::open(input_file) {
            let mut input_stream = BufReader::new(f).bytes();
            read_freq_table(&mut input_stream)
        } else {
            [0; 256]
        }
    };

    if let Ok(f) = File::open(input_file) {
        let mut input_stream = BufReader::new(f).bytes();
        let mut binary_out = BinaryWriter::new(output_stream);
        compress(freq_table, &mut input_stream, &mut binary_out);
    }
}
