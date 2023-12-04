use std::env;
use std::fs;

fn part(part: &str) -> Vec<i32> {
    return part.trim().split(" ").map(|x| x.trim()).filter(|x| x.len() > 0).map(|x| {
        x.parse::<i32>().unwrap()
    }).collect();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = &args[1];
    let mut values: i32 = 0;
    for line in fs::read_to_string(file_name).unwrap().lines() {
        let parts = line.split(":").skip(1).collect::<Vec<&str>>()[0]
            .split("|").map(|x| part(x.trim())).collect::<Vec<Vec<i32>>>();
        let winning = parts[0].clone();
        let having = parts[1].clone();
        let mut current = 0;
        for x in having.iter() {
            if winning.contains(x) {
                if current == 0 {
                    current += 1;
                } else {
                    current *= 2;
                }
            }
        }
        values += current;
    }
    println!("{}", values);
}