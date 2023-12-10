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

#[test]
fn transpose() {
    let input: String = String::from("1,2,3,4\n5,6,7,8\n10,11,12,13\n");
    let expected: String = String::from("1,5,10\n2,6,11\n3,7,12\n4,8,14\n");
    let mut mat: Vec<Vec<u8>> = vec![];
    input.lines().for_each(|line| {
        mat.push(line.split(",").map(|s| s.parse::<u8>().unwrap()).collect());
    });
    println!("{:?}", mat);
}
pub fn part2(input: String) -> i32 {
    42
}
