#![allow(unused_variables)]

pub fn part1(input: String) -> i32 {
    42
}

fn part1_sample() -> String {
    String::from(
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
    )
}

#[test]
fn test_part1() -> () {
    let res = part1(part1_sample());
    assert_eq!(4361, res);
}

pub fn part2(input: String) -> i32 {
    42
}
