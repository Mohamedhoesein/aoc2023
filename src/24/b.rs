use std::env;
use std::fs;
use z3::ast::Ast;

#[derive(Debug,Copy,Clone,PartialEq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug,Copy,Clone,PartialEq)]
struct Line {
    start: Point,
    offset: Point
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = &args[1];
    let mut lines: Vec<Line> = Vec::new();
    for line in fs::read_to_string(file_name).unwrap().lines() {
        let parts: Vec<&str> = line.split(" @ ").map(|s| s.trim()).collect::<Vec<&str>>().to_owned();
        let origin: Vec<&str> = parts[0].split(", ").map(|s| s.trim()).collect::<Vec<&str>>();
        let start = Point {
            x: origin[0].parse::<i64>().unwrap(),
            y: origin[1].parse::<i64>().unwrap(),
            z: origin[2].parse::<i64>().unwrap(),
        };
        let direction: Vec<&str> = parts[1].split(", ").map(|s| s.trim()).collect::<Vec<&str>>();
        let offset = Point {
            x: direction[0].parse::<i64>().unwrap(),
            y: direction[1].parse::<i64>().unwrap(),
            z: direction[2].parse::<i64>().unwrap(),
        };
        let line = Line {
            start,
            offset,
        };
        lines.push(line.clone());
    }

    //take from https://gist.github.com/WaterFace/1240609d0d4e15fa4ade3e471e7b501e
    let cfg = z3::Config::new();
    let context = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&context);

    let x = z3::ast::Int::new_const(&context, "x");
    let y = z3::ast::Int::new_const(&context, "y");
    let z = z3::ast::Int::new_const(&context, "z");
    let vx = z3::ast::Int::new_const(&context, "vx");
    let vy = z3::ast::Int::new_const(&context, "vy");
    let vz = z3::ast::Int::new_const(&context, "vz");

    for (i, line) in lines.iter().take(3).enumerate() {
        let a = z3::ast::Int::from_i64(&context, line.start.x);
        let va = z3::ast::Int::from_i64(&context, line.offset.x);
        let b = z3::ast::Int::from_i64(&context, line.start.y);
        let vb = z3::ast::Int::from_i64(&context, line.offset.y);
        let c = z3::ast::Int::from_i64(&context, line.start.z);
        let vc = z3::ast::Int::from_i64(&context, line.offset.z);

        let t = z3::ast::Int::new_const(&context, format!("t{i}"));
        solver.assert(&t.gt(&z3::ast::Int::from_i64(&context, 0)));
        solver.assert(&(x.clone() + vx.clone() * t.clone())._eq(&(a + va * t.clone())));
        solver.assert(&(y.clone() + vy.clone() * t.clone())._eq(&(b + vb * t.clone())));
        solver.assert(&(z.clone() + vz.clone() * t.clone())._eq(&(c + vc * t.clone())));
    }
    if solver.check() == z3::SatResult::Sat {
        let Some(m) = solver.get_model() else {
            println!("Failed to solve!");
            return;
        };
        println!("{}", m.eval(&(x + y + z), true).unwrap());
    }
}