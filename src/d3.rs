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
fn test_part1() -> () {
    let res = part1(part1_sample());
    assert_eq!(4361, res);
}

#[test]
fn transpose() {
    let input = part1_sample();
    let mut transformed: Vec<Vec<&str>> = vec![];
    input.lines().for_each(|line| {
        transformed.push(line.split('.').collect::<Vec<&str>>());
    });
    show_mat(&transformed);
}

fn show_mat(mat: &Vec<Vec<&str>>) {
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
