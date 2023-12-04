use std::fs::read_to_string;

pub fn problem_input(file_path: &str) -> String {
    read_to_string(file_path).unwrap()
}