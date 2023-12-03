
use crate::utils::*;
use regex::Regex;

pub fn main() -> u32  {
    let lines = read_lines("./inputs/3.txt");
    let mut res: Vec<u32> = Vec::new();

    println!("alpha: {}", '%'.is_alphanumeric());
    let numbers = Regex::new(r"(\d+)").unwrap();
    for (i,line) in lines.iter().enumerate() {
        numbers.find_iter(line).for_each(|x|{
            // println!("start: {}, end: {}",x.start(), x.end());
            // println!("{}: {}", i, x.as_str());
            //current line
            let mut pos_right_index = x.end() - 1;
            let mut pos_left_index = x.start();
            if pos_right_index < line.len() - 1 {
                pos_right_index = pos_right_index + 1;
            }
            if pos_left_index > 0 {
                pos_left_index = pos_left_index - 1;
            }

            println!("----------------");
            println!("left: {}, right: {}, x: {}, start: {}, end: {}", pos_left_index, pos_right_index, x.as_str(), x.start(), x.end() - 1);
            println!("line: {}", line);
            let char_right = line.chars().nth(pos_right_index).unwrap();
            let char_left = line.chars().nth(pos_left_index).unwrap();
            if !char_left.is_alphanumeric() && char_left != '.' {
                res.push(x.as_str().parse::<u32>().unwrap());
                println!("agrego {}", x.as_str());
            } else if !char_right.is_alphanumeric() && char_right != '.' {
                res.push(x.as_str().parse::<u32>().unwrap());
                println!("agrego {}", x.as_str());
            } else {
                let mut add = false;
                let mut diag_left_index = pos_left_index;
                let mut diag_right_index = pos_right_index;
                if pos_left_index > 0 {
                    diag_left_index = pos_left_index;
                }
                if pos_right_index < line.len() - 1 {
                    diag_right_index = pos_right_index + 1;
                }
                if i > 0 {
                    let prev_line = &lines[i-1];
                    let sub_string = prev_line.get(diag_left_index..diag_right_index).unwrap();
                    println!("prev_line: {}, sub_string: {}, has_symbols: {}", prev_line, sub_string, sub_string.to_string().has_symbols());
                    if sub_string.to_string().has_symbols() {
                        add = true;
                    }
                } 
                if i < lines.len() - 1 {
                    let next_line = &lines[i+1];
                    let sub_string = next_line.get(diag_left_index..diag_right_index).unwrap();
                    println!("next_line: {}, sub_string: {}, has_symbols: {}", next_line, sub_string, sub_string.to_string().has_symbols());
                    if sub_string.to_string().has_symbols() {
                        add = true;
                    }
                }
                if add {
                    res.push(x.as_str().parse::<u32>().unwrap());
                    println!("agrego: {}", x.as_str());
                }
            } 
        });
    }
    println!("res: {:?}", res);
    println!("res: {:?}", res.iter().sum::<u32>());
    0
}
