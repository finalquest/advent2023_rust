use crate::utils::*;
//Day 1


pub fn main() -> u32  {
    let lines = read_lines("./inputs/4.txt");
    let mut r:Vec<u32> = Vec::new();
    for line in lines {
        let mut res = Vec::new();
        let no_game = line.split(":").collect::<Vec<&str>>()[1].to_string();
        let numbers = no_game.split("|").map(|x| x.to_string()).collect::<Vec<String>>();
        let winning_numbers = numbers[0].split(" ").map(|x| x.to_string()).collect::<Vec<String>>(); 
        let own_numbers = numbers[1].split(" ").filter(|x| x!=&"").map(|x| x.to_string()).collect::<Vec<String>>();

        for num in winning_numbers {
            // println!("num: {}, own_numbers:{:?}", num, own_numbers);
            if own_numbers.contains(&num) {
                res.push(num.parse::<u32>().unwrap());
            }
        }
    // println!("--------------");
    // println!("{:?}", res);
    if res.len() > 0 {
        r.push(u32::pow(2, res.len() as u32 -1));
    }
    }
    println!("{:?}", r);
    r.iter().sum()
}
