
use crate::utils;
//Day 1


pub fn main() -> u32  {
    let lines = utils::read_lines("./inputs/1.txt");
    let mut res = Vec::new();
    for mut line in lines {
        line.retain(|c| c.is_ascii_digit());
        res.push(line);
    }
    sum_lines(res)
}

fn sum_lines(lines: Vec<String>) -> u32 {
    lines.iter().map(|x| {
            let a = if x.len() > 1 {
                let join = format!("{}{}", x.chars().nth(0).unwrap(), x.chars().nth(x.len()-1).unwrap());
                join.parse::<u32>().unwrap()
            } else {
                let join = format!("{}{}", x.chars().nth(0).unwrap(), x.chars().nth(0).unwrap());
                join.parse::<u32>().unwrap()
            };
        a
        }).sum()
}

fn convert_words_to_numbers(input: &str) -> String {
    let convertion = [
        ("oneight","18"),
        ("threeight","38"),
        ("fiveight", "58"),
        ("nineight", "98"),
        ("sevenine","79"),
        ("eightwo", "82"),
        ("twone", "21"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let mut output = input.to_string();
    for (word, number) in convertion.iter() {
        output = output.replace(word, number);
    }
    output
}

pub fn numbers_in_letters() -> u32 {
    let lines = utils::read_lines("./inputs/1.txt");
    let mut res = Vec::new();
    for line in lines {
        let mut a = convert_words_to_numbers(&line);
        a.retain(|c| c.is_ascii_digit());
        res.push(a);
    }
    sum_lines(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = main();
        println!("Res: {}", result);
    }
}
