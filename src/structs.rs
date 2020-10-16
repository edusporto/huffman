use bitvec::prelude::*;
use std::cmp::Ordering;

pub struct CompressedBuffer {
    pub tree: Node,
    pub bits: CompressedBits,
}

pub struct CompressedBits {
    pub container: Vec<BitVec<Msb0, u8>>,
}

impl CompressedBits {
    pub fn len(&self) -> usize {
        self.container.iter().map(|bv| bv.len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.container.iter().map(|bv| bv.len()).sum::<usize>() == 0
    }
}

/// Defines what can be stored inside a Node.
/// The Huffman tree in this program consists of nodes that
/// are either bytes or usizes which represent the added frequencies
/// of all child byte nodes.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Info {
    Byte(u8),
    Freq(usize),
}

#[derive(Debug, Clone)]
pub struct Node {
    pub info: Info,
    pub l: Option<Box<Node>>,
    pub r: Option<Box<Node>>,
}

impl Node {
    pub fn new(info: Info) -> Node {
        Node {
            info,
            l: None,
            r: None,
        }
    }

    fn insert(&mut self, left: bool, node: Node) -> Result<(), ()> {
        let node_location = if left { &mut self.l } else { &mut self.r };

        match node_location {
            Some(_) => Err(()),
            None => {
                *node_location = Some(Box::new(node));
                Ok(())
            }
        }
    }

    pub fn insert_l(&mut self, node: Node) -> Result<(), ()> {
        self.insert(true, node)
    }

    pub fn insert_r(&mut self, node: Node) -> Result<(), ()> {
        self.insert(false, node)
    }

    pub fn is_leaf(&self) -> bool {
        match (&self.l, &self.r) {
            (None, None) => true,
            _ => false,
        }
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.info.cmp(&other.info)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.info == other.info
    }
}

/// Struct that will initially populate the priority queue which will generate
/// the Huffman Tree.
#[derive(Debug, Copy, Clone)]
pub struct ByteFreq {
    pub byte: u8,
    pub freq: usize,
}

impl ByteFreq {
    pub fn new(byte: u8, freq: usize) -> ByteFreq {
        ByteFreq { byte, freq }
    }
}

impl Eq for ByteFreq {}

impl Ord for ByteFreq {
    fn cmp(&self, other: &Self) -> Ordering {
        self.freq.cmp(&other.freq)
    }
}

impl PartialOrd for ByteFreq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ByteFreq {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

/// Represents the nodes of the priority queue which will generate the Huffman Tree.
#[derive(Debug)]
pub enum PqPiece {
    ByteFreq(ByteFreq),
    Node(Node),
}

impl Eq for PqPiece {}

impl PqPiece {
    pub fn get_freq(&self) -> usize {
        match self {
            PqPiece::ByteFreq(bf) => bf.freq,
            PqPiece::Node(node) => match node.info {
                Info::Freq(f) => f,
                Info::Byte(_) => panic!("Can't get frequency from byte node"),
            },
        }
    }
}

impl Ord for PqPiece {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_freq().cmp(&other.get_freq())
    }
}

impl PartialOrd for PqPiece {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PqPiece {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
