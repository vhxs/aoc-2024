use core::num;

use crate::utils::utils::read_string;

pub fn run1() {
    let filename = "src/inputs/9.txt";
    if let Ok(string) = read_string(filename) {
        let mut maximum = 0;
        let expanded = expand_string(string, &mut maximum);
        let compressed = compress_string(expanded);
        let score = score_string(compressed);

        println!("Score: {}", score);
    }
}

pub fn run2() {
    let filename = "src/inputs/9.txt";
    if let Ok(string) = read_string(filename) {
        let mut maximum = 0;
        let expanded = expand_string(string, &mut maximum);
        let compressed = compress_string2(expanded, maximum);
        let score = score_string(compressed);

        println!("Score: {}", score);
    }
}

fn expand_string(string: String, maximum: &mut i32) -> Vec<String> {
    let mut is_file = true;
    let mut number = 0;
    let mut expanded: Vec<String> = Vec::new();

    for char in string.chars() {
        if is_file {
            if let Some(count) = char.to_string().parse().ok() {
                for _ in 0..count {
                    expanded.push(number.to_string());
                }
                *maximum = number;
                number += 1;
            }
        } else {
            if let Some(count) = char.to_string().parse().ok() {
                for _ in 0..count {
                    expanded.push(".".to_string());
                }
            }
        }
        is_file = !is_file;
    }
    return expanded;
}

fn compress_string(input: Vec<String>) -> Vec<String> {
    let mut sequence = input.clone();

    let mut start = 0;
    let mut end = sequence.len() - 1;

    loop {
        loop {
            if sequence[start] != "." {
                start += 1;
            } else {
                break;
            }
        }

        loop {
            if sequence[end] == "." {
                end -= 1;
            } else {
                break;
            }
        }

        if start >= end {
            break;
        }

        sequence[start] = sequence[end].clone();
        sequence[end] = ".".to_string();
    }

    return sequence;
}

fn compress_string2(input: Vec<String>, maximum: i32) -> Vec<String> {
    let mut sequence = input.clone();

    let mut number = maximum;
    let mut end = input.len() - 1;

    loop {
        loop {
            if sequence[end] != number.to_string() {
                end -= 1;
            } else {
                break;
            }
        }

        let mut start = end;
        loop {
            if start > 0 && sequence[start - 1] == number.to_string() {
                start -= 1;
            } else {
                break;
            }
        }

        let length = end - start + 1;

        let mut hole_start = 0;
        loop {
            loop {
                if hole_start < start && sequence[hole_start] != "." {
                    hole_start += 1;
                } else {
                    break;
                }
            }
            if hole_start >= start {
                break;
            }
            let mut hole_end = hole_start;
            loop {
                if hole_end + 1 < sequence.len() && sequence[hole_end + 1] == "." {
                    hole_end += 1;
                } else {
                    break;
                }
            }

            let hole_size = hole_end - hole_start + 1;
            if hole_size >= length {
                for i in 0..length {
                    sequence[hole_start + i] = sequence[start + i].to_string();
                    sequence[start + i] = ".".to_string();
                }
                break;
            } else {
                hole_start = hole_end + 1;
                if hole_end >= sequence.len() {
                    break;
                }
            }
        }

        number -= 1;
        if number == 0 {
            break;
        }
    }

    return sequence;
}

fn score_string(sequence: Vec<String>) -> i128 {
    let mut score: i128 = 0;
    let mut i: i128 = 0;
    for char in sequence {
        if char == "." {
        } else if let Some(number) = char.to_string().parse::<i128>().ok() {
            score += i * number;
        }

        i += 1;
    }

    return score;
}
