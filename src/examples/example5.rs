use std::{cmp::Ordering, collections::HashSet};

use crate::utils::utils::read_lines;

pub fn run1() {
    let filename = "src/inputs/5.txt";
    if let Ok(lines) = read_lines(filename) {
        let mut state = "poset";
        let mut pairs: HashSet<(i32, i32)> = HashSet::new();
        let mut lists: Vec<Vec<i32>> = Vec::new();

        let mut total = 0;

        for line in lines.flatten() {
            if line == "" {
                state = "lists";
                continue;
            }

            if state == "poset" {
                let pair_as_array: Vec<&str> = line.split("|").collect();
                if let Some(first) = pair_as_array[0].parse::<i32>().ok() {
                    if let Some(second) = pair_as_array[1].parse::<i32>().ok() {
                        pairs.insert((first, second));
                    }
                }
            }

            if state == "lists" {
                let list: Vec<i32> = line
                    .split(",")
                    .filter_map(|x| x.to_string().parse::<i32>().ok())
                    .collect();
                lists.push(list);
            }
        }

        for list in lists {
            if check_list1(&pairs, &list) {
                let mid = (list.len() - 1) / 2;
                total += list[mid];
            }
        }

        println!("{}", total);
    }
}

pub fn check_list1(pairs: &HashSet<(i32, i32)>, list: &Vec<i32>) -> bool {
    for i in 0..list.len() {
        for j in i..list.len() {
            let first = list[i];
            let second = list[j];
            if pairs.contains(&(second, first)) {
                return false;
            }
        }
    }
    return true;
}

pub fn run2() {
    let filename = "src/inputs/5.txt";
    if let Ok(lines) = read_lines(filename) {
        let mut state = "poset";
        let mut lists: Vec<Vec<i32>> = Vec::new();
        let mut pairs: HashSet<(i32, i32)> = HashSet::new();
        let mut total = 0;

        for line in lines.flatten() {
            if line == "" {
                state = "lists";
                continue;
            }

            if state == "poset" {
                let pair_as_array: Vec<&str> = line.split("|").collect();
                if let Some(first) = pair_as_array[0].parse::<i32>().ok() {
                    if let Some(second) = pair_as_array[1].parse::<i32>().ok() {
                        pairs.insert((first, second));
                    }
                }
            }

            if state == "lists" {
                let list: Vec<i32> = line
                    .split(",")
                    .filter_map(|x| x.to_string().parse::<i32>().ok())
                    .collect();
                lists.push(list);
            }
        }

        for list in lists {
            let mut list = list;
            if !check_list1(&pairs, &list) {
                list.sort_by(|a, b| {
                    if pairs.contains(&(*a, *b)) {
                        Ordering::Less // `a` comes before `b`
                    } else if pairs.contains(&(*b, *a)) {
                        Ordering::Greater // `b` comes before `a`
                    } else {
                        Ordering::Equal // No defined order, treat as equal
                    }
                });
                let mid = (list.len() - 1) / 2;
                total += list[mid];
            }
        }

        println!("{}", total);
    }
}
