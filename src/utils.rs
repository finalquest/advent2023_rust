

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_lines(path: &str) -> Vec<String> {
    let file = File::open(path).expect("File not found");
    BufReader::new(file).lines().map(|l| l.expect("Could not parse line")).collect()
}

