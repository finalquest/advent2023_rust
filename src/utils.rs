

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Range;

pub fn read_lines(path: &str) -> Vec<String> {
    let file = File::open(path).expect("File not found");
    BufReader::new(file).lines().map(|l| l.expect("Could not parse line")).collect()
}

pub trait OnlyDigits {
    fn only_digits(&mut self);
    fn only_digits_as_vector(&self) -> Vec<u32>;
    fn has_symbols(&self) -> bool;
    fn number_between_delimiter(&self, start_poisition: u32) -> &str;
    fn expand_range(&self) -> Vec<Range<u64>>;    
}


impl OnlyDigits for String {
    fn only_digits(&mut self) {
        self.retain(|c| c.is_ascii_digit())
    }

    fn only_digits_as_vector(&self) -> Vec<u32> {
        self.chars().filter(|c| c.is_ascii_digit() || c.is_whitespace()).collect::<String>().
        split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect()
    }

    fn has_symbols(&self) -> bool {
        self.chars().any(|c| !c.is_ascii_digit() && c != '.')
    }

    fn number_between_delimiter(&self, start_poisition: u32) -> &str {
        let mut pos = (start_poisition) as usize;
        // println!("start_poisition: {}", start_poisition);
        let chars = self.chars().collect::<Vec<char>>();
        for i in (0..pos+1).rev() {
            // println!("i: {}, char: {}", i, chars[i]);
            pos = i;
            if !chars[i].is_numeric() {
                break;
            }
            if i == 0 {
                break;
            }
            if !chars[i-1].is_numeric() {
                break;
            }
        }
        let sp = pos;
        pos = start_poisition as usize;
        for j in pos..chars.len() {
            pos = j;
            if !chars[j].is_numeric() {
                break;
            }
            if j == chars.len() - 1 {
                pos += 1;
                break;
            }
            if !chars[j+1].is_numeric() {
                pos += 1;
                break;
            }
        }
        let ep = pos;
        // println!("sp: {}, ep: {}", sp, ep);
        // println!("number: {}", &self[sp as usize..ep as usize]);
        &self[sp as usize..ep as usize]
    }
    fn expand_range(&self) -> Vec<Range<u64>> {
        //the range is in the forman x lenght so for example 79 2 means 79, 80
        let a = self.only_digits_as_vector().iter().map(|x| *x as u64).collect::<Vec<u64>>();
        let mut r:Vec<Range<u64>> = Vec::new();
        let mut c = 0;
        while c < a.len() {
                r.push(a[c]..a[c] + a[c+1]);
            c += 2;
        }
        r
    }
}
