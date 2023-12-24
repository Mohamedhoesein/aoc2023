use std::env;
use std::fs;

use itertools::Itertools;

#[derive(Debug,Clone,Copy)]
struct Point {
    row: usize,
    column: usize
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = &args[1];
    let mut row: usize = 0;
    let mut column: usize = 0;
    let mut round: Vec<Point> = Vec::new();
    let mut square: Vec<Point> = Vec::new();
    for line in fs::read_to_string(file_name).unwrap().lines() {
        for c in line.chars() {
            if c == '#' {
                square.push(Point { row: row, column: column });
            }
            if c == 'O' {
                round.push(Point { row: row, column: column });
            }
            column += 1;
        }
        column = 0;
        row += 1;
    }
    round.sort_by(|a, b| a.row.cmp(&b.row));
    for i in 0..round.len() {
        let r = round.get(i).unwrap();
        let mut all = square.to_vec();
        let mut other = round.iter().filter(|x| x.column != r.column || x.row != r.row).map(|x| *x).collect::<Vec<Point>>();
        all.append(&mut other);
        all = all.iter().filter(|a| a.column == r.column && a.row < r.row).sorted_by(|a, b| a.row.cmp(&b.row)).rev().map(|a| *a).collect();
        let p1 = all.get(0);
        if p1.is_none() {
            round[i].row = 0;
            continue;
        }
        let p = p1.unwrap();
        round[i].row = p.row + 1;
    }
    let result: usize = round.iter().map(|x| row - x.row).sum();
    println!("{}", result);
}