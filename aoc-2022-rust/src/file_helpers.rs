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

pub fn get_int_chunks_from_file(filename: impl AsRef<Path>) -> Vec<Vec<i32>> {
    let all_lines = get_lines_from_file(filename);
    let mut result: Vec<Vec<i32>> = vec![];
    let mut current_chunk: Vec<i32> = vec![];

    for line in all_lines {
        if line.is_empty() {
            let clone = current_chunk.clone();
            result.push(clone);
            current_chunk = vec![];
        } else {
            current_chunk.push(str::parse::<i32>(&line).expect("Could not parse to int"));
        }
    }

    if current_chunk.len() > 0 {
        result.push(current_chunk);
    }

    result
}
