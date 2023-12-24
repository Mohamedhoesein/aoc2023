use std::env;
use std::fs;

fn f(chars: Vec<u8>) -> u64 {
    let mut sum: u64 = 0;
    for c in chars {
        sum += c as u64;
        sum *= 17;
        sum %= 256;
    }
    return sum;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = &args[1];
    let raw = fs::read_to_string(file_name).unwrap();
    let line = raw.lines().collect::<Vec<&str>>()[0];
    let value = line.to_owned().split(',')
        .map(|x| x.to_owned().chars().map(|x| x as u8).collect::<Vec<u8>>())
        .map(f)
        .sum::<u64>();
    println!("{}", value);
}