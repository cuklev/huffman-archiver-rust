use std::io::{Read, Write};
use std::ops::Deref;
use huffman::huffman::{HuffmanNode, build_huffman_tree};
use binary::binary_writer::BinaryWriter;

pub fn read_freq_table<R: Read>(input_stream: &mut R) -> [u64; 256] {
    let mut freq_table = [0; 256];
    let mut ch = 0;

    while let Ok(_) = input_stream.read(&mut [ch]) {
        freq_table[ch as usize] += 1;
    }

    freq_table
}

fn dfs<W: Write>(node: &HuffmanNode, bit_sequence: &mut Vec<bool>, table: &mut [Vec<bool>], binary_out: &mut BinaryWriter<W>) {
    match *node {
        HuffmanNode::Leaf(ref x) => {
            binary_out.put_bit(1);
            binary_out.put_u8(*x);
            table[*x as usize] = bit_sequence.clone();
        }
        HuffmanNode::Node { ref left, ref right } => {
            binary_out.put_bit(0);

            bit_sequence.push(false);
            dfs(left.deref(), bit_sequence, table, binary_out);

            let index = bit_sequence.len() - 1;
            bit_sequence[index] = true;
            dfs(right.deref(), bit_sequence, table, binary_out);

            bit_sequence.pop();
        }
    }
}

pub fn compress<R: Read, W: Write>(freq_table: [u64; 256], input_stream: &mut R, binary_out: &mut BinaryWriter<W>) {
    let mut byte_length = 0;
    let mut non_zero = -1;
    for i in 0..freq_table.len() {
        byte_length += freq_table[i];
        if freq_table[i] > 0 {
            if non_zero >= 0 { non_zero = -2; }
            else if non_zero == -1 { non_zero = i as i32; }
        }
    }

    binary_out.put_u64(byte_length);

    if byte_length == 0 { return; }
    if non_zero >= 0 {
        binary_out.put_bit(1);
        binary_out.put_u8(non_zero as u8);
        return;
    }

    if let Some(root) = build_huffman_tree(freq_table) {
        let mut table = vec![];
        for _ in 0..freq_table.len() {
            table.push(vec![]);
        }

        let mut bit_sequence = vec![];
        dfs(&root, &mut bit_sequence, table.as_mut_slice(), binary_out);

        let mut ch = 0;
        while let Ok(_) = input_stream.read(&mut [ch]) {
            for bit in &table[ch as usize] {
                binary_out.put_bit(if *bit {1} else {0});
            }
        }
    }
}
