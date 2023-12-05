use std::env;
use std::fs;
use std::sync::Arc;

fn convert(key: i128, map: &Vec<(i128, i128, i128)>) -> i128 {
    for (k, v, p) in map {
        if key >= *v && key <= *v + *p {
            return *k + (key - *v);
        }
    }
    return key;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = &args[1];
    let mut seeds: Vec<i128> = Vec::new();
    let mut seed: Vec<(i128, i128, i128)> = Vec::new();
    let mut soil: Vec<(i128, i128, i128)> = Vec::new();
    let mut fertilizer: Vec<(i128, i128, i128)> = Vec::new();
    let mut water: Vec<(i128, i128, i128)> = Vec::new();
    let mut light: Vec<(i128, i128, i128)> = Vec::new();
    let mut temp: Vec<(i128, i128, i128)>= Vec::new();
    let mut humidity: Vec<(i128, i128, i128)> = Vec::new();
    let mut current: &mut Vec<(i128, i128, i128)> = &mut seed;
    let mut first: bool = true;
    for line in fs::read_to_string(file_name).unwrap().lines() {
        if first {
            first = false;
            line.split_whitespace().skip(1).map(|x| x.trim().parse::<i128>().unwrap()).for_each(|x| seeds.push(x));
            continue;
        }
        if line.len() == 0 {
            continue;
        }
        if line.starts_with("seed") {
            current = &mut seed;
        } else if line.starts_with("soil") {
            current = &mut soil;
        } else if line.starts_with("fertilizer") {
            current = &mut fertilizer;
        } else if line.starts_with("water") {
            current = &mut water;
        } else if line.starts_with("light") {
            current = &mut light;
        } else if line.starts_with("temp") {
            current = &mut temp;
        } else if line.starts_with("humidity") {
            current = &mut humidity;
        } else {
            let data: Vec<i128> = line.split_whitespace().map(|x| x.trim().parse::<i128>().unwrap()).collect();
            current.append(&mut vec![(data[0], data[1], data[2])]);
        }
    }
    let mut low_plot: i128 = i128::MAX;
    let mut handle: Vec<std::thread::JoinHandle<()>> = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        for s in seeds[i]..seeds[i] + seeds[i + 1] {
            let plot = convert(convert(convert(convert(convert(convert(convert(s, &seed), &soil), &fertilizer), &water), &light), &temp), &humidity);
            if plot < low_plot {
                low_plot = plot;
            }
        }
    }
    println!("{}", low_plot);
}