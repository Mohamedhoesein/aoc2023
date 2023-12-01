use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = &args[1];
    let mut values: i32 = 0;
    for line in fs::read_to_string(file_name).unwrap().lines() {
        let mut part: String = "".to_owned();
        for c in line.chars() {
            if c.is_digit(10) {
                part.push(c);
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_digit(10) {
                part.push(c);
                break;
            }
        }
        values += part.parse::<i32>().unwrap();
    }
    println!("{}", values);
}