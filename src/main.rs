//use bitvec::prelude::*;

use std::collections::{BinaryHeap, HashMap};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::cmp::Reverse;

use huffman::structs::{ByteFreq, PqPiece, Info, Node};

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

        // TODO: Create function to compare ByteFreq to Node
    }

    for val in freq_pq.into_sorted_vec() {
        println!("{:?}", val);
    }

    Ok(())
}
