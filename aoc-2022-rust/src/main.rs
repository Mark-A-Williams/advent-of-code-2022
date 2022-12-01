use stopwatch::Stopwatch;
use crate::solutions::day_1::{day_1_part_2, day_1_part_1};

mod solutions;


extern crate stopwatch;

fn main() {
    let mut sw = Stopwatch::start_new();
    day_1_part_1();
    println!("Part 1 completed in {}ms", sw.elapsed_ms());
    sw.reset();
    sw.start();
    day_1_part_2();
    println!("Part 2 completed in {}ms", sw.elapsed_ms());
}
