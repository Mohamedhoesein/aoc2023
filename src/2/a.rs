use std::env;
use std::fs;

fn main() {
    let red = 12;
    let green = 13;
    let blue = 14;
    let args: Vec<String> = env::args().collect();
    let file_name: &String = &args[1];
    let mut i = 1;
    let mut sum = 0;
    for line in fs::read_to_string(file_name).unwrap().lines() {
        let r = line.split(":").skip(1).all(|x| {
            return x.trim().split(";").all(|y| {
                return y.trim().split(",").all(|z| {
                    let parts: Vec<&str> = z.trim().split(" ").collect();
                    let amount: i32 = parts[0].parse::<i32>().unwrap();
                    return (parts[1] == "red" && amount <= red) || (parts[1] == "green" && amount <= green) || (parts[1] == "blue" && amount <= blue);
                });
            });
        });
        if r {
            sum += i;
        }
        i += 1;
    }
    println!("{}", sum);
}