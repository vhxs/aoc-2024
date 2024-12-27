use std::collections::HashMap;

use crate::utils::utils::{read_lines, read_numbers_from_line};

pub fn run1() {
    let filename = "src/inputs/1.txt";
    if let Ok(lines) = read_lines(filename) {
        // Collect numbers into two vectors
        let mut numbers1 = vec![0; 2];
        let mut numbers2 = vec![0; 2];
        for line in lines.flatten() {
            let pair: Vec<u32> = read_numbers_from_line(line);

            if let Some(val) = pair.get(0) {
                numbers1.push(*val);
            }
            if let Some(val) = pair.get(1) {
                numbers2.push(*val);
            }
        }

        // Sort them
        numbers1.sort();
        numbers2.sort();

        // Then sum the differences
        let mut sum = 0;
        for i in 0..numbers1.len() {
            if let Some(val1) = numbers1.get(i) {
                if let Some(val2) = numbers2.get(i) {
                    sum += (*val1 as i32 - *val2 as i32).abs()
                }
            }
        }

        // Print the answer
        println!("Sum: {}", sum);
    } else {
        println!("File not found: {}", filename);
    }
}

pub fn run2() {
    let filename = "src/inputs/1.txt";
    if let Ok(lines) = read_lines(filename) {
        // Collect numbers into two vectors
        let mut numbers1 = vec![0; 2];
        let mut numbers2 = vec![0; 2];
        for line in lines.flatten() {
            let pair: Vec<u32> = line
                .split_whitespace()
                .map(|s| s.to_string().parse::<u32>().unwrap())
                .collect();

            if let Some(val) = pair.get(0) {
                numbers1.push(*val);
            }
            if let Some(val) = pair.get(1) {
                numbers2.push(*val);
            }
        }

        // Compute counts
        let mut number_counts: HashMap<u32, u32> = HashMap::new();
        for number in numbers2 {
            if let Some(count) = number_counts.get(&number) {
                number_counts.insert(number, count + 1);
            } else {
                number_counts.insert(number, 1);
            };
        }

        // Compute score
        let mut sum = 0;
        for number in numbers1 {
            if let Some(count) = number_counts.get(&number) {
                sum += number * count;
            }
        }

        println!("Score: {}", sum);
    } else {
        println!("File not found: {}", filename);
    }
}
