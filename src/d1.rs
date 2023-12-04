
pub fn part1() -> i32 {
    sample_data()
    .lines()
    .fold(0, |acc, s| acc + get_caliberation_unit(s.as_bytes().to_vec()))
}

fn get_caliberation_unit(val: Vec<u8>) -> i32 {
    let mut start_idx: usize = 0;
    let mut end_idx = val.len() - 1;
    let first: i32;
    let last: i32;
    while start_idx < end_idx {


    }
}


pub fn part2() -> () {}

fn sample_data() -> String {
    String::from("1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet")
}