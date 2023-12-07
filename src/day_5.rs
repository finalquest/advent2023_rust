use crate::utils::*;
//Day 1


pub fn main() -> u32  {
    let lines = read_lines("./inputs/5.txt");
    let mut r:Vec<u32> = Vec::new();
    let seeds = lines[0].only_digits_as_vector();
    for seed in seeds {
        let mut map = seed;
        for (i, line) in lines.iter().enumerate() {
            if i == 0 || line.chars().nth(0).is_none() {
                continue;
            }
            if !line.chars().nth(0).unwrap().is_numeric() {
                continue;
            }

            let lasv = line.only_digits_as_vector();
            // println!("lasv {:?}, seed:{}", lasv, seed);
            if seed >= lasv[1] && seed < lasv[1]+lasv[2] - 1 {
                map = lasv[1] + seed - lasv[2];
                // println!("line: {:?}, map:{}", line.only_digits_as_vector(), map);
            }
        }
        println!("res: {}", map)
    }
    0
}
