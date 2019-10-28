use std::cmp::Ordering;

#[derive(Debug, Copy, Clone)]
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
        let node_location: &mut Option<Box<Node>> = if left { &mut self.l } else { &mut self.r };

        match node_location {
            Some(_) => {
                return Err(());
            }
            None => {
                *node_location = Some(Box::new(node));
            }
        }

        Ok(())
    }

    pub fn insert_l(&mut self, node: Node) -> Result<(), ()> {
        self.insert(true, node)
    }

    pub fn insert_r(&mut self, node: Node) -> Result<(), ()> {
        self.insert(false, node)
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
