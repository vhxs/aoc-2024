use crate::utils::utils::read_string;

pub fn run1() {
    let filename = "src/inputs/11.txt";
    if let Ok(string) = read_string(filename) {
        let numbers: Vec<i128> = string
            .split(" ")
            .filter_map(|x| x.to_string().parse::<i128>().ok())
            .collect();

        let mut new_numbers = numbers.clone();
        for i in 0..75 {
            new_numbers = blink(new_numbers);
            println!("Count: {} {}", i, new_numbers.len());
        }
    }
}

fn blink(old_state: Vec<i128>) -> Vec<i128> {
    let mut new_state: Vec<i128> = Vec::new();
    for number in old_state {
        if number == 0 {
            new_state.push(1);
        } else if number.to_string().len() % 2 == 0 {
            let number_string = number.to_string();
            let length = number_string.len() / 2;

            let left_number = &number_string[..length].parse::<i128>().ok();
            if let Some(left_number) = left_number {
                new_state.push(*left_number);
            }
            let right_number = &number_string[length..].parse::<i128>().ok();
            if let Some(right_number) = right_number {
                new_state.push(*right_number);
            }
        } else {
            new_state.push(number * 2024);
        }
    }

    return new_state;
}
