use regex::Regex;

use crate::utils::utils::read_lines;

struct Machine {
    one: (i128, i128, i128),
    two: (i128, i128, i128),
}

pub fn run1() {
    let filename = "src/inputs/13.txt";
    if let Ok(lines) = read_lines(filename) {
        let lines: Vec<String> = lines.flatten().collect();
        let num_lines = lines.len();

        let mut total_tokens = 0;
        let mut i = 0;
        let re1 = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
        let re2 = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();
        while i < num_lines {
            let captures1 = re1.captures(&lines[i]).unwrap();
            let x1 = captures1.get(1).map_or("", |m| m.as_str()).parse().unwrap();
            let x2 = captures1.get(2).map_or("", |m| m.as_str()).parse().unwrap();

            let captures2 = re1.captures(&lines[i + 1]).unwrap();
            let y1 = captures2.get(1).map_or("", |m| m.as_str()).parse().unwrap();
            let y2 = captures2.get(2).map_or("", |m| m.as_str()).parse().unwrap();

            let captures3 = re2.captures(&lines[i + 2]).unwrap();
            let p1: i128 = captures3.get(1).map_or("", |m| m.as_str()).parse().unwrap();
            let p2: i128 = captures3.get(2).map_or("", |m| m.as_str()).parse().unwrap();

            let machine = Machine {
                one: (x1, y1, p1),
                two: (x2, y2, p2),
            };

            let tokens = min_tokens(&machine);
            if tokens > 0 {
                total_tokens += tokens;
            }

            i += 4;
        }

        println!("Tokens: {}", total_tokens);
    }
}

pub fn run2() {
    let filename = "src/inputs/13.txt";
    if let Ok(lines) = read_lines(filename) {
        let lines: Vec<String> = lines.flatten().collect();
        let num_lines = lines.len();

        let mut total_tokens = 0;
        let mut i = 0;
        let re1 = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
        let re2 = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();
        while i < num_lines {
            let captures1 = re1.captures(&lines[i]).unwrap();
            let x1 = captures1.get(1).map_or("", |m| m.as_str()).parse().unwrap();
            let x2 = captures1.get(2).map_or("", |m| m.as_str()).parse().unwrap();

            let captures2 = re1.captures(&lines[i + 1]).unwrap();
            let y1 = captures2.get(1).map_or("", |m| m.as_str()).parse().unwrap();
            let y2 = captures2.get(2).map_or("", |m| m.as_str()).parse().unwrap();

            let captures3 = re2.captures(&lines[i + 2]).unwrap();
            let p1: i128 = captures3.get(1).map_or("", |m| m.as_str()).parse().unwrap();
            let p2: i128 = captures3.get(2).map_or("", |m| m.as_str()).parse().unwrap();

            let machine = Machine {
                one: (x1, y1, p1 + 10000000000000),
                two: (x2, y2, p2 + 10000000000000),
            };

            let tokens = min_tokens(&machine);
            if tokens > 0 {
                total_tokens += tokens;
            }

            i += 4;
        }

        println!("Tokens: {}", total_tokens);
    }
}

fn min_tokens(machine: &Machine) -> i128 {
    let a = machine.one.0;
    let b = machine.one.1;
    let c = machine.two.0;
    let d = machine.two.1;

    let e = machine.one.2;
    let f = machine.two.2;

    let determinant = a * d - b * c;

    if determinant == 0 {
        let determinant2 = a * f - c * e;
        if determinant2 != 0 {
            return -1;
        }

        panic!("Need to solve this case");
    }

    let solution = solve(a, b, c, d, e, f);
    if solution != (-1, -1) {
        return 3 * solution.0 + solution.1;
    }

    return -1;
}

fn solve(a: i128, b: i128, c: i128, d: i128, e: i128, f: i128) -> (i128, i128) {
    println!("{}x + {}y = {}", a, b, e);
    println!("{}x + {}y = {}", c, d, f);
    let determinant = a * d - b * c;
    let numerator1 = e * d - b * f;
    let numerator2 = a * f - c * e;

    let x = numerator1 / determinant;
    let y = numerator2 / determinant;
    println!("{} {}", x, y);

    if numerator1 % determinant != 0 {
        return (-1, -1);
    }
    if x < 0 {
        return (-1, -1);
    }
    if numerator2 % determinant != 0 {
        return (-1, -1);
    }
    if y < 0 {
        return (-1, -1);
    }

    return (x, y);
}
