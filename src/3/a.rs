use std::env;
use std::fs;
use array2d;

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
    for line in lines.clone() {
        for c in line.chars() {
            let result = board.set(y, x, c);
            if result.is_err() {
                panic!("Error setting value: {:?}", result);
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }
    let mut elements: Vec<(i32, i32)> = Vec::new();
    for y in 0..rows {
        for x in 0..columns {
            let char = board.get(y, x).unwrap();
            if *char != '.' && !char.is_digit(10) {
                for i in -1..=1 {
                    for j in -1..=1 {
                        if i == 0 && j == 0 {
                            continue;
                        }
                        let new_x = x as i32 + i;
                        let new_y = y as i32 + j;
                        if new_x < 0 || new_y < 0 {
                            continue;
                        }
                        if (new_x - 1) == columns.try_into().unwrap() &&
                            (new_y) == rows.try_into().unwrap() &&
                            board.get(y, x).unwrap().is_digit(10) {
                            elements.push((new_y, new_x));
                        }
                        else if (new_x + 1) == columns.try_into().unwrap() &&
                            (new_y) == rows.try_into().unwrap() {
                            let mut found = false;
                            let mut t = new_x as usize;
                            #[warn(while_true)]
                            loop {
                                if board.get(new_y as usize, t).unwrap().is_digit(10) {
                                    found = true;
                                    if t == 0 {
                                        break;
                                    }
                                    t -= 1;
                                }
                                else {
                                    t += 1;
                                    break;
                                }
                            }
                            if found && !elements.contains(&(new_y, t as i32 + 1)) {
                                elements.push((new_y, t as i32 + 1));
                            }
                        }
                        else {
                            let mut found = false;
                            let mut t = new_x as usize;
                            #[warn(while_true)]
                            loop {
                                if board.get(new_y as usize, t).unwrap().is_digit(10) {
                                    found = true;
                                    if t == 0 {
                                        break;
                                    }
                                    t -= 1;
                                }
                                else {
                                    t += 1;
                                    break;
                                }
                            }
                            if found && !elements.contains(&(new_y, t as i32)) {
                                elements.push((new_y, t as i32));
                            }
                        }
                    }
                }
            }
        }
    }
    let mut f = 0;
    for pair in elements {
        y = pair.0 as usize;
        x = pair.1 as usize;
        let mut result = 0;
        while board.get(y, x).unwrap().is_digit(10) {
            result *= 10;
            result += board.get(y, x).unwrap().to_digit(10).unwrap();
            if x >= columns - 1 {
                break;
            }
            x += 1;
        }
        f += result;
    }
    println!("{}", f);
}