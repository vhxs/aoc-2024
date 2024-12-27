use core::num;
use std::collections::{HashMap, HashSet};

use crate::utils::utils::read_lines;

type Node = (usize, usize);

struct Graph {
    nodes: HashSet<Node>,
    node_labels: HashMap<Node, String>,
    edges: HashMap<Node, Vec<Node>>,
}

impl Graph {
    fn add_node(&mut self, node: Node, label: String) {
        self.nodes.insert(node.clone());
        self.node_labels.insert(node, label);
    }

    fn add_edges(&mut self, node: Node, incident: Vec<Node>) {
        self.edges.insert(node, incident);
    }

    fn new() -> Graph {
        return Graph {
            nodes: HashSet::new(),
            node_labels: HashMap::new(),
            edges: HashMap::new(),
        };
    }

    fn components(&self) -> Vec<HashSet<Node>> {
        let mut remaining_nodes = self.nodes.clone();
        let mut components: Vec<HashSet<Node>> = Vec::new();
        let mut removed: HashSet<Node> = HashSet::new();

        for node in remaining_nodes.drain() {
            if removed.contains(&node) {
                continue;
            }

            let mut stack: Vec<Node> = Vec::new();
            stack.push(node);

            let mut component: HashSet<Node> = HashSet::new();

            while stack.len() > 0 {
                let next_node = stack.pop();
                if let Some(next_node) = next_node {
                    if !component.contains(&next_node) {
                        for adjacent_node in self.edges[&next_node].clone() {
                            // println!(
                            //     // "{} {} is adjacent to {} {}",
                            //     // adjacent_node.0,
                            //     // adjacent_node.1, next_node.0, next_node.1
                            // );
                            stack.push(adjacent_node);
                        }
                    }

                    component.insert(next_node.clone());
                    removed.insert(next_node);
                }
            }

            components.push(component);
        }

        return components;
    }
}

pub fn run1() {
    let filename = "src/inputs/12.txt";
    if let Ok(lines) = read_lines(filename) {
        let mut grid: Vec<Vec<String>> = Vec::new();

        for line in lines.flatten() {
            let mut row_chars: Vec<String> = Vec::new();
            for char in line.chars() {
                row_chars.push(char.to_string());
            }
            grid.push(row_chars);
        }

        let graph = graph_from_grid(&grid);
        let components = graph.components();

        let mut total = 0;
        let num_rows = grid.len();
        let num_cols = grid[0].len();
        for component in components {
            total += price(&component, num_rows as i32, num_cols as i32);
        }

        println!("Price: {}", total);
    }
}

pub fn run2() {
    let filename = "src/inputs/12.txt";
    if let Ok(lines) = read_lines(filename) {
        let mut grid: Vec<Vec<String>> = Vec::new();

        for line in lines.flatten() {
            let mut row_chars: Vec<String> = Vec::new();
            for char in line.chars() {
                row_chars.push(char.to_string());
            }
            grid.push(row_chars);
        }

        let graph = graph_from_grid(&grid);
        let components = graph.components();

        let mut total = 0;
        let num_rows = grid.len();
        let num_cols = grid[0].len();
        for component in components {
            total += price2(&component, num_rows as i32, num_cols as i32);
        }

        println!("Price: {}", total);
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

fn graph_from_grid(grid: &Vec<Vec<String>>) -> Graph {
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut graph = Graph::new();
    for i in 0..num_rows {
        for j in 0..num_cols {
            graph.add_node((i, j), grid[i][j].clone());
            let mut incident: Vec<Node> = Vec::new();
            if i > 0 && grid[i][j] == grid[i - 1][j] {
                incident.push((i - 1, j));
            }
            if i < num_rows - 1 && grid[i][j] == grid[i + 1][j] {
                incident.push((i + 1, j));
            }
            if j < num_cols - 1 && grid[i][j] == grid[i][j + 1] {
                incident.push((i, j + 1));
            }
            if j > 0 && grid[i][j] == grid[i][j - 1] {
                incident.push((i, j - 1));
            }
            graph.add_edges((i, j), incident);
        }
    }

    return graph;
}

fn perimeter(component: &HashSet<Node>, num_rows: i32, num_cols: i32) -> i32 {
    let mut count = 0;
    for node in component {
        let i = node.0;
        let j = node.1;

        if i == 0 || !component.contains(&(i - 1, j)) {
            count += 1;
        }
        if i >= num_rows as usize || !component.contains(&(i + 1, j)) {
            count += 1;
        }
        if j >= num_cols as usize || !component.contains(&(i, j + 1)) {
            count += 1;
        }
        if j == 0 || !component.contains(&(i, j - 1)) {
            count += 1;
        }
    }

    return count;
}

fn count_corners(component: &HashSet<Node>, num_rows: i32, num_cols: i32) -> i32 {
    let mut count = 0;
    for node in component {
        let i = node.0;
        let j = node.1;

        let top = i > 0 && component.contains(&(i - 1, j));
        let bottom = i < num_rows as usize - 1 && component.contains(&(i + 1, j));
        let left = j > 0 && component.contains(&(i, j - 1));
        let right = j < num_cols as usize - 1 && component.contains(&(i, j + 1));

        let top_left = i > 0 && j > 0 && component.contains(&(i - 1, j - 1));
        let top_right = i > 0 && j < num_cols as usize - 1 && component.contains(&(i - 1, j + 1));
        let bottom_left = i < num_rows as usize - 1 && j > 0 && component.contains(&(i + 1, j - 1));
        let bottom_right = i < num_rows as usize - 1
            && j < num_cols as usize - 1
            && component.contains(&(i + 1, j + 1));

        if (!top && !left) || (top && left && !top_left) {
            count += 1;
        }
        if (!top && !right) || (top && right && !top_right) {
            count += 1;
        }
        if (!bottom && !left) || (bottom && left && !bottom_left) {
            count += 1;
        }
        if (!bottom && !right) || (bottom && right && !bottom_right) {
            count += 1;
        }
    }
    return count;
}

fn area(component: &HashSet<Node>) -> i32 {
    return component.len() as i32;
}

fn price(component: &HashSet<Node>, num_rows: i32, num_cols: i32) -> i32 {
    let perimeter = perimeter(component, num_rows, num_cols);
    let area = area(component);

    return area * perimeter;
}

fn price2(component: &HashSet<Node>, num_rows: i32, num_cols: i32) -> i32 {
    let corners = count_corners(component, num_rows, num_cols);
    let area = area(component);

    println!("Area: {}, Corners {}", area, corners);

    return area * corners;
}
