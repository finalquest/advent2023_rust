use crate::utils::*;
//Day 1


pub fn main() -> u64  {
    let lines = read_lines("./inputs/5.txt");
    let mut r:Vec<u64> = Vec::new();
    let seeds = lines[0].only_digits_as_vector().iter().map(|x| *x as u64).collect::<Vec<u64>>();
    for seed in seeds {
        // println!("seed: {}", seed);
        let mut map = seed;
        let mut last_value = seed;
        let mut mach = false;
        for (i, line) in lines.iter().enumerate() {
            // println!("ARRANCO FOR");
            if i == 0 || line.chars().nth(0).is_none() {
                // println!("1");
                continue;

            }
            if line.chars().nth(0).unwrap().is_alphabetic() {
                // println!("nueva linea type: {} last value fue, {}", line, map);
                last_value = map;
                mach = false;
                continue;
            }
            if !line.chars().nth(0).unwrap().is_numeric() {
                // println!("3");
                continue;
            }
            if mach {
                // println!("4");
                last_value = map;
                continue;
            }

            let lasv = line.only_digits_as_vector().iter().map(|x| *x as u64).collect::<Vec<u64>>();
            // println!("lasv {:?}, last:{}", lasv, last_value);
            if last_value >= lasv[1] && last_value <= lasv[1] +lasv[2] - 1 {
                let diff:i64 = (lasv[0] as i64 - lasv[1] as i64).try_into().unwrap();
                map = (last_value as i64 + diff) as u64;
                mach = true;
                // println!("diff: {}, map:{}", diff, map );
                // println!("line: {:?}, map:{}", line.only_digits_as_vector(), map);
            }
        }
        // println!("salgo del for");
        r.push(map);
        // println!("-----------------");
        // println!("res: {}", map)
    }
    r.iter().min().unwrap().clone()
}

pub fn main2() -> u64 {
    let lines = read_lines("./inputs/5.txt");
    let mut r:Vec<u64> = Vec::new();
    let seeds = lines[0].expand_range();
    // println!("seeds: {:?}", seeds.len());
    // println!("seeds: {:?}", seeds);
    for seed in &seeds {
        // println!("seed: {}", seed.start);
    let mut low = seed.start + 1;
    let mut high = seed.end;
    let mut min = val(&lines, seed.start);
    while low <= high {
        let mid = low + (high - low) / 2;
        // println!("mid: {}", mid);
        let v = val(&lines, mid);
        // println!("salgo del for");
        r.push(v);
        if v < min  {
            high = mid - 1;
            min = v;
        } else {
            low = mid + 1;
        }
        // println!("-----------------");
        // println!("res: {}", map)
    }
    }
    r.iter().min().unwrap().clone()
}

fn val(lines: &Vec<String>, ivalue: u64) -> u64 {
    let mut mach = false;
    let mut last_value = ivalue;
    let mut map = ivalue;
 for (i, line) in lines.iter().enumerate() {
            // println!("ARRANCO FOR");
            if i == 0 || line.chars().nth(0).is_none() {
                // println!("1");
                continue;

            }
            if line.chars().nth(0).unwrap().is_alphabetic() {
                // println!("nueva linea type: {} last value fue, {}", line, map);
                last_value = map;
                mach = false;
                continue;
            }
            if !line.chars().nth(0).unwrap().is_numeric() {
                // println!("3");
                continue;
            }
            if mach {
                // println!("4");
                last_value = map;
                continue;
            }

            let lasv = line.only_digits_as_vector().iter().map(|x| *x as u64).collect::<Vec<u64>>();
            // println!("lasv {:?}, last:{}", lasv, last_value);
            if last_value >= lasv[1] && last_value <= lasv[1] +lasv[2] - 1 {
                let diff:i64 = (lasv[0] as i64 - lasv[1] as i64).try_into().unwrap();
                map = (last_value as i64 + diff) as u64;
                mach = true;
                // println!("diff: {}, map:{}", diff, map );
                // println!("line: {:?}, map:{}", line.only_digits_as_vector(), map);
            }
        }
    map
}
