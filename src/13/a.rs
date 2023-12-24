use std::env;
use std::fs;

fn columns_same(board: &array2d::Array2D<char>, rows: usize, left_of_mirror: usize, right_of_mirror: usize) -> bool {
    for i in 0..rows {
        if board.get(i, left_of_mirror) != board.get(i, right_of_mirror) {
            return false;
        }
    }
    return true;
}

fn mirror_columns(board: &array2d::Array2D<char>, rows: usize, columns: usize, left_of_mirror: usize, right_of_mirror: usize) -> usize {
    let left_size = left_of_mirror + 1;
    let right_size = columns - right_of_mirror;
    let amount = if left_size < right_size { left_size } else { right_size };

    for i in 0..amount {
        if !columns_same(board, rows, left_of_mirror - i, right_of_mirror + i) {
            return 0;
        }
    }
    return left_of_mirror + 1;
}

fn rows_same(board: &array2d::Array2D<char>, columns: usize, top_of_mirror: usize, bot_of_mirror: usize) -> bool {
    for i in 0..columns {
        if board.get(top_of_mirror, i) != board.get(bot_of_mirror, i) {
            return false;
        }
    }
    return true;
}

fn mirror_rows(board: &array2d::Array2D<char>, rows: usize, columns: usize, top_of_mirror: usize, bot_of_mirror: usize) -> usize {
    let top_size = top_of_mirror + 1;
    let bot_size = rows - bot_of_mirror;
    let amount = if top_size < bot_size { top_size } else { bot_size };

    for i in 0..amount {
        if !rows_same(board, columns, top_of_mirror - i, bot_of_mirror + i) {
            return 0;
        }
    }
    return top_of_mirror + 1;
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = &args[1];
    let mut boards: Vec<array2d::Array2D<char>> = Vec::new();
    let mut current_lines: Vec<Vec<char>> = Vec::new();
    for line in fs::read_to_string(file_name).unwrap().lines().clone() {
        if line.len() == 0 {
            boards.push(array2d::Array2D::from_rows(&current_lines).unwrap());
            current_lines.clear();
            continue;
        }
        else {
            current_lines.push(line.chars().collect());
        }
    }

    boards.push(array2d::Array2D::from_rows(&current_lines).unwrap());

    let mut count = 0;

    for board in boards {
        let mut left_of_mirror = 0;
        let mut right_of_mirror = 1;
        let mut top_of_mirror = 0;
        let mut bot_of_mirror = 1;
        let rows = board.num_rows();
        let columns = board.num_columns();
        while right_of_mirror < columns {
            let mirror_size = mirror_columns(&board, rows, columns, left_of_mirror, right_of_mirror);
            if mirror_size > 0 {
                count += mirror_size;
                break;
            }
            left_of_mirror += 1;
            right_of_mirror += 1;
        }
        while bot_of_mirror < rows {
            let mirror_size = mirror_rows(&board, rows, columns, top_of_mirror, bot_of_mirror);
            if mirror_size > 0 {
                count += mirror_size * 100;
                break;
            }
            top_of_mirror += 1;
            bot_of_mirror += 1;
        }
    }
    println!("{}", count);
}