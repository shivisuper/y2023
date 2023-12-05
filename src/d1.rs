#![allow(unused_variables)]

use std::collections::HashMap;

pub fn part1(input: String) -> i32 {
    input.lines().fold(0, |acc, s| {
        acc + get_caliberation_unit(s.trim().as_bytes().to_vec())
    })
}

fn get_caliberation_unit(val: Vec<u8>) -> i32 {
    let digit_chars = val
        .into_iter()
        .filter(|x| x.is_ascii_digit())
        .map(char::from)
        .collect::<Vec<char>>();

    let unit_as_string = format!("{}{}", digit_chars[0], digit_chars[digit_chars.len() - 1]);

    unit_as_string.parse().unwrap()
}

pub fn part2(input: String) -> String {
    let numbers_map = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    let number_literals = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let new_input: String = input
        .lines()
        .map(|s| {
            let mut val_with_digits = s.trim().to_string();
            for k in number_literals {
                val_with_digits = val_with_digits.replace(k, numbers_map.get(k).unwrap());
            }
            val_with_digits
        })
        .fold(String::new(), |acc, s| format!("{acc}{s}\n"));
    new_input
}

fn sample_data_part2() -> String {
    String::from(
        "two1nine
	eightwothree
	abcone2threexyz
	xtwone3four
	4nineeightseven2
	zoneight234
	7pqrstsixteen",
    )
}

#[test]
fn test_part2_sample() {
    let result = part2(sample_data_part2());
    assert_eq!(
        result,
        String::from("219\n8wo3\nabc123xyz\nx2ne34\n49872\nz1ight234\n7pqrst6teen\n"),
    );
}

fn sample_data_part1() -> String {
    String::from(
        "1abc2
	pqr3stu8vwx
	a1b2c3d4e5f
	treb7uchet",
    )
}

#[test]
fn test_sample_data() {
    let result = part1(sample_data_part1());
    assert_eq!(result, 142);
}
