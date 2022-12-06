use super::file_helpers::get_lines_from_file;

pub fn part_1() {
    find_index_at_start_of_message(4);
}

pub fn part_2() {
    find_index_at_start_of_message(14);
}

fn find_index_at_start_of_message(marker_length: usize) {
    let lines = get_lines_from_file("../inputs/6.txt");

    let input: &String = &lines[0];
    let datastream: Vec<char> = input.chars().collect();

    let mut index = 0;
    while index <= datastream.len() {
        if index < marker_length {
            index += 1;
            continue;
        }

        let last_4_chars: Vec<char> = datastream[index - marker_length..index].to_vec();

        if values_are_unique(last_4_chars) {
            break;
        }

        index += 1;
    }

    println!("{}", index)
}

fn values_are_unique(values: Vec<char>) -> bool {
    for element in &values {
        if values.iter().filter(|x| x == &element).count() > 1 {
            return false;
        }
    }

    true
}
