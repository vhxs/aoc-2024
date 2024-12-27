use crate::utils::utils::read_lines;

pub fn run1() {
    let filename = "src/inputs/7.txt";
    if let Ok(lines) = read_lines(filename) {
        let mut total = 0;

        for line in lines.flatten() {
            let split = line.split(": ").collect::<Vec<&str>>();
            let target = split[0].parse::<i128>().ok();
            let numbers: Vec<i128> = split[1]
                .split(" ")
                .filter_map(|x| x.to_string().parse::<i128>().ok())
                .collect();

            if let Some(target) = target {
                let n = numbers.len() - 1;
                let total_sequences = 1 << n;

                for i in 0..total_sequences {
                    let sequence: String = (0..n)
                        .rev()
                        .map(|bit| if (i & (1 << bit)) != 0 { '*' } else { '+' })
                        .collect();

                    if check(&target, &numbers, &sequence) {
                        total += target;
                        break;
                    }
                }
            }
        }

        println!("Total: {}", total);
    }
}

pub fn run2() {
    let filename = "src/inputs/7.txt";
    if let Ok(lines) = read_lines(filename) {
        let mut total = 0;

        for line in lines.flatten() {
            let split = line.split(": ").collect::<Vec<&str>>();
            let target = split[0].parse::<i128>().ok();
            let numbers: Vec<i128> = split[1]
                .split(" ")
                .filter_map(|x| x.to_string().parse::<i128>().ok())
                .collect();

            if let Some(target) = target {
                let n = numbers.len() - 1;
                let total_sequences = usize::pow(3, n as u32);

                for i in 0..total_sequences {
                    let mut value = i;
                    let sequence: String = (0..n)
                        .rev()
                        .map(|_| {
                            let char = match value % 3 {
                                0 => '|',
                                1 => '+',
                                2 => '*',
                                _ => unreachable!(),
                            };
                            value /= 3;
                            char
                        })
                        .collect();

                    if check(&target, &numbers, &sequence) {
                        total += target;
                        break;
                    }
                }
            }
        }

        println!("Total: {}", total);
    }
}

fn check(target: &i128, numbers: &Vec<i128>, operations: &String) -> bool {
    let mut accum = numbers[0];
    let mut i = 1;
    for op in operations.chars() {
        if op == '*' {
            accum = accum * numbers[i];
        } else if op == '+' {
            accum = accum + numbers[i];
        } else {
            accum = concatenate_numbers(accum, numbers[i]);
        }

        if accum > *target {
            return false;
        }

        i += 1;
    }

    if accum == *target {
        return true;
    }
    return false;
}

fn concatenate_numbers(a: i128, b: i128) -> i128 {
    let concatenated = format!("{}{}", a, b);
    concatenated.parse::<i128>().unwrap()
}
