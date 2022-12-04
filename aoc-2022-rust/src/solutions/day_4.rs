use super::file_helpers::get_lines_from_file;

pub fn part_1() {
    let elf_pairs = get_lines_from_file("../inputs/4.txt");
    let mut total = 0;

    for elf_pair in elf_pairs {
        let elf_ranges = parse_line_to_elf_ranges(elf_pair);

        if ranges_overlap_fully(&elf_ranges[0], &elf_ranges[1]) {
            total += 1;
        }
    }

    println!("{}", total);
}

pub fn part_2() {
    let elf_pairs = get_lines_from_file("../inputs/4.txt");
    let mut total = 0;

    for elf_pair in elf_pairs {
        let elf_ranges = parse_line_to_elf_ranges(elf_pair);

        if ranges_overlap_partially(&elf_ranges[0], &elf_ranges[1]) {
            total += 1;
        }
    }

    println!("{}", total);
}

fn parse_line_to_elf_ranges(line: String) -> Vec<ElfRange> {
    let elves = line.split(",");

    elves.map(|elf| parse_elf(elf)).collect()
}

fn ranges_overlap_fully(range_1: &ElfRange, range_2: &ElfRange) -> bool {
    if range_1.min <= range_2.min && range_1.max >= range_2.max {
        true
    } else if range_2.min <= range_1.min && range_2.max >= range_1.max {
        true
    } else {
        false
    }
}

fn ranges_overlap_partially(range_1: &ElfRange, range_2: &ElfRange) -> bool {
    if range_1.max < range_2.min || range_2.max < range_1.min {
        false
    } else {
        true
    }
}

fn parse_elf(elf: &str) -> ElfRange {
    let range_bounds: Vec<i32> = elf
        .split("-")
        .map(|bound| bound.parse::<i32>().expect("Couldn't parse i32"))
        .collect();

    ElfRange {
        min: range_bounds[0],
        max: range_bounds[1],
    }
}

struct ElfRange {
    min: i32,
    max: i32,
}
