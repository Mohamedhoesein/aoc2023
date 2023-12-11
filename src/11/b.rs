use std::env;
use std::fs;
use pathfinding::directed::astar::astar;

fn get_neighbors(column: usize, row: usize, columns: usize, rows: usize) -> Vec<Point> {
    let mut neighbors: Vec<Point> = vec![];
    if column > 0 {
        neighbors.push(Point { column: column - 1, row: row });
    }
    if column < columns - 1 {
        neighbors.push(Point { column: column + 1, row: row });
    }
    if row > 0 {
        neighbors.push(Point { column: column, row: row - 1 });
    }
    if row < rows - 1 {
        neighbors.push(Point { column: column, row: row + 1 });
    }
    neighbors
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    row: usize,
    column: usize,
}

impl Point {
    fn diff(&self, other: &Point) -> usize {
        let x = self.column as i32 - other.column as i32;
        let y = self.row as i32 - other.row as i32;
        (x.abs() + y.abs()) as usize
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = &args[1];
    let borrow = fs::read_to_string(file_name).unwrap();
    let lines = borrow.lines();
    let rows = lines.clone().count();
    let columns = lines.clone().next().unwrap().len();
    let mut board = array2d::Array2D::filled_with('.', rows, columns);
    let mut empty_columns: Vec<usize> = vec![];
    let mut empty_rows: Vec<usize> = vec![];
    let mut column: usize = 0;
    let mut row: usize = 0;
    let mut points: Vec<Point> = vec![];
    for line in lines.clone() {
        if !line.contains("#") {
            empty_rows.push(row);
        }
        for c in line.chars() {
            let _ = board.set(row, column, c);
            if c == '#' {
                points.push(Point { column: column, row: row });
            }
            column += 1;
        }
        column = 0;
        row += 1;
    }
    board.as_columns().iter().enumerate().for_each(|(i, c)| {
        if !c.contains(&'#') {
            empty_columns.push(i);
        }
    });
    for row in empty_rows.clone() {
        for column in empty_columns.clone() {
            let _ = board.set(row, column, '#');
        }
    }
    let total_columns = board.num_columns() + empty_columns.len();
    let total_rows = board.num_rows() + empty_rows.len();
    let mut s = 0;
    let processed: Vec<(Point, Point)> = vec![];
    for p in points.clone() {
        for p2 in points.clone().iter().filter(|t| **t != p).map(|t| t) {
            if processed.contains(&(p, *p2)) || processed.contains(&(*p2, p)) {
                continue;
            }
            let r = astar(&p, |t| {
                let neighbours = get_neighbors(t.column, t.row, total_columns, total_rows);
                let values = neighbours.iter()
                .map(|a| {
                    if empty_columns.contains(&a.column) && a.column != t.column {
                        return (*a, 1000000);
                    }
                    if empty_rows.contains(&a.row) && a.row != t.row {
                        return (*a, 1000000);
                    }
                    return (*a, 1)
                })
                .collect::<Vec<(Point, usize)>>();
                return values;
            },
            |t| t.diff(p2),
            |t| t == p2);
            s += r.unwrap().1;
        }
    }
    println!("{}", s / 2);
}