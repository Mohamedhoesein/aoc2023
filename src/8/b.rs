use std::collections::HashMap;
use std::env;
use std::fs;
use num::integer::lcm;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = &args[1];
    let mut m: HashMap<String, (String, String)> = HashMap::new();
    let f = fs::read_to_string(file_name).unwrap();
    let path = f.lines().collect::<Vec<&str>>()[0];
    for line in f.lines().skip(1) {
        if line.len() == 0 {
            continue;
        }
        let mut parts: Vec<&str> = line.split(" = ").collect();
        let key: String = parts[0].to_owned();
        let mut value: String = parts[1].to_owned();
        value = value[1..value.len()-1].to_owned();
        parts = value.split(", ").collect::<Vec<&str>>();
        m.insert(key, (parts[0].to_owned(), parts[1].to_owned()));
    }
    let mut steps: i128 = 0;
    let mut paths: HashMap<String, i128> = HashMap::new();
    m.keys().filter(|x| x.ends_with("A")).for_each(|x| {paths.insert(x.to_owned(), 0);});
    for key in paths.clone().keys().to_owned() {
        let mut finished = false;
        let mut current: String = key.clone().to_owned();
        while !finished {
            for c in path.chars() {
                if c == 'L' {
                    current = m[&current].0.to_owned();
                } else {
                    current = m[&current].1.to_owned();
                }
                steps += 1;
                if current.ends_with("Z") {
                    println!("{} {}", steps, paths.len());
                    paths.remove(key);
                    paths.insert(key.clone().to_owned(), steps);
                    finished = true;
                    break;
                }
            }
        }
        steps = 0;
    }
    let base = *(paths.values().collect::<Vec<&i128>>()[0]);
    let r = paths.values().fold(base, |base, x| lcm(*x, base));
    println!("{}", r);
}