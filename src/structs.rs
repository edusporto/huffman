use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Info {
    Byte(u8),
    Freq(usize),
}

#[derive(Debug)]
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

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum PqPiece {
    ByteFreq(ByteFreq),
    Node(Node),
}
