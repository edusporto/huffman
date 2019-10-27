use bitvec::prelude::*;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct ByteFreq {
    pub byte: u8,
    pub freq: usize,
}

impl ByteFreq {
    fn new(byte: u8, freq: usize) -> ByteFreq {
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
        .map(|(&byte, &freq)| ByteFreq { byte, freq })
        .collect();

    /*while let Some(a) = freq_pq.pop() {
        println!("{:?}", a);
    }

    println!("{:?}", freq_pq);*/

    Ok(())
}
