#![allow(unused_variables)]
#![allow(dead_code)]

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

pub fn part2(input: String) -> i32 {
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
    input
        .lines()
        .map(|s| {
            let mut digits_string = String::with_capacity(s.len());
            let mut idx: usize = 0;
            let input_chars = s.as_bytes();
            while idx < s.len() {
                let mut matched_str = String::new();
                for (k, v) in &numbers_map {
                    if s[idx..].starts_with(k) {
                        digits_string.push_str(v);
                        matched_str.push_str(k);
                        break;
                    } else if input_chars[idx].is_ascii_digit() {
                        digits_string.push(input_chars[idx] as char);
                        break;
                    }
                }
                if !matched_str.is_empty() {
                    idx += matched_str.len();
                } else {
                    idx += 1;
                }
            }
            let digits_char = digits_string.as_bytes();
            println!("{digits_string}");
            format!(
                "{}{}",
                digits_char[0] as char,
                digits_char[digits_char.len() - 1] as char
            )
        })
        .fold(0, |acc, s| acc + s.parse::<i32>().unwrap())
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
        // String::from("219\n8wo3\nabc123xyz\nx2ne34\n49872\nz1ight234\n7pqrst6teen\n"),
        281
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
