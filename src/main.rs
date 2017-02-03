mod binary;
mod huffman;
mod commands;

fn main() {
    commands::compress::compress_file_to_file("test1", "test2");
}
