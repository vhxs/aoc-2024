use regex::Regex;

use crate::utils::utils::read_lines;

pub fn run1() {
    let filename = "src/inputs/4.txt";
    if let Ok(lines) = read_lines(filename) {
        let mut letters: Vec<Vec<String>> = Vec::new();
        for line in lines.flatten() {
            let mut row: Vec<String> = Vec::new();

            for char in line.chars() {
                row.push(char.to_string());
            }
            letters.push(row);
        }

        let num_rows = letters.len();
        let num_cols = letters[0].len();

        let mut count = 0;

        // rows
        for i in 0..num_rows {
            let mut string = "".to_string();
            for j in 0..num_cols {
                string += &letters[i][j];
            }
            count += xmas_count(&string);
        }

        // columns
        for j in 0..num_cols {
            let mut string = "".to_string();
            for i in 0..num_rows {
                string += &letters[i][j];
            }
            count += xmas_count(&string);
        }

        // diagonal
        for row in 0..num_rows {
            let mut i = row;
            let mut j = 0;
            let mut string = "".to_string();
            loop {
                string += &letters[i][j];

                i += 1;
                j += 1;
                if i >= num_rows || j >= num_cols {
                    break;
                }
            }
            count += xmas_count(&string);
        }

        for col in 1..num_cols {
            let mut i = 0;
            let mut j = col;
            let mut string = "".to_string();
            loop {
                string += &letters[i][j];

                i += 1;
                j += 1;
                if i >= num_rows || j >= num_cols {
                    break;
                }
            }
            count += xmas_count(&string);
        }

        // anti
        for row in 0..num_rows {
            let mut i: i32 = row as i32;
            let mut j = 0;
            let mut string = "".to_string();
            loop {
                string += &letters[i as usize][j];

                i -= 1;
                j += 1;
                if i < 0 || j >= num_cols {
                    break;
                }
            }
            count += xmas_count(&string);
        }

        for col in 1..num_cols {
            let mut i: i32 = (num_rows - 1) as i32;
            let mut j = col;
            let mut string = "".to_string();
            loop {
                string += &letters[i as usize][j];

                i -= 1;
                j += 1;
                if i < 0 || j >= num_cols {
                    break;
                }
            }
            count += xmas_count(&string);
        }

        println!("{}", &count);
    } else {
        println!("File not found: {}", filename);
    }
}

pub fn xmas_count(string: &String) -> u32 {
    let re1 = Regex::new(r"XMAS").unwrap();
    let re2 = Regex::new(r"SAMX").unwrap();

    let forwards: Vec<&str> = re1.find_iter(string).map(|m| m.as_str()).collect();
    let backwards: Vec<&str> = re2.find_iter(string).map(|m| m.as_str()).collect();

    return forwards.len() as u32 + backwards.len() as u32;
}

pub fn run2() {
    let filename = "src/inputs/4.txt";
    let mut letters: Vec<Vec<String>> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let mut row: Vec<String> = Vec::new();

            for char in line.chars() {
                row.push(char.to_string());
            }
            letters.push(row);
        }

        let num_rows = letters.len();
        let num_cols = letters[0].len();

        let mut total = 0;

        for i in 0..num_rows - 2 {
            for j in 0..num_cols - 2 {
                if check_is_x_mas(i, j, &letters) {
                    total += 1;
                }
            }
        }

        println!("Total: {}", total);
    } else {
        println!("File not found: {}", filename);
    }
}

fn check_is_x_mas(i: usize, j: usize, matrix: &Vec<Vec<String>>) -> bool {
    let diagonal = "".to_string() + &matrix[i][j] + &matrix[i + 1][j + 1] + &matrix[i + 2][j + 2];
    let anti = "".to_string() + &matrix[i][j + 2] + &matrix[i + 1][j + 1] + &matrix[i + 2][j];

    if diagonal == "MAS" || diagonal == "SAM" {
        if anti == "MAS" || anti == "SAM" {
            return true;
        }
    }

    return false;
}
