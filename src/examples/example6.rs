use crate::utils::utils::read_lines;
use std::{
    cmp::{max, min},
    collections::HashSet,
};

pub fn run1() {
    let filename = "src/inputs/6.txt";
    if let Ok(lines) = read_lines(filename) {
        let mut grid: Vec<Vec<String>> = Vec::new();

        let mut orientation = '^';
        let mut row = 0;
        let mut col = 0;

        let mut i = 0;
        for line in lines.flatten() {
            let mut j = 0;
            let mut row_chars: Vec<String> = Vec::new();
            for char in line.chars() {
                if char == '^' || char == '>' || char == 'v' || char == '<' {
                    row = i;
                    col = j;
                    orientation = char;
                    row_chars.push("X".to_string());
                } else {
                    row_chars.push(char.to_string());
                }

                j += 1;
            }
            grid.push(row_chars);

            i += 1;
        }

        let mut orientation = orientation.to_string();
        let mut visited: HashSet<(i32, i32, String)> = HashSet::new();
        let mut obstructions: HashSet<(i32, i32)> = HashSet::new();

        loop {
            if !step(&mut grid, &mut row, &mut col, &mut orientation) {
                break;
            }
        }

        let count = count_xs(&grid);
        println!("Count: {}", count);
    }
}

fn step(
    grid: &mut Vec<Vec<String>>,
    row: &mut i32,
    col: &mut i32,
    orientation: &mut String,
) -> bool {
    // check what's in front of you first
    let mut next_row = row.clone();
    let mut next_col = col.clone();
    if orientation == "^" {
        next_row -= 1;
    } else if orientation == ">" {
        next_col += 1;
    } else if orientation == "v" {
        next_row += 1;
    } else {
        next_col -= 1;
    }

    // if out of bounds, return false
    let num_rows = grid.len() as i32;
    let num_cols = grid[0].len() as i32;
    if next_row < 0 || next_row >= num_rows || next_col < 0 || next_col >= num_cols {
        return false;
    }

    // if obstruction, turn and return
    if grid[next_row as usize][next_col as usize] == "#" {
        if orientation == "^" {
            *orientation = ">".to_string();
        } else if orientation == ">" {
            *orientation = "v".to_string();
        } else if orientation == "v" {
            *orientation = "<".to_string();
        } else {
            *orientation = "^".to_string();
        }
        return true;
    }

    // otherwise, move and put down an X
    *row = next_row;
    *col = next_col;
    grid[next_row as usize][next_col as usize] = "X".to_string();

    return true;
}

fn count_xs(grid: &Vec<Vec<String>>) -> i32 {
    let mut count = 0;
    for row_chars in grid {
        for char in row_chars {
            if char == "X" {
                count += 1;
            }
        }
    }

    return count;
}
