use crate::solutions::file_helpers::get_lines_from_file;

pub fn part_1() {
    let rounds = get_lines_from_file("../inputs/2.txt");
    let mut score: i32 = 0;

    for round in rounds {
        let parsed: Vec<&str> = round.split(' ').collect();
        let opponent_choice = parse_opponent_play(parsed[0]);

        let my_choice = match parsed[1] {
            "X" => Choice::Rock,
            "Y" => Choice::Paper,
            "Z" => Choice::Scissors,
            _ => panic!("Unexpected choice"),
        };

        score += get_round_score(opponent_choice, my_choice);
    }

    println!("{}", score)
}

pub fn part_2() {
    let rounds = get_lines_from_file("../inputs/2.txt");
    let mut score: i32 = 0;

    for round in rounds {
        let parsed: Vec<&str> = round.split(' ').collect();
        let opponent_choice = parse_opponent_play(parsed[0]);

        let desired_result = match parsed[1] {
            "X" => RoundResult::Loss,
            "Y" => RoundResult::Draw,
            "Z" => RoundResult::Win,
            _ => panic!("Unexpected result"),
        };

        score += get_round_score_from_desired_result(opponent_choice, desired_result);
    }

    println!("{}", score)
}

fn get_round_score_from_desired_result(
    opponent_choice: Choice,
    desired_result: RoundResult,
) -> i32 {
    match (desired_result, opponent_choice) {
        (RoundResult::Win, Choice::Rock) => 6 + 2,
        (RoundResult::Win, Choice::Paper) => 6 + 3,
        (RoundResult::Win, Choice::Scissors) => 6 + 1,
        (RoundResult::Loss, Choice::Rock) => 0 + 3,
        (RoundResult::Loss, Choice::Paper) => 0 + 1,
        (RoundResult::Loss, Choice::Scissors) => 0 + 2,
        (RoundResult::Draw, Choice::Rock) => 3 + 1,
        (RoundResult::Draw, Choice::Paper) => 3 + 2,
        (RoundResult::Draw, Choice::Scissors) => 3 + 3,
    }
}

fn get_round_score(opponent_choice: Choice, my_choice: Choice) -> i32 {
    match (opponent_choice, my_choice) {
        (Choice::Rock, Choice::Rock) => 3 + 1,
        (Choice::Paper, Choice::Rock) => 0 + 1,
        (Choice::Scissors, Choice::Rock) => 6 + 1,
        (Choice::Rock, Choice::Paper) => 6 + 2,
        (Choice::Paper, Choice::Paper) => 3 + 2,
        (Choice::Scissors, Choice::Paper) => 0 + 2,
        (Choice::Rock, Choice::Scissors) => 0 + 3,
        (Choice::Paper, Choice::Scissors) => 6 + 3,
        (Choice::Scissors, Choice::Scissors) => 3 + 3,
    }
}

fn parse_opponent_play(opponent_choice: &str) -> Choice {
    match opponent_choice {
        "A" => Choice::Rock,
        "B" => Choice::Paper,
        "C" => Choice::Scissors,
        _ => panic!("Unexpected choice"),
    }
}

enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum RoundResult {
    Win,
    Loss,
    Draw,
}
