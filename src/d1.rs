
pub fn part1(input: String) -> i32 {
	input
	.lines()
	.fold(0, |acc, s| acc + get_caliberation_unit(s.as_bytes().to_vec()))
}

fn get_caliberation_unit(val: Vec<u8>) -> i32 {
	let digit_chars = val
	.into_iter()
	.filter(|x| x.is_ascii_digit())
	.map(|x| char::from(x))
	.collect::<Vec<char>>();

	let unit_as_string = format!("{}{}", digit_chars[0], digit_chars[digit_chars.len() - 1]);

	unit_as_string.parse().unwrap()
}


pub fn part2() -> () {}

fn sample_data_part1() -> String {
	String::from("1abc2
	pqr3stu8vwx
	a1b2c3d4e5f
	treb7uchet")
}

#[test]
fn test_sample_data() {
	let result = part1(sample_data_part1());
	assert_eq!(result, 142);
}