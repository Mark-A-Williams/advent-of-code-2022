extern crate stopwatch;
mod solutions;

use stopwatch::Stopwatch;
use crate::solutions::day_1::{part_2, part_1};

fn main() {
    let mut sw = Stopwatch::start_new();
    part_1();
    println!("Part 1 completed in {}ms", sw.elapsed_ms());

    sw.reset();
    
    sw.start();
    part_2();
    println!("Part 2 completed in {}ms", sw.elapsed_ms());
}
