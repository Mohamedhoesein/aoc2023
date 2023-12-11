use std::env;
use std::fs;

#[derive(Debug,PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

fn change_dir(token: char, current: Direction) -> Direction {
    if token == '7' {
        if current == Direction::Up {
            return Direction::Left;
        }
        else {
            return Direction::Down;
        }
    }
    else if token == 'F' {
        if current == Direction::Up {
            return Direction::Right;
        }
        else {
            return Direction::Down;
        }
    }
    else if token == 'L' {
        if current == Direction::Down {
            return Direction::Right;
        }
        else {
            return Direction::Up;
        }
    }
    else if token == 'J' {
        if current == Direction::Down {
            return Direction::Left;
        }
        else {
            return Direction::Up;
        }
    }
    return current;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = &args[1];
    let borrow = fs::read_to_string(file_name).unwrap();
    let lines = borrow.lines();
    let rows = lines.clone().count();
    let columns = lines.clone().next().unwrap().len();
    let mut board = array2d::Array2D::filled_with('.', rows, columns);
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut sx: usize = 0;
    let mut sy: usize = 0;
    for line in lines.clone() {
        for c in line.chars() {
            let _ = board.set(y, x, c);
            if c == 'S' {
                sx = x;
                sy = y;
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }

    let mut side1 = (sx, sy - 1);
    let mut side1_dir = Direction::Up;
    let mut side2 = (sx - 1, sy);
    let mut side2_dir = Direction::Left;
    let mut distance = 1;
    loop {
        let side1_pref = side1.clone();
        let side2_pref = side2.clone();
        if side1_dir == Direction::Up {
            side1.1 -= 1;
        }
        else if side1_dir == Direction::Right {
            side1.0 += 1;
        }
        else if side1_dir == Direction::Down {
            side1.1 += 1;
        }
        else if side1_dir == Direction::Left {
            side1.0 -= 1;
        }
        if side2_dir == Direction::Up {
            side2.1 -= 1;
        }
        else if side2_dir == Direction::Right {
            side2.0 += 1;
        }
        else if side2_dir == Direction::Down {
            side2.1 += 1;
        }
        else if side2_dir == Direction::Left {
            side2.0 -= 1;
        }
        let one = board.get(side1.1, side1.0).unwrap();
        side1_dir = change_dir(*one, side1_dir);
        let two = board.get(side2.1, side2.0).unwrap();
        side2_dir = change_dir(*two, side2_dir);
        distance += 1;
        if side1 == side2 || side1 == side2_pref || side2 == side1_pref {
            break;
        }
    }
    println!("{}", distance);
}