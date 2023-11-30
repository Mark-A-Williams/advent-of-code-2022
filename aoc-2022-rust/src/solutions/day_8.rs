use itertools::Itertools;

use crate::solutions::file_helpers::get_multidimensional_int_array_from_file;

pub fn part_1() {
    let forest = get_multidimensional_int_array_from_file("../inputs/8.txt");
    // let mut state = [[0, 1, 2, 3], [1, 2, 4]];
    // let foo = state.get(0);
    println!("{:?}", forest)
}

pub fn part_2() {}

fn is_tree_in_position_visible(x: i32, y: i32, map: Vec<Vec<i32>>) -> bool {
    // A tree is visible if:
    // - Everything to the right is shorter OR
    // - Everything to the left is shorter OR
    // - Everything above is shorter OR
    // - Everything below is shorter OR

    let my_row = map.iter().nth(y as usize).expect("Row does not exist");
    let my_column = 

    let visible_x = 
}

fn get_rotated_map(map: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if map.iter().map(|line| line.len()).unique().count() > 1 {
        panic!("All lines must be the same length");
    }

    let mut result: Vec<Vec<i32>> = vec![vec![]];
    for i in map[0].len() {
        result[0].append((map[i] as isize))
    }
}