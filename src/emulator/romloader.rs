use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;

pub fn load_rom(filename: &str) -> Vec<u8> {
    let mut byte_vector = vec!();
    let file = File::open(filename).unwrap();
    debug!("File: {:?}", file);
    for byte in file.bytes() {
        debug!("Byte: {:?}", byte);
        byte_vector.push(byte.unwrap());
    }
    byte_vector
}