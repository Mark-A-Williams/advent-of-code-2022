use file_helpers::{get_int_chunks_from_file, get_int_lines_from_file, get_lines_from_file};

mod file_helpers;

fn main() {
    day_1_part_1()
}

pub fn day_1_part_1() {
    let chunks = get_int_chunks_from_file("../inputs/1.txt");
    for chunk in &chunks {
        for line in chunk {
            print!("{line} ")
        }
        println!("\n");
    }
    println!("{}", chunks.len());
}
