use std::env;
use std::fs;

fn parse(line: &str) -> u64 {
    return line.split(":").collect::<Vec<&str>>()[1].chars().filter(|c| !c.is_whitespace()).collect::<String>().parse::<u64>().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = &args[1];
    let raw: String = fs::read_to_string(file_name).unwrap();
    let lines: Vec<&str> = raw.lines().collect();
    let time: u64 = parse(lines[0]);
    let distance: u64 = parse(lines[1]);
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
    println!("{}", total);
}