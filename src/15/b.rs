use std::env;
use std::fs;
use std::collections::HashMap;

fn f(chars: Vec<u8>) -> u64 {
    let mut sum: u64 = 0;
    for c in chars {
        sum += c as u64;
        sum *= 17;
        sum %= 256;
    }
    return sum;
}

fn string_to_ascii(string: String) -> Vec<u8> {
    return string.chars().map(|x| x as u8).collect::<Vec<u8>>();
}

fn main() {
    let mut hashmap: HashMap<u8, Vec<(String, String)>> = HashMap::new();
    let args: Vec<String> = env::args().collect();
    let file_name: &String = &args[1];
    let raw = fs::read_to_string(file_name).unwrap();
    let line = raw.lines().collect::<Vec<&str>>()[0].to_owned();
    let elements = line.split(',');
    for element in elements {
        if element.contains("=") {
            let pair = element.split('=').collect::<Vec<&str>>();
            let key = pair[0].to_owned();
            let value = pair[1].to_owned();
            let bucket = f(string_to_ascii(key.clone())) as u8;
            if hashmap.contains_key(&bucket) {
                let mut nested = hashmap.get(&bucket).unwrap().to_owned();
                let p = nested.iter().position(|x| x.0 == key);
                if p.is_some() {
                    nested[p.unwrap()].1 = value.to_owned();
                } else {
                    nested.push((key.clone(), value.clone()));
                }
                hashmap.insert(bucket, nested);
            } else {
                let mut nested = Vec::new();
                nested.push((key.clone(), value.clone()));
                hashmap.insert(bucket, nested);
            }
        }
        else {
            let key = element.split('-').collect::<Vec<&str>>()[0].to_owned();
            let bucket = f(string_to_ascii(key.clone())) as u8;
            if hashmap.contains_key(&bucket) {
                let mut nested = hashmap.get(&bucket).unwrap().to_owned();
                let p = nested.iter().position(|x| x.0 == key);
                if p.is_some() {
                    nested.remove(p.unwrap());
                }
                hashmap.insert(bucket, nested);
            }
        }
    }

    let result = hashmap.iter().flat_map(|x| {
        let box_multiplier = 1 + (*x.0 as u64);
        return x.1.iter().enumerate().map(move |y| {
            let index = y.0;
            let raw_value = y.1.1.to_owned();
            let value = raw_value.parse::<u64>().unwrap();
            println!("{} {} {}", box_multiplier, index + 1, value);
            return box_multiplier * ((index as u64) + 1) * value;
        }).collect::<Vec<u64>>();
    }).sum::<u64>();
    println!("{}", result);
}