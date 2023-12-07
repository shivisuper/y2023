mod d1;
mod utils;

use crate::utils::file::problem_input;
fn main() {
    println!(
        "Part 1 solution is: {}",
        d1::part1(problem_input("src/resources/d1_p1.txt"))
    );
    println!(
        "Part 2 solution is: {}",
        d1::part2(problem_input("src/resources/d1_p1.txt"))
    );
}
