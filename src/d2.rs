#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet};

use regex::Regex;

pub fn part1(input: String) -> i32 {
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;

    input
        .lines()
        .map(|game_input| {
            let game_id = extract_game_id(game_input);
            let color_map = extract_color_map(game_input);
            let mut is_valid_game = true;
            for (color, max_count) in &color_map {
                if *color == "red" && *max_count > MAX_RED {
                    is_valid_game = false;
                    break;
                } else if *color == "green" && *max_count > MAX_GREEN {
                    is_valid_game = false;
                    break;
                } else if *color == "blue" && *max_count > MAX_BLUE {
                    is_valid_game = false;
                    break;
                }
            }
            (game_id.parse::<i32>().unwrap(), is_valid_game)
        })
        .filter(|(_, is_valid_game)| *is_valid_game)
        .fold(0, |acc, (game_id, _)| acc + game_id)
}

fn extract_game_id(input: &str) -> &str {
    let (_, [game_id]) = Regex::new(r"Game (?<game_id>[0-9]{1,3}):")
        .unwrap()
        .captures(input)
        .unwrap()
        .extract();

    game_id
}

fn extract_color_map(input: &str) -> HashMap<&str, i32> {
    let re = Regex::new(r"(?<count>[0-9]{1,2})\s(?<color>blue|green|red)").unwrap();
    let color_vecs: Vec<(&str, i32)> = re
        .captures_iter(input)
        .map(|caps| {
            let (_, [count, color]) = caps.extract();
            (color, count.parse::<i32>().unwrap())
        })
        .collect();

    create_color_map(color_vecs)
}

fn create_color_map(vals: Vec<(&str, i32)>) -> HashMap<&str, i32> {
    let mut res: HashMap<&str, i32> = HashMap::new();
    for (color, cnt) in vals {
        let max_count = res.entry(color).or_insert(i32::MIN);
        if cnt > *max_count {
            *max_count = cnt;
        }
    }
    res
}

fn part1_sample() -> String {
    String::from(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    )
}

#[test]
fn test_part1() -> () {
    let res = part1(part1_sample());
    assert_eq!(8, res);
}

#[test]
fn test_part2() -> () {
    let res = part2(part2_sample());
    assert_eq!(2286, res);
}

pub fn part2(input: String) -> i32 {
    input
        .lines()
        .map(|game_input| {
            let color_map = extract_color_map(game_input);
            let mut cube_power = 1;
            for (_, v) in color_map {
                cube_power *= v;
            }
            cube_power
        })
        .fold(0, |acc, s| acc + s)
}

fn part2_sample() -> String {
    String::from(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    )
}
