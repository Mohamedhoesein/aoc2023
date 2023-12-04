use std::env;
use std::fs;
use array2d;

fn part(part: &str) -> Vec<i32> {
    return part.trim().split(" ").map(|x| x.trim()).filter(|x| x.len() > 0).map(|x| {
        x.parse::<i32>().unwrap()
    }).collect();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = &args[1];
    let count = fs::read_to_string(file_name).unwrap().lines().collect::<Vec<&str>>().len();
    let mut board = array2d::Array2D::filled_with(1, count, 1);
    let mut i = 0;
    let mut val = 0;
    for line in fs::read_to_string(file_name).unwrap().lines() {
        let parts = line.split(":").skip(1).collect::<Vec<&str>>()[0]
            .split("|").map(|x| part(x.trim())).collect::<Vec<Vec<i32>>>();
        let winning = parts[0].clone();
        let having = parts[1].clone();
        let mut won = 0;
        for x in having.iter() {
            if winning.contains(x) {
                won += 1;
            }
        }
        let amount_add = board.get(i, 0).unwrap() * won;
        for j in (i + 1)..(i + won + 1) {
            if j > count {
                break;
            }
            board.set(j, 0, board.get(i, 0).unwrap() + board.get(j, 0).unwrap()).unwrap();
        }
        val += amount_add + 1;
        i += 1;
    }
    
    println!("{}", val);
}