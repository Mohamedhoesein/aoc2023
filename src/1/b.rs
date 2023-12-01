use std::env;
use std::fs;

fn get_char(line: &str, index: usize) -> char {
    line.chars().nth(index).unwrap()
}

fn has_string(line: &str, index: usize, string: &str) -> bool {
    let mut i = 0;
    for c in string.chars() {
        if line.len() <= index + i {
            return false;
        }
        if get_char(line, index + i) != c {
            return false;
        }
        i += 1;
    }
    true
}

fn has_digit(line: &str, index: usize) -> &str {
    if has_string(line, index, "zero") {
        return "0";
    }
    else if has_string(line, index, "one") {
        return "1";
    }
    else if has_string(line, index, "two") {
        return "2";
    }
    else if has_string(line, index, "three") {
        return "3";
    }
    else if has_string(line, index, "four") {
        return "4";
    }
    else if has_string(line, index, "five") {
        return "5";
    }
    else if has_string(line, index, "six") {
        return "6";
    }
    else if has_string(line, index, "seven") {
        return "7";
    }
    else if has_string(line, index, "eight") {
        return "8";
    }
    else if has_string(line, index, "nine") {
        return "9";
    }
    else {
        return "";
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let mut values = 0;
    for line in fs::read_to_string(file_name).unwrap().lines() {
        let mut part = "".to_owned();
        for i in 0..line.len() {
            let c = get_char(line, i);
            if c.is_digit(10) {
                part.push(c);
                break;
            }
            let digit: &str = has_digit(line, i); 
            if digit.len() > 0 {
                part.push(digit.chars().next().unwrap());
                break;
            }
        }
        for i in (0..line.len()).rev() {
            let c = get_char(line, i);
            if c.is_digit(10) {
                part.push(c);
                break;
            }
            let digit: &str = has_digit(line, i); 
            if digit.len() > 0 {
                part.push(digit.chars().next().unwrap());
                break;
            }
        }
        values += part.parse::<i32>().unwrap();
    }
    println!("{}", values);
}