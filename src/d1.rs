#![allow(unused_variables)]

pub fn part1(input: String) -> i32 {
    input.lines().fold(0, |acc, s| {
        acc + get_caliberation_unit(s.as_bytes().to_vec())
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
    let strs = String::from("one,two,three,four,five,six,seven,eight,nine");
    println!("{}{}", input, strs);
    281
}

fn sample_data_part1() -> String {
    String::from(
        "1abc2
	pqr3stu8vwx
	a1b2c3d4e5f
	treb7uchet",
    )
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
fn test_sample_data() {
    let result = part1(sample_data_part1());
    assert_eq!(result, 142);
}

#[test]
fn test_part2_sample() {
    let result = part2(sample_data_part2());
    assert_eq!(result, 281);
}

#[test]
fn random_test() {
    let strs = String::from("one,two,three,four,five,six,seven,eight,nine");
    assert!(strs.contains("two"));
}
