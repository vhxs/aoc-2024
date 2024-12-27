use std::collections::{HashMap, HashSet};

use crate::utils::utils::read_lines;

pub fn run1() {
    let filename = "src/inputs/8.txt";
    if let Ok(lines) = read_lines(filename) {
        let mut grid: Vec<Vec<String>> = Vec::new();
        let mut antennas: HashMap<String, HashSet<(i32, i32)>> = HashMap::new();

        let mut i = 0;
        for line in lines.flatten() {
            let mut row_chars: Vec<String> = Vec::new();
            let mut j = 0;
            for char in line.chars() {
                row_chars.push(char.to_string());

                if char != '.' {
                    antennas
                        .entry(char.to_string())
                        .or_insert_with(HashSet::new)
                        .insert((i, j));
                }

                j += 1;
            }
            grid.push(row_chars);

            i += 1;
        }
        print_grid(&grid);

        let num_rows = grid.len();
        let num_cols = grid[0].len();

        let count = count_antinodes(antennas, num_rows as i32, num_cols as i32);

        println!("Count: {}", count);
    }
}

pub fn run2() {
    let filename = "src/inputs/8.txt";
    if let Ok(lines) = read_lines(filename) {
        let mut grid: Vec<Vec<String>> = Vec::new();
        let mut antennas: HashMap<String, HashSet<(i32, i32)>> = HashMap::new();

        let mut i = 0;
        for line in lines.flatten() {
            let mut row_chars: Vec<String> = Vec::new();
            let mut j = 0;
            for char in line.chars() {
                row_chars.push(char.to_string());

                if char != '.' {
                    antennas
                        .entry(char.to_string())
                        .or_insert_with(HashSet::new)
                        .insert((i, j));
                }

                j += 1;
            }
            grid.push(row_chars);

            i += 1;
        }
        print_grid(&grid);

        let num_rows = grid.len();
        let num_cols = grid[0].len();

        let count = count_antinodes2(antennas, num_rows as i32, num_cols as i32);

        println!("Count: {}", count);
    }
}

fn print_grid(grid: &Vec<Vec<String>>) {
    for row_chars in grid {
        for char in row_chars {
            print!("{}", char);
        }
        print!("\n");
    }
}

fn count_antinodes(
    antennas: HashMap<String, HashSet<(i32, i32)>>,
    num_rows: i32,
    num_cols: i32,
) -> i32 {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (_, locations) in antennas.iter() {
        for (loc1, loc2) in locations
            .iter()
            .flat_map(|a| locations.iter().map(move |b| (a, b)))
        {
            if loc1 < loc2 {
                let xdelta = loc2.0 - loc1.0;
                let ydelta = loc2.1 - loc1.1;

                let loc0 = (loc1.0 - xdelta, loc1.1 - ydelta);
                let loc3 = (loc2.0 + xdelta, loc2.1 + ydelta);

                if check_bounds(loc0, num_rows, num_cols) {
                    antinodes.insert(loc0);
                }
                if check_bounds(loc3, num_rows, num_cols) {
                    antinodes.insert(loc3);
                }
            }
        }
    }
    return antinodes.len() as i32;
}

fn count_antinodes2(
    antennas: HashMap<String, HashSet<(i32, i32)>>,
    num_rows: i32,
    num_cols: i32,
) -> i32 {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (_, locations) in antennas.iter() {
        for (loc1, loc2) in locations
            .iter()
            .flat_map(|a| locations.iter().map(move |b| (a, b)))
        {
            if loc1 < loc2 {
                let xdelta = loc2.0 - loc1.0;
                let ydelta = loc2.1 - loc1.1;

                antinodes.insert(*loc1);
                antinodes.insert(*loc2);

                let mut k = 1;
                loop {
                    let loc0 = (loc1.0 - k * xdelta, loc1.1 - k * ydelta);
                    if check_bounds(loc0, num_rows, num_cols) {
                        antinodes.insert(loc0);
                        k += 1;
                    } else {
                        break;
                    }
                }

                let mut k = 1;
                loop {
                    let loc3 = (loc2.0 + k * xdelta, loc2.1 + k * ydelta);
                    if check_bounds(loc3, num_rows, num_cols) {
                        antinodes.insert(loc3);
                        k += 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }
    return antinodes.len() as i32;
}

fn check_bounds(loc: (i32, i32), num_rows: i32, num_cols: i32) -> bool {
    if loc.0 >= 0 && loc.0 < num_rows && loc.1 >= 0 && loc.1 < num_cols {
        return true;
    }
    return false;
}
