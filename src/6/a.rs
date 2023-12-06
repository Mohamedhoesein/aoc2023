use std::env;
use std::fs;
use itertools::izip;

fn parse(line: &str) -> Vec<u64> {
    return line.split_whitespace().skip(1).filter(|x| x.len() > 0).map(|x| x.trim().parse::<u64>().unwrap()).collect()
}

fn zeros(size: usize) -> Vec<u64> {
    let mut zero_vec: Vec<u64> = Vec::with_capacity(size as usize);
    for _ in 0..size {
        zero_vec.push(0);
    }
    return zero_vec;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = &args[1];
    let raw: String = fs::read_to_string(file_name).unwrap();
    let lines: Vec<&str> = raw.lines().collect();
    let times: Vec<u64> = parse(lines[0]);
    let distances: Vec<u64> = parse(lines[1]);
    let len: usize = times.len();
    let mut data: Vec<u64> = zeros(len);
    let mut index = 0;
    for (time, distance) in izip!(times, distances) {
        let mut total: u64 = 0;
        for i in 1..time {
            let d = (time - i) * i;
            if d > distance {
                total += 1;
            }
            if total != 0 && d < distance {
                break;
            }
        }
        data[index] = total;
        index += 1;
    }
    let result = data.iter().filter(|x| **x > 0).fold(1, |acc, x| acc * x);
    println!("{:?}", result)
}