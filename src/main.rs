use clap::{App, Arg};

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Huffman Compression Implementation")
        .version("0.0.2")
        .author("Eduardo Sandalo Porto")
        .about("Compresses files using the Huffman algorithm")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to compress")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("decompress")
                .short("d")
                .help("Decompresses the input file"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Sets the output compressed file")
                .takes_value(true)
                .default_value("output.hff"),
        )
        .get_matches();

    // The clap crate prevents INPUT from being empty
    let file_name = matches.value_of("INPUT").unwrap();
    let mut f = File::open(file_name)?;
    let mut content = Vec::new();
    f.read_to_end(&mut content)?;

    if !matches.is_present("decompress") {
        let threads = num_cpus::get();
        let compressed = huffman::compress(&content, threads);
        println!("Content size: {}", content.len());
        println!("Compressed size: {}", compressed.len());
        println!(
            "Compressed to {:.2}% of original file",
            compressed.len() as f64 / content.len() as f64 * 100.0
        );
    } else {
        println!("Decompression feature not implemented yet.");
    }

    Ok(())
}
