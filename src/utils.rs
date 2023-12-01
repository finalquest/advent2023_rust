

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_lines(path: &str) -> Vec<String> {
    let file = File::open(path).expect("File not found");
    BufReader::new(file).lines().map(|l| l.expect("Could not parse line")).collect()
}

pub trait OnlyDigits {
    fn only_digits(&mut self);
}

impl OnlyDigits for String {
    fn only_digits(&mut self) {
        self.retain(|c| c.is_ascii_digit())
    }
}
