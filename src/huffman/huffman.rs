use std::cmp::{PartialEq, Eq, PartialOrd, Ord, Ordering};
use std::collections::BinaryHeap;

pub enum HuffmanNode {
    Leaf(u8),
    Node {
        left: Box<HuffmanNode>,
        right: Box<HuffmanNode>
    },
}

struct NodeFreqPair {
    freq: u64,
    node: HuffmanNode,
}

impl PartialEq for NodeFreqPair {
    fn eq(&self, other: &NodeFreqPair) -> bool {
        self.freq == other.freq
    }
}

impl Eq for NodeFreqPair {}

impl PartialOrd for NodeFreqPair {
    fn partial_cmp(&self, other: &NodeFreqPair) -> Option<Ordering> {
        Some(self.freq.cmp(&other.freq))
    }
}

impl Ord for NodeFreqPair {
    fn cmp(&self, other: &NodeFreqPair) -> Ordering {
        self.freq.cmp(&other.freq)
    }
}

pub fn build_huffman_tree(freq_table: [u64; 256]) -> Option<HuffmanNode> {
    let mut q = BinaryHeap::new();

    for i in 0..freq_table.len() {
        if freq_table[i] > 0 {
            q.push(NodeFreqPair {
                freq: freq_table[i],
                node: HuffmanNode::Leaf(i as u8),
            });
        }
    }

    while q.len() > 1 {
        if let Some(x) = q.pop() {
            if let Some(y) = q.pop() {
                q.push(NodeFreqPair {
                    freq: x.freq + y.freq,
                    node: HuffmanNode::Node {
                        left: Box::new(x.node),
                        right: Box::new(y.node),
                    },
                });
            }
        }
    }
    
    if let Some(x) = q.pop() { Some(x.node) } else { None }
}
