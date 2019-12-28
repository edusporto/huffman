extern crate bitvec;
extern crate clap;

use bitvec::prelude::*;

pub mod structs;

use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap};

use structs::{ByteFreq, Info, Node, PqPiece};

pub fn compress(content: &[u8]) -> Box<[u8]> {
    let freq = freq_of_bytes(content);
    let freq_pq = map_to_pq(freq);
    let tree = pq_to_tree(freq_pq);
    let code_map = gen_code_map(tree);

    let mut compressed = BitVec::<BigEndian, u8>::new();
    for b in content {
        compressed.append(&mut code_map.get(&b).unwrap().clone());
    }

    compressed.into_boxed_slice()
}

fn freq_of_bytes(content: &[u8]) -> BTreeMap<u8, usize> {
    let mut freq: BTreeMap<u8, usize> = BTreeMap::new();

    for &b in content {
        *freq.entry(b).or_insert(0) += 1;
    }

    freq
}

fn map_to_pq(map: BTreeMap<u8, usize>) -> BinaryHeap<Reverse<PqPiece>> {
    map.iter()
        .map(|(&byte, &freq)| PqPiece::ByteFreq(ByteFreq { byte, freq }))
        .map(Reverse)
        .collect()
}

fn pq_to_tree(mut pq: BinaryHeap<Reverse<PqPiece>>) -> Node {
    while pq.len() > 1 {
        let (Reverse(a), Reverse(b)) = (pq.pop().unwrap(), pq.pop().unwrap());

        let (freq_a, freq_b) = (a.get_freq(), b.get_freq());

        let mut node = Node::new(Info::Freq(freq_a + freq_b));

        let bytefreq_to_node = |val| match val {
            PqPiece::ByteFreq(bf) => Node::new(Info::Byte(bf.byte)),
            PqPiece::Node(node) => node,
        };

        let (node_l, node_r) = (bytefreq_to_node(a), bytefreq_to_node(b));
        node.insert_l(node_l).unwrap();
        node.insert_r(node_r).unwrap();

        pq.push(Reverse(PqPiece::Node(node)));
    }

    let Reverse(tree) = pq.pop().unwrap();
    match tree {
        PqPiece::Node(node) => node,
        PqPiece::ByteFreq(_) => {
            panic!("The last piece remaining of the priority queue should be a node")
        }
    }
}

fn gen_code_map(first: Node) -> HashMap<u8, BitVec> {
    let mut code_map: HashMap<u8, BitVec> = HashMap::new();
    let mut stack: Vec<(Node, BitVec)> = Vec::new();
    stack.push((first, BitVec::new()));

    while let Some(stack_piece) = stack.pop() {
        let (node, bitvec) = stack_piece;
        let (node_l, node_r) = (node.l.unwrap(), node.r.unwrap());
        let (mut bitvec_l, mut bitvec_r) = (bitvec.clone(), bitvec.clone());
        bitvec_l.push(false);
        bitvec_r.push(true);

        let mut is_leaf = |node: Box<Node>, bitvec| {
            if let Info::Byte(b) = node.info {
                code_map.insert(b, bitvec);
            } else {
                stack.push((*node, bitvec));
            }
        };

        is_leaf(node_l, bitvec_l);
        is_leaf(node_r, bitvec_r);
    }

    code_map
}
