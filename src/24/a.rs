use std::env;
use std::fs;

#[derive(Debug,Copy,Clone,PartialEq)]
struct Point {
    x: i128,
    y: i128
}

#[derive(Debug,Copy,Clone,PartialEq)]
struct Line {
    start: Point,
    end: Point,
    offset: Point
}

impl Line {
    fn in_line(&self, p: Point) -> bool {
        if self.offset.x < 0 && self.offset.y < 0 {
            return p.x <= self.start.x && p.y <= self.start.y;
        }
        else if self.offset.x < 0 && self.offset.y > 0 {
            return p.x <= self.start.x && p.y >= self.start.y;
        }
        else if self.offset.x > 0 && self.offset.y < 0 {
            return p.x >= self.start.x && p.y <= self.start.y;
        }
        else if self.offset.x > 0 && self.offset.y > 0 {
            return p.x >= self.start.x && p.y >= self.start.y;
        }
        else if self.offset.x == 0 && self.offset.y < 0 {
            return p.x == self.start.x && p.y <= self.start.y;
        }
        else if self.offset.x == 0 && self.offset.y > 0 {
            return p.x == self.start.x && p.y >= self.start.y;
        }
        else if self.offset.x < 0 && self.offset.y == 0 {
            return p.x <= self.start.x && p.y == self.start.y;
        }
        else if self.offset.x > 0 && self.offset.y == 0 {
            return p.x >= self.start.x && p.y == self.start.y;
        }
        else if self.offset.x == 0 && self.offset.y == 0 {
            return p.x == self.start.x && p.y == self.start.y;
        }
        return false;
    }
}

#[derive(Debug,Copy,Clone)]
struct Intersection {
    line1: Line,
    line2: Line,
    point: Point
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = &args[1];
    let mut lines: Vec<Line> = Vec::new();
    for line in fs::read_to_string(file_name).unwrap().lines() {
        let parts: Vec<&str> = line.split(" @ ").map(|s| s.trim()).collect::<Vec<&str>>().to_owned();
        let origin: Vec<&str> = parts[0].split(", ").map(|s| s.trim()).collect::<Vec<&str>>();
        let start = Point {
            x: origin[0].parse::<i128>().unwrap(),
            y: origin[1].parse::<i128>().unwrap(),
        };
        let direction: Vec<&str> = parts[1].split(", ").map(|s| s.trim()).collect::<Vec<&str>>();
        let end = Point {
            x: start.x + direction[0].parse::<i128>().unwrap(),
            y: start.y + direction[1].parse::<i128>().unwrap(),
        };
        let offset = Point {
            x: direction[0].parse::<i128>().unwrap(),
            y: direction[1].parse::<i128>().unwrap(),
        };
        let line = Line {
            start,
            end,
            offset,
        };
        lines.push(line.clone());
    }
    let mut checked: Vec<(Line, Line)> = Vec::new();
    let mut intersections: Vec<Intersection> = Vec::new();
    for l1 in lines.iter() {
        for l2 in lines.iter() {
            if l1 == l2 || checked.contains(&(l1.clone(), l2.clone())) || checked.contains(&(l2.clone(), l1.clone())) {
                continue;
            }
            checked.push((l1.clone(), l2.clone()));
            let x1 = l1.start.x;
            let y1 = l1.start.y;
            let x2 = l1.end.x;
            let y2 = l1.end.y;
            let x3 = l2.start.x;
            let y3 = l2.start.y;
            let x4 = l2.end.x;
            let y4 = l2.end.y;
            if ((x1 - x2)*(y3 - y4) - (y1 - y2)*(x3 - x4)) == 0 {
                continue;
            }
            let x = ((x1*y2 - y1*x2)*(x3 - x4) - (x1 - x2)*(x3*y4 - y3*x4)) /
                        ((x1 - x2)*(y3 - y4) - (y1 - y2)*(x3 - x4));
            let y = ((x1*y2 - y1*x2)*(y3 - y4) - (y1 - y2)*(x3*y4 - y3*x4)) /
                        ((x1 - x2)*(y3 - y4) - (y1 - y2)*(x3 - x4));
            let point = Point {
                x,
                y,
            };
            let intersection = Intersection {
                line1: l1.clone(),
                line2: l2.clone(),
                point,
            };
            intersections.push(intersection);
        }
    }
    intersections = intersections.iter().filter(|i| {
        return i.line1.in_line(i.point) && i.line2.in_line(i.point) &&
                i.point.x >= 200000000000000 && i.point.y >= 200000000000000 &&
                i.point.x <= 400000000000000 && i.point.y <= 400000000000000;
    }).map(|x| *x).collect();
    let amount = intersections.len();
    println!("{}", amount);
}