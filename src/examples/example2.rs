use crate::utils::utils::{read_lines, read_numbers_from_line};

pub fn run1() {
    let filename = "src/inputs/2.txt";
    if let Ok(lines) = read_lines(filename) {
        let mut num_safe_reports = 0;
        for line in lines.flatten() {
            let numbers: Vec<i32> = read_numbers_from_line(line);
            if is_safe(&numbers) {
                num_safe_reports += 1;
            }
        }

        println!("Safe: {}", num_safe_reports);
    } else {
        println!("File not found: {}", filename);
    }
}

pub fn run2() {
    let filename = "src/inputs/2.txt";
    if let Ok(lines) = read_lines(filename) {
        let mut num_safe_reports = 0;
        for line in lines.flatten() {
            let numbers: Vec<i32> = read_numbers_from_line(line);
            if is_safe_with_tolerance(&numbers) {
                num_safe_reports += 1;
            }
        }

        println!("Safe: {}", num_safe_reports);
    } else {
        println!("File not found: {}", filename);
    }
}

fn is_safe(report: &Vec<i32>) -> bool {
    let first = report[0];
    let second = report[1];

    let correct_sign = ((second - first) as f64).signum();
    if correct_sign == 0.0 {
        return false;
    }
    for i in 1..report.len() {
        let sign = ((report[i] - report[i - 1]) as f64).signum();
        let difference = ((report[i] - report[i - 1]) as i32).abs();

        if !(sign == correct_sign && difference >= 1 && difference <= 3) {
            return false;
        }
    }

    return true;
}

fn is_safe_with_tolerance(report: &Vec<i32>) -> bool {
    if is_safe(report) {
        return true;
    } else {
        for i in 0..report.len() {
            let mut report2 = report.clone();
            report2.remove(i);
            if is_safe(&report2) {
                return true;
            }
        }
        return false;
    }
}
