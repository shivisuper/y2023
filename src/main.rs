mod d1;
mod d2;
mod d3;
mod utils;

use crate::utils::file::problem_input;
fn main() {
    println!(
        "Part 1 solution is: {}",
        d3::part1(problem_input("src/resources/d3.txt"))
    );
    println!(
        "Part 2 solution is: {}",
        d3::part2(problem_input("src/resources/d3.txt"))
    );
}
