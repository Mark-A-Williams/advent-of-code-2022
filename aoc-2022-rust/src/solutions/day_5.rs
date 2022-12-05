use super::file_helpers::get_lines_from_file;

pub fn part_1() {
    let mut crate_stacks = parse_crates();

    let moves = parse_moves();

    for crate_move in moves {
        move_crates_9000(&crate_move, &mut crate_stacks)
    }

    for stack in crate_stacks.iter() {
        print!("{:}", stack.last().unwrap());
    }

    println!();
}

pub fn part_2() {
    let mut crate_stacks = parse_crates();

    let moves = parse_moves();

    for crate_move in moves {
        move_crates_9001(&crate_move, &mut crate_stacks)
    }

    for stack in crate_stacks.iter() {
        print!("{:}", stack.last().unwrap());
    }

    println!();
}

fn move_crates_9000(crate_move: &CrateMove, crate_stacks: &mut Vec<Vec<char>>) {
    let mut crates_moved = 0;
    while crates_moved < crate_move.how_many {
        let popped_crate = &crate_stacks[crate_move.from_index].pop().unwrap();
        crate_stacks[crate_move.to_index].push(popped_crate.to_owned());
        crates_moved += 1;
    }
}

fn move_crates_9001(crate_move: &CrateMove, crate_stacks: &mut Vec<Vec<char>>) {
    let from_stack = &mut crate_stacks[crate_move.from_index];
    let new_height = from_stack.len() - crate_move.how_many as usize;

    let removed_crates: Vec<char> = from_stack.drain(new_height..).collect();

    let to_stack = &mut crate_stacks[crate_move.to_index];

    for removed_crate in removed_crates {
        to_stack.push(removed_crate.to_owned());
    }
}

fn parse_crates() -> Vec<Vec<char>> {
    let max_stack_size = 8;
    let number_of_crates = 9;

    let mut crates: Vec<Vec<char>> = (0..number_of_crates).map(|_| vec![]).collect();

    let all_lines = get_lines_from_file("../inputs/5.txt");
    let crate_lines = all_lines[0..max_stack_size].iter();

    for line in crate_lines.rev() {
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

fn parse_moves() -> Vec<CrateMove> {
    let max_stack_size = 8;

    let all_lines = get_lines_from_file("../inputs/5.txt");
    let move_lines = all_lines[max_stack_size + 2..].iter();

    let mut moves: Vec<CrateMove> = vec![];

    for line in move_lines {
        let mut parts = line.split_ascii_whitespace();

        let msg = "Couldn't parse to integer";

        moves.push(CrateMove {
            how_many: parts.nth(1).unwrap().parse().expect(msg),
            from_index: parts.nth(1).unwrap().parse::<usize>().expect(msg) - 1,
            to_index: parts.nth(1).unwrap().parse::<usize>().expect(msg) - 1,
        });
    }

    moves
}

struct CrateMove {
    how_many: i32,
    from_index: usize,
    to_index: usize,
}
