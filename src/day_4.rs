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

pub fn main2() -> u32  {
    let lines = read_lines("./inputs/4.txt");
    let mut tup = lines.iter().map(|x| (x, 1)).collect::<Vec<(&String, u32)>>();
    
    let mut r:Vec<u32> = Vec::new();
    let mut i = 0;
    println!("len: {:?}", tup);
    while i < tup.len() {
        println!();
        println!("ARRANCO LINEA: {} con start iter: {}", tup[i].0, tup[i].1);
        for k in 0..tup[i].1 {
            let mut res = Vec::new();
            let no_game = tup[i].0.split(":").collect::<Vec<&str>>()[1].to_string();
            let numbers = no_game.split("|").map(|x| x.to_string()).collect::<Vec<String>>();
            let winning_numbers = numbers[0].split(" ").map(|x| x.to_string()).collect::<Vec<String>>(); 
            let own_numbers = numbers[1].split(" ").filter(|x| x!=&"").map(|x| x.to_string()).collect::<Vec<String>>();
            for num in winning_numbers {
                // println!("num: {}, own_numbers:{:?}", num, own_numbers);
                if own_numbers.contains(&num) {
                    res.push(num.parse::<u32>().unwrap());
                }
            }
            if res.len() == 0 {
                break;
            }
            // println!("--------------");
            // println!("{:?}", res);
            let length = if res.len() > tup.len() - i {
                tup.len() - i
            } else {
                res.len()
            };

            // println!("k: {}", k);
            // println!("line: {:?}", tup[i]);
            // println!("res: {:?}", res);
            // println!("length: {}", length);
            for j in 0..length {
                if(i+j+1 < tup.len()) {
                    tup[i+j+1]= (tup[i+j+1].0, tup[i+j+1].1+1)
                }
            }
        }
        i+=1;
    }
    tup.iter().map(|x| x.1).sum()
    // for line in lines {
    //
    // if res.len() > 0 {
    //     r.push(u32::pow(2, res.len() as u32 -1));
    // }
    // }
    // println!("{:?}", r);
    // r.iter().sum()
}
