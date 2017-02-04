use std::io::{Read, Write};
use std::ops::Deref;
use binary::binary_reader::BinaryReader;
use huffman::huffman::HuffmanNode;

pub fn dfs<R: Read>(binary_in: &mut BinaryReader<R>) -> HuffmanNode {
    if binary_in.get_bit() == 1 {
        HuffmanNode::Leaf(binary_in.get_u8())
    } else {
        HuffmanNode::Node {
left: Box::new(dfs(binary_in)),
          right: Box::new(dfs(binary_in)),
        }
    }
}

pub fn decompress<R: Read, W: Write>(binary_in: &mut BinaryReader<R>, output_stream: &mut W) {
    let mut bytes_left = binary_in.get_u64();
    if bytes_left == 0 { return; }

    let root = dfs(binary_in);

    if let HuffmanNode::Leaf(x) = root {
        while bytes_left > 0 {
            output_stream.write(&[x]);
        }
        return;
    }

    let mut node = &root;
    loop {
        match *node {
            HuffmanNode::Leaf(x) => {
                output_stream.write(&[x]);
                bytes_left -= 1;
                if bytes_left == 0 { break; }
            }
            HuffmanNode::Node { ref left, ref right } => {
                node = if binary_in.get_bit() == 0 {
                    left.deref()
                } else {
                    right.deref()
                };
            }
        }
    }
}
