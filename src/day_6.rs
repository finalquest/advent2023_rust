use crate::utils::*;
//Day 1


pub fn main() -> u64  {
    let lines = read_lines("./inputs/6.txt");
    let s = lines[0].only_digits_as_vector();
    let d = lines[1].only_digits_as_vector();
    let mut res: Vec<u32> = Vec::new();
    println!("s: {:?}", s);
    println!("d: {:?}", d);
    
    for i in 0..s.len() {
        let mut r: Vec<u32> = Vec::new();
        for j in 1..s[i]+1 {
            // println!("i: {}, j: {}, d:{}", i, j, d[i]);
            let a = (s[i] - j) * (s[i] - (s[i] - j));
            println!("a: {}", a);
            if a > d[i] {
                r.push(j);
            }
        }
        res.push(r.len() as u32);
    }
    res.iter().product::<u32>().try_into().unwrap()
}

pub fn main2() -> u64 {
    let mut lines = read_lines("./inputs/6.txt");
    lines[0].only_digits();
    lines[1].only_digits();
    println!("lines: {:?}", lines);
    let s = lines[0].parse::<u64>().unwrap();
    let d = lines[1].parse::<u64>().unwrap();
    let mut count = 0;
    println!("s: {:?}", s);
    println!("d: {:?}", d);
        for j in 1..s+1 {
            // println!("i: {}, j: {}, d:{}", i, j, d[i]);
            let a = (s - j) * (s - (s - j));
            // println!("a: {}", a);
            if a > d  {
                count+=1
            }
    }
    count
}
