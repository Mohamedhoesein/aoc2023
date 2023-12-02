use std::env;
use std::fs;

extern crate itertools;
use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = &args[1];
    let mut sum = 0;
    for line in fs::read_to_string(file_name).unwrap().lines() {
        let game_parts: Vec<&str> = line.split(":").collect();
        let counts: Vec<(String, i32)> = game_parts[1].trim().split(";").flat_map(|y| {
            let parts: Vec<(String, i32)> = y.trim().split(",").map(|z| {
                let parts: Vec<&str> = z.trim().split(" ").collect();
                let amount: i32 = parts[0].parse::<i32>().unwrap();
                return (parts[1].trim().to_lowercase(), amount);
            }).collect();
            return parts;
        }).sorted_by(|a, b| a.0.cmp(&b.0)).collect::<Vec<(String, i32)>>();
        let grouped: Vec<(String, Vec<i32>)> = counts.iter().group_by(|elt| elt.0.to_owned()).into_iter().map(|(l, group)| {
            return (l, group.collect::<Vec<&(String, i32)>>().iter().map(|x| (*x).1).collect::<Vec<i32>>());
        }).collect::<Vec<(String, Vec<i32>)>>();
        let mut i: i32 = 1;
        grouped.iter()
            .map(|x| *x.1.iter().max().unwrap()).collect::<Vec<i32>>()
            .iter().for_each(|t| i = *t * i);
        if grouped.len() != 3 {
            i = 0;
        }
        sum += i;
    }
    println!("{}", sum);
}