#![allow(unused_variables)]
#![allow(dead_code)]

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
fn test_part1() {
    let res = part1(part1_sample());
    assert_eq!(4361, res);
}

#[test]
fn parse_input() {
    let mut processed_input = part1_sample()
        .lines()
        .map(|line| {
            let line_with_empty = format!(".{line}.");
            line_with_empty.chars().collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    let empty_row = "."
        .repeat(processed_input[0].len())
        .chars()
        .collect::<Vec<_>>();
    processed_input.insert(0, empty_row.clone());
    processed_input.push(empty_row);
    show_mat(&processed_input);
}

fn show_mat(mat: &Vec<Vec<char>>) {
    let mut idx = 1;
    for row in mat {
        idx += 1;
        for col in row {
            print!("{} ", *col);
        }
        println!();
    }
}

pub fn part2(input: String) -> i32 {
    42
}
