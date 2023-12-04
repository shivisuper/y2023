mod d1;
mod utils;

use crate::utils::file::problem_input;
fn main() {
    println!("Solution is: {}", d1::part1(problem_input("src/resources/d1_p1.txt")));
}
