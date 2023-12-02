
use crate::utils::*;
use regex::Regex;

pub fn main() -> u32  {
    let lines = read_lines("./inputs/2.txt");
    let mut res: Vec<u32> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let a = line[line.find(':').unwrap()+2..line.len()].to_string();
        let split = a.split(";");
        let mut valid = true;
        for part in split {
            let values = Regex::new(r"(\d+)\s(green|blue|red)").unwrap(); 
            for capture in values.captures_iter(part) {
                let v = capture[1].parse::<u32>().unwrap();
                match &capture[2] {
                    "green" => {
                        if v > 13 {valid = false}
                    },
                    "blue" => {
                        if v > 14 {valid = false}
                    },
                    "red" => {
                        if v > 12 {valid = false}
                    },
                    _ => println!("no color"),
                }
            }
        } 

        if valid {
            res.push((i as u32) +1);
        } 
    }
    res.iter().sum::<u32>()
}

//part 2

pub fn part2() -> u32 {
    let lines = read_lines("./inputs/2.txt");
    let mut res: Vec<u32> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let a = line[line.find(':').unwrap()+2..line.len()].to_string();
        let split = a.split(";");
        let mut amount_red = 0;
        let mut amount_green = 0;
        let mut amount_blue = 0;

        for part in split {
            let values = Regex::new(r"(\d+)\s(green|blue|red)").unwrap(); 
            for capture in values.captures_iter(part) {
                let v = capture[1].parse::<u32>().unwrap();
                match &capture[2] {
                    "green" => {
                        if v > amount_green {amount_green = v}
                    },
                    "blue" => {
                        if v > amount_blue {amount_blue = v}
                    },
                    "red" => {
                        if v > amount_red {amount_red = v}
                    },
                    _ => println!("no color"),
                }
            }
        } 

        
        res.push(amount_red * amount_green * amount_blue);
     
    }
    res.iter().sum::<u32>()
}
