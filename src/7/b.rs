use std::env;
use std::fs;

use itertools::Itertools;

fn map(a: char) -> u8 {
    match a {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        _ => a.to_digit(10).unwrap() as u8,
    }
}

fn find_target(lengths: &Vec<(char, usize)>, target: usize) -> bool {
    if lengths.iter().any(|x| x.1 == target) {
        return true;
    }
    let o_joker = lengths.iter().find_or_first(|x| (**x).0 == 'J');
    if o_joker.is_none() || o_joker.unwrap().0 != 'J' {
        return false;
    }
    let joker = o_joker.unwrap();
    for len in lengths.iter().filter(|x| x.0 != 'J') {
        if len.1 + joker.1 >= target {
            return true;
        }
    }
    return false;
}

fn find_pair(lengths: &Vec<(char, usize)>, t1: usize, t2: usize) -> bool {
    if t1 == t2 && lengths.iter().filter(|x| x.1 == t1).count() == 2 {
        return true;
    }
    if t1 != t2 && lengths.iter().any(|x| x.1 == t1) && lengths.iter().any(|x| x.1 == t2) {
        return true;
    }
    let o_joker = lengths.iter().find_or_first(|x| (**x).0 == 'J');
    if o_joker.is_none() || o_joker.unwrap().0 != 'J' {
        return false;
    }
    let joker = o_joker.unwrap();
    for i in 0..joker.1 {
        let mut le = '0';
        let mut ri = '0';
        let over = joker.1 - i;
        for l in lengths.iter().filter(|x| x.0 != 'J') {
            if l.1 + i >= t1 {
                le = l.0;
                break;
            }
            if l.1 >= t2 {
                ri = l.0;
                break;
            }
        }
        if le == '0' && ri == '0' {
            continue;
        }
        let against = if le == '0' { ri } else { le };
        for l in lengths.iter().filter(|x| x.0 != 'J' && x.0 != against) {
            if l.1 + over - i >= t2 && l.0 != le && ri == '0' {
                ri = l.0;
                break;
            }
            if l.1 + over - i >= t1 && l.0 != ri && le == '0' {
                le = l.0;
                break;
            }
        }
        if le != '0' && ri != '0' {
            return true;
        }
    }
    return false;
}

fn get_strenght(hand: &str) -> i8 {
    let mut chars: Vec<char> = hand.chars().collect();
    chars.sort();
    let mut lengths: Vec<(char, usize)> = chars.iter().group_by(|x| *x)
        .into_iter().map(|(l, group)| {
            return (*l, group.collect::<Vec<&char>>().len());
        }).collect();
    lengths.sort_by(|a, b| b.1.cmp(&a.1));
    if find_target(&lengths, 5) {
        return 10;
    }
    else if find_target(&lengths, 4) {
        return 9;
    }
    else if find_pair(&lengths, 3, 2) {
        return 8;
    }
    else if find_target(&lengths, 3) {
        return 7;
    }
    else if find_pair(&lengths, 2, 2) {
        return 6;
    }
    else if find_target(&lengths, 2) {
        return 5;
    }
    return 0;
}

fn order(a: &str, b: &str) -> std::cmp::Ordering {
    if get_strenght(a) > get_strenght(b) {
        return std::cmp::Ordering::Less;
    }
    else if get_strenght(a) < get_strenght(b) {
        return std::cmp::Ordering::Greater;
    }
    let achar: Vec<char> = a.chars().collect();
    let bchar: Vec<char> = b.chars().collect();
    for i in 0..a.len() {
        if map(achar[i]) > map(bchar[i]) {
            return std::cmp::Ordering::Less;
        }
        else if map(achar[i]) < map(bchar[i]) {
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
        values += data[i].1 * ((i as i64) + 1);
    }
    println!("{}", values);
}