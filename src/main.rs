use bitvec::prelude::*;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use huffman::structs::{ByteFreq, Info, Node, PqPiece};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file_name = args.get(1);

    match file_name {
        Some(_) => {}
        None => return Err(format!("Usage: {} [FILE TO COMPRESS]", args[0]).into()),
    }

    let mut f = File::open(file_name.unwrap())?;
    let mut content = Vec::new();
    f.read_to_end(&mut content)?;

    let mut freq: HashMap<u8, usize> = HashMap::new();

    content.iter().for_each(|&b| {
        *freq.entry(b).or_insert(0) += 1;
    });

    let mut freq_pq: BinaryHeap<_> = freq
        .iter()
        .map(|(&byte, &freq)| PqPiece::ByteFreq(ByteFreq { byte, freq }))
        .map(Reverse)
        .collect();

    while freq_pq.len() >= 2 {
        let Reverse(a) = freq_pq.pop().unwrap();
        let Reverse(b) = freq_pq.pop().unwrap();

        let freq_a = a.get_freq();
        let freq_b = b.get_freq();

        let node_l = match a {
            PqPiece::ByteFreq(bf) => Node::new(Info::Byte(bf.byte)),
            PqPiece::Node(node) => node,
        };

        let node_r = match b {
            PqPiece::ByteFreq(bf) => Node::new(Info::Byte(bf.byte)),
            PqPiece::Node(node) => node,
        };

        let mut node = Node::new(Info::Freq(freq_a + freq_b));
        node.insert_l(node_l).unwrap();
        node.insert_r(node_r).unwrap();

        freq_pq.push(Reverse(PqPiece::Node(node)));
    }

    let mut code_map: HashMap<u8, BitVec> = HashMap::new();
    let mut stack: Vec<(Node, BitVec)> = Vec::new();

    let Reverse(first) = freq_pq.pop().unwrap();
    let first = match first {
        PqPiece::Node(node) => node,
        PqPiece::ByteFreq(_) => {
            panic!("The last piece remaining of the priority queue shouldn't be a node")
        }
    };
    stack.push((first, BitVec::new()));

    while let Some(stack_piece) = stack.pop() {
        let (node, bitvec) = stack_piece;
        let (node_l, node_r) = (node.l.unwrap(), node.r.unwrap());
        let (mut bitvec_l, mut bitvec_r) = (bitvec.clone(), bitvec.clone());
        bitvec_l.push(false);
        bitvec_r.push(true);

        if let Info::Byte(b) = node_l.info {
            code_map.insert(b, bitvec_l);
        } else {
            stack.push((*node_l, bitvec_l));
        }

        if let Info::Byte(b) = node_r.info {
            code_map.insert(b, bitvec_r);
        } else {
            stack.push((*node_r, bitvec_r));
        }
    }

    let mut compressed = BitVec::<BigEndian, u8>::new();
    content.iter().for_each(|b| {
        compressed.append(&mut code_map.get(&b).unwrap().clone());
    });

    println!("{:?}", compressed);

    Ok(())
}
