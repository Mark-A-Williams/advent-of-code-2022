use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn get_lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn get_int_lines_from_file(filename: impl AsRef<Path>) -> Vec<i32> {
    let string_lines = get_lines_from_file(filename);
    string_lines
        .iter()
        .map(|l| str::parse::<i32>(&l).expect("Could not parse to int"))
        .collect()
}
