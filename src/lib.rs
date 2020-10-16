extern crate bitvec;
extern crate clap;
extern crate num_cpus;
extern crate rayon;

use bitvec::prelude::*;
use rayon::prelude::*;

pub mod structs;

use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};

use structs::{ByteFreq, CompressedBits, CompressedBuffer, Info, Node, PqPiece};

pub fn compress(content: &[u8], threads: usize) -> CompressedBuffer {
    let freq = freq_of_bytes(content, threads);

    let freq_pq = map_to_pq(freq);

    let tree = pq_to_tree(freq_pq);

    let code_map = gen_code_map(tree.clone());

    let compressed = compress_bits(content, &code_map, threads);

    CompressedBuffer {
        tree,
        bits: compressed,
    }
}

impl CompressedBuffer {
    pub fn into_bitvec(self) -> BitVec<Msb0, u8> {
        let mut compressed = BitVec::<Msb0, u8>::new();

        let compressed_tree = compress_tree(self.tree);
        let compressed_bits = self.bits;

        compressed_tree.iter().for_each(|&bit| compressed.push(bit));
        for bits in compressed_bits.container.iter() {
            bits.iter().for_each(|&bit| compressed.push(bit));
        }

        compressed
    }
}

pub fn compress_tree(tree: Node) -> BitVec<Msb0, u8> {
    let mut compressed = BitVec::<Msb0, u8>::new();
    let mut stack = vec![tree];

    while let Some(node) = stack.pop() {
        if node.is_leaf() {
            compressed.push(true);

            if let Info::Byte(byte) = node.info {
                for i in [128, 64, 32, 16, 8, 4, 2, 1].iter() {
                    // push each of the byte's bits to the bitvec
                    compressed.push(byte & i > 0);
                }
            } else {
                panic!(
                    "(internal error) All of the leaf nodes from the tree should contain a byte"
                );
            }
        } else {
            compressed.push(false);
        }

        if let Some(node_l) = node.l {
            stack.push(*node_l);
        }

        if let Some(node_r) = node.r {
            stack.push(*node_r);
        }
    }

    compressed
}

fn freq_of_bytes(content: &[u8], threads: usize) -> BTreeMap<u8, usize> {
    content
        .par_chunks(content.len() / threads)
        .map(|x| freq(x))
        .reduce_with(combine)
        .unwrap_or_default()
}

fn freq(content: &[u8]) -> BTreeMap<u8, usize> {
    let mut freq: BTreeMap<u8, usize> = BTreeMap::new();

    for &b in content {
        *freq.entry(b).or_insert(0) += 1;
    }

    freq
}

fn combine(mut m1: BTreeMap<u8, usize>, m2: BTreeMap<u8, usize>) -> BTreeMap<u8, usize> {
    for (key, val) in m2.iter() {
        *m1.entry(*key).or_insert(0) += *val;
    }

    m1
}

fn map_to_pq(map: BTreeMap<u8, usize>) -> BinaryHeap<Reverse<PqPiece>> {
    map.iter()
        .map(|(&byte, &freq)| PqPiece::ByteFreq(ByteFreq { byte, freq }))
        .map(Reverse)
        .collect()
}

fn pq_to_tree(mut pq: BinaryHeap<Reverse<PqPiece>>) -> Node {
    if pq.len() == 1 {
        // The file to be compressed only contains repetitions of the same byte
        let Reverse(a) = pq.pop().unwrap();
        match a {
            PqPiece::ByteFreq(bf) => return Node::new(Info::Byte(bf.byte)),
            PqPiece::Node(_) => panic!(
                "(internal error) A node was added to the priority queue before \
                 creating the pq_to_tree function"
            ),
        }
    }

    while pq.len() >= 2 {
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
        PqPiece::ByteFreq(_) => panic!(
            "(internal error) The last piece remaining of the priority queue should be a node"
        ),
    }
}

fn gen_code_map(first: Node) -> Vec<BitVec<Msb0, u8>> {
    // Using a Vec as the map is more performant than using a HashMap
    // or BTreeMap and does not have significant memory usage impacts
    let mut code_vec: Vec<BitVec<Msb0, u8>> = vec![BitVec::new(); 256];
    let mut stack: Vec<(Node, BitVec<Msb0, u8>)> = Vec::new();

    if first.is_leaf() {
        // The file to be compressed only contains repetitions of the same byte
        if let Info::Byte(b) = first.info {
            let mut bv = BitVec::<Msb0, u8>::new();
            bv.push(false);
            code_vec[b as usize] = bv;

            return code_vec;
        } else {
            panic!("(internal error) The only node of the tree should be a byte");
        }
    }

    stack.push((first, BitVec::new()));

    while let Some(stack_piece) = stack.pop() {
        let (node, bitvec) = stack_piece;
        let (node_l, node_r) = (node.l.unwrap(), node.r.unwrap());
        let (mut bitvec_l, mut bitvec_r) = (bitvec.clone(), bitvec.clone());
        bitvec_l.push(false);
        bitvec_r.push(true);

        let mut treat_leaf = |node: Box<Node>, bitvec| match node.info {
            Info::Byte(b) => {
                // found a byte while going through the tree
                code_vec[b as usize] = bitvec;
            }
            Info::Freq(_) => {
                // keep searching for bytes
                stack.push((*node, bitvec));
            }
        };

        treat_leaf(node_l, bitvec_l);
        treat_leaf(node_r, bitvec_r);
    }

    code_vec
}

fn compress_bits(content: &[u8], code_map: &[BitVec<Msb0, u8>], threads: usize) -> CompressedBits {
    let mut compressed_vec: Vec<(usize, BitVec<Msb0, u8>)> = content
        .par_chunks(content.len() / threads)
        .enumerate()
        .map(|(i, chunk)| {
            let mut compressed_chunk = BitVec::<Msb0, u8>::new();

            for &b in chunk {
                let code = &code_map[b as usize];

                for bit in code.iter() {
                    compressed_chunk.push(*bit);
                }
            }

            (i, compressed_chunk)
        })
        .collect();

    compressed_vec.sort();

    CompressedBits {
        container: compressed_vec.into_iter().map(|tuple| tuple.1).collect(),
    }
}
