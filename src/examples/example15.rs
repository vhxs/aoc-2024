use crate::utils::utils::read_lines;

pub fn run1() {
    let filename = "src/inputs/15.txt";
    if let Ok(lines) = read_lines(filename) {
        let mut grid: Vec<Vec<String>> = Vec::new();
        let lines: Vec<String> = lines.flatten().collect();

        let mut pos: (i32, i32) = (0, 0);
        let mut i = 0;
        loop {
            if lines[i] == "" {
                i += 1;
                break;
            }
            let mut j = 0;
            let mut row_chars: Vec<String> = Vec::new();
            for char in lines[i].chars() {
                if char == '@' {
                    pos = (i as i32, j);
                }
                row_chars.push(char.to_string());
                j += 1;
            }
            grid.push(row_chars);
            i += 1;
        }

        let chars: Vec<char> = lines[i..]
            .into_iter()
            .map(|line| line.chars())
            .flatten()
            .collect();

        for char in chars {
            pos = step(&mut grid, pos, char);
        }

        let score = score_grid(&grid);

        println!("Score: {}", score);
    }
}

fn step(grid: &mut Vec<Vec<String>>, pos: (i32, i32), direction: char) -> (i32, i32) {
    let delta = if direction == '^' {
        (-1, 0)
    } else if direction == '>' {
        (0, 1)
    } else if direction == 'v' {
        (1, 0)
    } else {
        (0, -1)
    };

    let next_pos = (pos.0 + delta.0, pos.1 + delta.1);
    if grid[next_pos.0 as usize][next_pos.1 as usize] == "." {
        grid[next_pos.0 as usize][next_pos.1 as usize] = "@".to_string();
        grid[pos.0 as usize][pos.1 as usize] = ".".to_string();
        return next_pos;
    }

    if grid[next_pos.0 as usize][next_pos.1 as usize] == "#".to_string() {
        return pos;
    }

    let mut future_pos = next_pos;
    let can_move = loop {
        future_pos.0 += delta.0;
        future_pos.1 += delta.1;

        if grid[future_pos.0 as usize][future_pos.1 as usize] == "#".to_string() {
            break false;
        } else if grid[future_pos.0 as usize][future_pos.1 as usize] == ".".to_string() {
            break true;
        }
    };
    if can_move {
        grid[future_pos.0 as usize][future_pos.1 as usize] = "O".to_string();
        grid[next_pos.0 as usize][next_pos.1 as usize] = "@".to_string();
        grid[pos.0 as usize][pos.1 as usize] = ".".to_string();

        return next_pos;
    }

    return pos;
}

fn score_grid(grid: &Vec<Vec<String>>) -> i32 {
    let score = grid
        .into_iter()
        .enumerate()
        .map(|(i, row)| {
            row.into_iter()
                .enumerate()
                .map(|(j, c)| if c == "O" { 100 * i + j } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>();

    score as i32
}
