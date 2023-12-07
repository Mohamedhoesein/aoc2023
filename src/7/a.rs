use std::env;
use std::fs;

use itertools::Itertools;

fn map(a: char) -> u8 {
    match a {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => a.to_digit(10).unwrap() as u8,
    }
}

fn get_strenght(hand: &str) -> i8 {
    let mut chars: Vec<char> = hand.chars().collect();
    chars.sort();
    let lengths: Vec<usize> = chars.iter().group_by(|x| *x)
        .into_iter().map(|(_l, group)| {
            return group.collect::<Vec<&char>>().len();
        }).collect();
    if lengths.contains(&5) {
        return 10;
    }
    else if lengths.contains(&4) {
        return 9;
    }
    else if lengths.contains(&3) && lengths.contains(&2) {
        return 8;
    }
    else if lengths.contains(&3) {
        return 7;
    }
    else if lengths.iter().filter(|x| **x == 2).count() == 2 {
        return 6;
    }
    else if lengths.iter().filter(|x| **x == 2).count() == 1 {
        return 5;
    }
    return 0;
}

fn order(a: &str, b: &str) -> std::cmp::Ordering {
    println!("{} {}", a, b);
    if get_strenght(a) > get_strenght(b) {
        println!("str a > b");
        return std::cmp::Ordering::Less;
    }
    else if get_strenght(a) < get_strenght(b) {
        println!("str b > a");
        return std::cmp::Ordering::Greater;
    }
    let achar: Vec<char> = a.chars().collect();
    let bchar: Vec<char> = b.chars().collect();
    for i in 0..a.len() {
        if map(achar[i]) > map(bchar[i]) {
            println!("{} > {}", achar[i], bchar[i]);
            return std::cmp::Ordering::Less;
        }
        else if map(achar[i]) < map(bchar[i]) {
            println!("{} < {}", achar[i], bchar[i]);
            return std::cmp::Ordering::Greater;
        }
    }
    return std::cmp::Ordering::Equal;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = &args[1];
    let mut data: Vec<(&str, i64)> = Vec::new();
    let lines = fs::read_to_string(file_name).unwrap();
    for line in lines.lines() {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let hand = parts[0];
        let bid = parts[1].parse::<i64>().unwrap();
        data.push((hand, bid));        
    }
    data.sort_by(|a, b| order(a.0, b.0));
    data = data.iter().rev().map(|f| *f).collect();
    let mut values: i64 = 0;
    for i in 0..data.len() {
        println!("{:?} {}", data[i], ((i as i64) + 1));
        values += data[i].1 * ((i as i64) + 1);
    }
    println!("{}", values);
}