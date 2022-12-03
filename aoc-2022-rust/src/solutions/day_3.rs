use std::fmt::Error;

use super::file_helpers::get_lines_from_file;

pub fn part_1() {
    let rucksacks = get_lines_from_file("../inputs/3.txt");
    let mut total_priority = 0;

    for rucksack in rucksacks {
        let all_items: Vec<char> = rucksack.chars().collect();

        let (first_compartment, second_compartment) = all_items.split_at(all_items.len() / 2);
        let item_in_common = get_item_in_common(first_compartment, second_compartment);

        total_priority += get_priority_of_item(item_in_common);
    }

    println!("{}", total_priority);
}

pub fn part_2() {
    todo!();
}

fn get_item_in_common(first_compartment: &[char], second_compartment: &[char]) -> char {
    for item in first_compartment {
        if second_compartment.contains(item) {
            return item.to_owned();
        }
    }

    panic!("No item in common")
}

fn get_priority_of_item(item: char) -> u32 {
    let ascii_code = item as u32;
    if item.is_ascii_uppercase() {
        ascii_code - 38
    } else if item.is_ascii_lowercase() {
        ascii_code - 96
    } else {
        panic!("Couldn't parse to ASCII character");
    }
}
