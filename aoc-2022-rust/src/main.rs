use file_helpers::{get_int_chunks_from_file, get_int_lines_from_file, get_lines_from_file};

mod file_helpers;

fn main() {
    day_1_part_1()
}

pub fn day_1_part_1() {
    let elves = get_int_chunks_from_file("../inputs/1.txt");

    let elf_totals: Vec<i32> = elves.iter().map(|elf| elf.iter().sum::<i32>()).collect();
    println!("{:?}", elf_totals.iter().max());
}
