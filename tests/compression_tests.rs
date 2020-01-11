extern crate huffman;

use huffman::compress;

#[test]
fn containing_all_zeroes() {
    let empty = [0_u8; 1024];
    compress(&empty, 1);
}

#[test]
fn two_different_bytes() {
    let mut empty = [0_u8; 1024];
    empty[0] = 1;
    compress(&empty, 1);
}
