use clap::{App, Arg, ArgMatches};

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
            Arg::with_name("threads")
                .short("t")
                .long("threads")
                .value_name("THREAD_AM0UNT")
                .help("Sets the amount of threads to use")
                .help("Default is the amount of cores the CPU has")
                .takes_value(true),
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

    if matches.is_present("decompress") {
        println!("Decompression feature not implemented yet.");
    } else {
        let threads = get_threads(&matches)?;
        let compressed = huffman::compress(&content, threads);

        println!("Content size: {}", content.len());
        println!("Compressed size: {}", compressed.bits.len());
        println!(
            "Compressed to {:.2}% of original file",
            compressed.bits.len() as f64 / content.len() as f64 * 100.0
        );
    }

    Ok(())
}

pub fn get_threads(matches: &ArgMatches) -> Result<usize, String> {
    match matches.value_of("threads") {
        Some(t) => {
            let t = t.parse::<usize>();
            let cpus = num_cpus::get();
            match t {
                Ok(t) => {
                    if t > cpus {
                        Err(format!("There are only {} threads avaiable", cpus))
                    } else if t < 1 {
                        Err(format!("Unexpected amount of threads {}", t))
                    } else {
                        Ok(t)
                    }
                }
                Err(e) => Err(format!("Number of threads not valid: {}", e)),
            }
        }
        None => Ok(num_cpus::get()),
    }
}
