#![allow(unused_variables)]
use std::fs::File;
use std::io::{Read};
use std::io::prelude::*;

fn main() {
    let file = "test.txt";
    let mut output = String::new();

    let mut f = File::create(file).expect("File cannot be created");
    f.write_all(b"Hello world").expect("Failed to write string to file");

    let mut f = File::open(file).expect("Failed to open file");
    f.read_to_string(&mut output).expect("Failed to read string from file");

    println!("Output: {}", output);

    std::fs::remove_file(file).expect("Failed to delete file");
}
