use std::{
    cmp::{max, min},
    time,
};

use regex::Regex;

use crate::utils::utils::read_lines;

pub fn run1() {
    let filename = "src/inputs/14.txt";
    if let Ok(lines) = read_lines(filename) {
        let lines: Vec<String> = lines.flatten().collect();
        let num_lines = lines.len();

        let mut top_left = 0;
        let mut top_right = 0;
        let mut bottom_left = 0;
        let mut bottom_right = 0;

        let mut i = 0;
        let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
        while i < num_lines {
            let captures = re.captures(&lines[i]).unwrap();

            let p1: i32 = captures[1].parse().unwrap();
            let p2: i32 = captures[2].parse().unwrap();
            let v1: i32 = captures[3].parse().unwrap();
            let v2: i32 = captures[4].parse().unwrap();

            let mut x1 = p1;
            let mut x2 = p2;
            for _ in 0..100 {
                x1 += v1;
                x2 += v2;

                if x1 < 0 {
                    x1 += 101;
                }
                if x2 < 0 {
                    x2 += 103;
                }

                x1 = x1 % 101;
                x2 = x2 % 103;
            }

            if x1 < 50 && x2 < 51 {
                top_left += 1;
            }
            if x1 > 50 && x2 < 51 {
                top_right += 1;
            }
            if x1 < 50 && x2 > 51 {
                bottom_left += 1;
            }
            if x1 > 50 && x2 > 51 {
                bottom_right += 1;
            }

            i += 1;
        }

        println!(
            "{} {} {} {}",
            top_left, top_right, bottom_left, bottom_right
        );
        let factor = top_left * top_right * bottom_left * bottom_right;
        println!("{}", factor);
    }
}

pub fn run2() {
    let filename = "src/inputs/14.txt";
    let ten_millis = time::Duration::from_millis(250);

    if let Ok(lines) = read_lines(filename) {
        let lines: Vec<String> = lines.flatten().collect();
        let num_lines = lines.len();

        let mut i = 0;
        let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
        let mut robot_pos: Vec<(i32, i32)> = Vec::new();
        let mut robot_vel: Vec<(i32, i32)> = Vec::new();

        while i < num_lines {
            let captures = re.captures(&lines[i]).unwrap();

            let p1: i32 = captures[1].parse().unwrap();
            let p2: i32 = captures[2].parse().unwrap();
            let v1: i32 = captures[3].parse().unwrap();
            let v2: i32 = captures[4].parse().unwrap();

            robot_pos.push((p1, p2));
            robot_vel.push((v1, v2));

            i += 1;
        }

        for step in 1..20000 {
            for i in 0..num_lines {
                let (mut x1, mut x2) = robot_pos[i];
                let (v1, v2) = robot_vel[i];

                x1 += v1;
                x2 += v2;

                if x1 < 0 {
                    x1 += 101;
                }
                if x2 < 0 {
                    x2 += 103;
                }

                x1 = x1 % 101;
                x2 = x2 % 103;

                robot_pos[i] = (x1, x2);
            }

            if check_tree(&robot_pos) {
                println!("Steps: {}", step);
            };

            // println!();
        }
    }
}

fn check_tree(robot_pos: &Vec<(i32, i32)>) -> bool {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for _ in 0..101 {
        let mut row: Vec<char> = Vec::new();
        for _ in 0..103 {
            row.push('.');
        }
        grid.push(row);
    }

    for pos in robot_pos {
        let (x, y) = pos;
        grid[*x as usize][*y as usize] = 'X';
    }

    let num_rows = grid.len() as i32;
    let num_cols = grid[0].len() as i32;

    let mut count2 = 0;
    for (i, j) in robot_pos {
        let mut count = 0;
        for x in max(0, i - 2)..min(i + 2, num_rows) {
            for y in max(0, j - 2)..min(j + 2, num_cols) {
                if grid[x as usize][y as usize] == 'X' {
                    count += 1
                }
            }
        }
        if count >= 3 {
            count2 += 1;
        }
    }
    if count2 >= 250 {
        print_grid(grid);
        return true;
    }

    return false;
}

fn print_grid(grid: Vec<Vec<char>>) {
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    for i in 0..num_rows {
        for j in 0..num_cols {
            print!("{}", grid[i][j]);
        }
        println!();
    }
}
