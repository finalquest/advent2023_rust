use crate::utils::*;
use std::cmp::Ordering;


pub fn main() -> u64  {

    fn get_card_value(card: &str) -> u32 {
        // println!("card: {}", card);
        match card {
            "A" => 13,
            "K" => 12,
            "Q" => 11,
            "T" => 10,
            "9" => 9,
            "8" => 8,
            "7" => 7,
            "6" => 6,
            "5" => 5,
            "4" => 4,
            "3" => 3,
            "2" => 2,
            "J" => 1,
            _ => card.parse::<u32>().unwrap()
        }
    }

    let lines = read_lines("./inputs/7.txt");
    let mut five_of_kind: Vec<&str> = Vec::new();
    let mut four_of_kind: Vec<&str> = Vec::new();
    let mut three_of_kind: Vec<&str> = Vec::new();
    let mut full_house: Vec<&str> = Vec::new();
    let mut double_pair: Vec<&str> = Vec::new();
    let mut pair: Vec<&str> = Vec::new();
    let mut high_card: Vec<&str> = Vec::new();


    for line in lines.iter() {
        let splitted = line.split(" ").collect::<Vec<&str>>();
        let mut repeated = splitted[0].to_string().repeated();

        // println!("splitted: {:?}, repeated: {:?}, line: {:?}", repeated.keys().len(), repeated, line);
        let vj = match repeated.get(&'J') {
            Some(&v) => v,
            _ => -1
        };

        println!("repeated prev: {:?}", repeated);
        if (vj < 5 && vj > -1) {
            let aa = repeated.iter()
                .filter(|&(key, _)| key != &'J')
                .max_by_key(|entry | entry.1).unwrap();

            repeated.insert(*aa.0, *aa.1 + repeated.get(&'J').unwrap());
            repeated.remove(&'J');
        }

        println!("repeated post: {:?}", repeated);

        if repeated.keys().len() == 1 {
            five_of_kind.push(line);
        } else if repeated.keys().len() == 2 {
            if repeated.values().any(|&x| x == 4) {
                four_of_kind.push(line);
            } else {
                full_house.push(line);
            }
        } else if repeated.keys().len() == 3 {
            if repeated.values().any(|&x| x == 3) {
                three_of_kind.push(line);
            } else {
                double_pair.push(line);
            }
        } else if repeated.keys().len() == 4 {
            pair.push(line);
        } else {
            high_card.push(line);
        }
    }

    fn sort(a: &str, b: &str ) -> Ordering {
        let a = a.split(" ").collect::<Vec<&str>>();
        let b = b.split(" ").collect::<Vec<&str>>();
        let va1 = get_card_value(a[0].chars().nth(0).unwrap().to_string().as_str());
        let va2 = get_card_value(a[0].chars().nth(1).unwrap().to_string().as_str());
        let va3 = get_card_value(a[0].chars().nth(2).unwrap().to_string().as_str());
        let va4 = get_card_value(a[0].chars().nth(3).unwrap().to_string().as_str());
        let va5 = get_card_value(a[0].chars().nth(4).unwrap().to_string().as_str());
        let vb1 = get_card_value(b[0].chars().nth(0).unwrap().to_string().as_str());
        let vb2 = get_card_value(b[0].chars().nth(1).unwrap().to_string().as_str());
        let vb3 = get_card_value(b[0].chars().nth(2).unwrap().to_string().as_str());
        let vb4 = get_card_value(b[0].chars().nth(3).unwrap().to_string().as_str());
        let vb5 = get_card_value(b[0].chars().nth(4).unwrap().to_string().as_str());

        if va1 != vb1 {
            return va1.cmp(&vb1);
        } else if va2 != vb2 {
            return va2.cmp(&vb2);
        } else if va3 != vb3 {
            return va3.cmp(&vb3);
        } else if va4 != vb4 {
            return va4.cmp(&vb4);
        } else if va5 != vb5 {
            return va5.cmp(&vb5);
        }
        Ordering::Equal
    }

    five_of_kind.sort_by(|a, b| sort(a, b));
    four_of_kind.sort_by(|a, b| sort(a, b));
    full_house.sort_by(|a, b| sort(a, b));
    three_of_kind.sort_by(|a, b| sort(a, b));
    double_pair.sort_by(|a, b| sort(a, b));
    pair.sort_by(|a, b| sort(a, b));
    high_card.sort_by(|a, b| sort(a, b));

    let mut res: Vec<&str> = Vec::new();
    res.append(&mut high_card);
    res.append(&mut pair);
    res.append(&mut double_pair);
    res.append(&mut three_of_kind);
    res.append(&mut full_house);
    res.append(&mut four_of_kind);
    res.append(&mut five_of_kind);

    // println!("prev: {:?}", vec![high_card, pair, double_pair, full_house, three_of_kind, four_of_kind, five_of_kind]);

    let result: u64 = res.iter().enumerate().fold(0, |acc, (index, &item)| {
        // You can use both `index` and `item` here
        // acc + &format!("{}: {}, ", index, item)
        let c = item.split(" ").collect::<Vec<&str>>()[1].to_string().parse::<u64>().unwrap();
        // println!("index: {}, item: {}, parsed: {}", index, item,c);
        acc + c * (index as u64 + 1)
    });
    println!("res: {:?}", result);

    0
}

