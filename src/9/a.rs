use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = &args[1];
    let mut values: i64 = 0;
    for line in fs::read_to_string(file_name).unwrap().lines() {
        let mut all_changes: Vec<Vec<i64>> = Vec::new();
        let current_changes = line.split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        if current_changes.iter().all(|x| *x == 0) {
            continue;
        }
        all_changes.push(current_changes.clone());
        let mut temp = current_changes.clone().iter().zip(current_changes.iter().skip(1)).map(|(x, y)| (y - x)).collect::<Vec<i64>>();
        all_changes.push(temp.clone());
        while temp.clone().iter().any(|x| *x != 0) {
            temp = temp.iter().zip(temp.iter().skip(1)).map(|(x, y)| (y - x)).collect::<Vec<i64>>();
            all_changes.push(temp.clone());
        }
        println!("{:?}", all_changes);
        let mut start = all_changes.len()-1;
        if all_changes.last().unwrap().len() == 0 {
            start -= 1;
        }
        let mut intermediate = 0;
        for i in (0..start).rev() {
            intermediate += all_changes[i].last().unwrap();
        }
        values += intermediate;
    }
    println!("{}", values);
}