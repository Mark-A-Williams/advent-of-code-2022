use file_helpers::{get_int_lines_from_file, get_lines_from_file};

mod file_helpers;

fn main() {
    let lines = get_lines_from_file("../inputs/words.txt");
    for line in lines {
        println!("{:?}", line);
    }

    let lines = get_int_lines_from_file("../inputs/numbers.txt");
    for line in lines {
        println!("{:?}", line);
    }
}
