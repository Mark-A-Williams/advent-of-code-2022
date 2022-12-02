use crate::solutions::file_helpers::get_lines_from_file;

pub fn part_1() {
    let rounds = get_lines_from_file("../inputs/2.txt");
    let mut score: i32 = 0;

    for round in rounds {
        let parsed: Vec<&str> = round.split(' ').collect();
        let opponent_choice = parsed[0];
        let my_choice = parsed[1];

        match my_choice {
            "X" => score += 1,
            "Y" => score += 2,
            "Z" => score += 3,
            _ => panic!("Unexpected choices"),
        }

        let round_result = get_round_result(opponent_choice, my_choice);

        match round_result {
            RoundResult::Draw => score += 3,
            RoundResult::Win => score += 6,
            _ => (),
        }
    }

    println!("{}", score)
}

pub fn part_2() {}

fn get_round_result(opponent_choice: &str, my_choice: &str) -> RoundResult {
    match (opponent_choice, my_choice) {
        ("A", "X") => RoundResult::Draw,
        ("B", "X") => RoundResult::Loss,
        ("C", "X") => RoundResult::Win,
        ("A", "Y") => RoundResult::Win,
        ("B", "Y") => RoundResult::Draw,
        ("C", "Y") => RoundResult::Loss,
        ("A", "Z") => RoundResult::Loss,
        ("B", "Z") => RoundResult::Win,
        ("C", "Z") => RoundResult::Draw,
        (_, _) => panic!("Unexpected choices"),
    }
}

enum RoundResult {
    Win,
    Loss,
    Draw,
}
