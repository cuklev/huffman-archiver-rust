mod binary;
mod huffman;
use huffman::huffman::build_huffman_tree;

fn main() {
    let f = [0; 256];
    build_huffman_tree(f);
}
