

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_lines(path: &str) -> Vec<String> {
    let file = File::open(path).expect("File not found");
    BufReader::new(file).lines().map(|l| l.expect("Could not parse line")).collect()
}

pub trait OnlyDigits {
    fn only_digits(&mut self);
    fn has_symbols(&self) -> bool;
    fn number_between_delimiter(&self, start_poisition: u32) -> &str;
    
}


impl OnlyDigits for String {
    fn only_digits(&mut self) {
        self.retain(|c| c.is_ascii_digit())
    }

    fn has_symbols(&self) -> bool {
        self.chars().any(|c| !c.is_ascii_digit() && c != '.')
    }

    fn number_between_delimiter(&self, start_poisition: u32) -> &str {
        let mut pos = (start_poisition) as usize;
        println!("start_poisition: {}", start_poisition);
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
            println!("j: {}, char: {}", j, chars[j]);
            pos = j;
            if !chars[j].is_numeric() {
                break;
            }
            if j == chars.len() - 1 {
                println!("salgo por j == chars.len() + 1");
                pos += 1;
                break;
            }
            if !chars[j+1].is_numeric() {
                println!("salgo por !chars[j+1].is_numeric()");
                pos += 1;
                break;
            }
        }
        let ep = pos;
        println!("sp: {}, ep: {}", sp, ep);
        println!("number: {}", &self[sp as usize..ep as usize]);
        &self[sp as usize..ep as usize]
    }
}
