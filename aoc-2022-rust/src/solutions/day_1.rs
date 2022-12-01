use crate::solutions::file_helpers::get_int_chunks_from_file;

pub fn day_1_part_1() {
    let elves = get_int_chunks_from_file("../inputs/1.txt");

    let elf_totals: Vec<i32> = elves.iter().map(|elf| elf.iter().sum::<i32>()).collect();
    println!("{:?}", elf_totals.iter().max());
}

pub fn day_1_part_2() {
    let elves = get_int_chunks_from_file("../inputs/1.txt");

    let mut elf_totals: Vec<i32> = elves.iter().map(|elf| elf.iter().sum::<i32>()).collect();
    elf_totals.sort();
    elf_totals.reverse();

    let top_3_total = elf_totals[0] + elf_totals[1] + elf_totals[2];
    println!("{:?}", top_3_total)
}