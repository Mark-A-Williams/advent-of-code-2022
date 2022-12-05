use super::file_helpers::get_lines_from_file;

pub fn part_1() {
    let crates = parse_crates();
    for ccrate in crates.iter() {
        println!("{:?}", ccrate);
    }
}

pub fn part_2() {
    todo!()
}

fn parse_crates() -> Vec<Vec<char>> {
    let mut crates: Vec<Vec<char>> = (0..9).map(|_| vec![]).collect();

    let all_lines = get_lines_from_file("../inputs/5.txt");
    let first_few_lines = all_lines[0..8].iter();

    for line in first_few_lines.rev() {
        let character_enumerator = line.chars().skip(1).step_by(4).enumerate();
        for (index, character) in character_enumerator {
            if character.is_ascii_uppercase() {
                let relevant_crate = &mut crates[index];
                relevant_crate.push(character);
            }
        }
    }

    crates
}
