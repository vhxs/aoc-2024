use std::{
    fs::File,
    io::{self, BufRead, Read},
    path::Path,
    str::FromStr,
};

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_numbers_from_line<Parseable>(line: String) -> Vec<Parseable>
where
    Parseable: FromStr,
{
    return line
        .split_whitespace()
        .filter_map(|s| s.to_string().parse::<Parseable>().ok())
        .collect();
}

pub fn read_string<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let mut reader = io::BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    Ok(content)
}
