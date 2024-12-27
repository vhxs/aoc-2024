use std::collections::HashSet;

use crate::utils::utils::read_lines;

pub fn run1() {
    let filename = "src/inputs/10.txt";
    if let Ok(lines) = read_lines(filename) {
        let mut grid: Vec<Vec<i32>> = Vec::new();

        for line in lines.flatten() {
            let mut row_chars: Vec<i32> = Vec::new();
            for char in line.chars() {
                if let Some(number) = char.to_string().parse().ok() {
                    row_chars.push(number);
                }
            }
            grid.push(row_chars);
        }

        let mut count = 0;

        let num_rows = grid.len();
        let num_cols = grid[0].len();
        for i in 0..num_rows {
            for j in 0..num_cols {
                if grid[i][j] == 0 {
                    let c = count_paths(i as i32, j as i32, &grid);
                    count += c.len();
                }
            }
        }

        println!("Count: {}", count);
    }
}

pub fn run2() {
    let filename = "src/inputs/10.txt";
    if let Ok(lines) = read_lines(filename) {
        let mut grid: Vec<Vec<i32>> = Vec::new();

        for line in lines.flatten() {
            let mut row_chars: Vec<i32> = Vec::new();
            for char in line.chars() {
                if let Some(number) = char.to_string().parse().ok() {
                    row_chars.push(number);
                }
            }
            grid.push(row_chars);
        }

        let mut count = 0;

        let num_rows = grid.len();
        let num_cols = grid[0].len();
        for i in 0..num_rows {
            for j in 0..num_cols {
                if grid[i][j] == 0 {
                    let c = count_paths2(i as i32, j as i32, &grid);
                    count += c;
                }
            }
        }

        println!("Count: {}", count);
    }
}

fn count_paths(i: i32, j: i32, grid: &Vec<Vec<i32>>) -> HashSet<(i32, i32)> {
    let current = grid[i as usize][j as usize];
    if current == 9 {
        let mut end = HashSet::new();
        end.insert((i, j));
        return end;
    }

    let mut ends = HashSet::new();

    // up
    if (i - 1) >= 0 && grid[i as usize - 1][j as usize] == current + 1 {
        ends.extend(count_paths(i - 1, j, grid));
    }
    // down
    if (i + 1) < grid.len() as i32 && grid[i as usize + 1][j as usize] == current + 1 {
        ends.extend(count_paths(i + 1, j, grid));
    }
    // left
    if (j - 1) >= 0 && grid[i as usize][j as usize - 1] == current + 1 {
        ends.extend(count_paths(i, j - 1, grid));
    }
    // right
    if (j + 1) < grid.len() as i32 && grid[i as usize][j as usize + 1] == current + 1 {
        ends.extend(count_paths(i, j + 1, grid));
    }

    return ends;
}

fn count_paths2(i: i32, j: i32, grid: &Vec<Vec<i32>>) -> i32 {
    let current = grid[i as usize][j as usize];
    if current == 9 {
        return 1;
    }

    let mut count = 0;

    // up
    if (i - 1) >= 0 && grid[i as usize - 1][j as usize] == current + 1 {
        count += count_paths2(i - 1, j, grid);
    }
    // down
    if (i + 1) < grid.len() as i32 && grid[i as usize + 1][j as usize] == current + 1 {
        count += count_paths2(i + 1, j, grid);
    }
    // left
    if (j - 1) >= 0 && grid[i as usize][j as usize - 1] == current + 1 {
        count += count_paths2(i, j - 1, grid);
    }
    // right
    if (j + 1) < grid.len() as i32 && grid[i as usize][j as usize + 1] == current + 1 {
        count += count_paths2(i, j + 1, grid);
    }

    return count;
}
