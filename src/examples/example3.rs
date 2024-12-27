use regex::Regex;

use crate::utils::utils::read_lines;

pub fn run1() {
    let filename = "src/inputs/3.txt";
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    if let Ok(lines) = read_lines(filename) {
        let mut sum = 0;
        for line in lines.flatten() {
            for (_, [num1, num2]) in re.captures_iter(&line).map(|c| c.extract()) {
                sum += num1.parse::<u32>().unwrap() * num2.parse::<u32>().unwrap();
            }
        }

        println!("Sum: {}", sum);
    } else {
        println!("File not found: {}", filename);
    }
}

pub fn run2() {
    let filename = "src/inputs/3.txt";
    let do_or_dont_re = Regex::new(r"(.*?)(do\(\)|don't\(\))(.*)").unwrap();
    if let Ok(lines) = read_lines(filename) {
        let mut sum = 0;
        let mut last_op: String = "do()".to_string();
        for line in lines.flatten() {
            let mut remaining = line;
            loop {
                if let Some(capture) = do_or_dont_re.captures(&remaining.clone()) {
                    let next = capture[1].to_string();
                    if last_op == "do()" {
                        sum += score_chunk(&next);
                    }

                    remaining = capture[3].to_string();
                    last_op = capture[2].to_string();
                } else {
                    if last_op == "do()" {
                        sum += score_chunk(&remaining);
                    }
                    break;
                }
            }
        }
        println!("Sum: {}", sum);
    } else {
        println!("File not found: {}", filename);
    }
}

pub fn score_chunk(chunk: &String) -> u32 {
    let mut sum = 0;
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    for (_, [num1, num2]) in re.captures_iter(chunk).map(|c| c.extract()) {
        sum += num1.parse::<u32>().unwrap() * num2.parse::<u32>().unwrap();
    }
    return sum;
}
