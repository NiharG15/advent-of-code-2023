use std::cmp::max;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Round {
    red: u32,
    blue: u32,
    green: u32,
}

#[derive(Debug)]
struct Game {
    rounds: Vec<Round>,
}

lazy_static! {
    static ref RED_PATTERN: Regex = Regex::new("(\\d+) red").unwrap();
    static ref BLUE_PATTERN: Regex = Regex::new("(\\d+) blue").unwrap();
    static ref GREEN_PATTERN: Regex = Regex::new("(\\d+) green").unwrap();
}

fn check_regex_and_return_match(re: &Regex, haystack: &str) -> Option<u32> {
    if let Some(cap) = re.captures(haystack) {
        if let Some(value) = cap.get(1) {
            return Some(value.as_str().parse().unwrap());
        }
    }

    None
}

fn to_round(string: &str) -> Round {
    Round {
        red: check_regex_and_return_match(&RED_PATTERN, string).unwrap_or(0),
        green: check_regex_and_return_match(&GREEN_PATTERN, string).unwrap_or(0),
        blue: check_regex_and_return_match(&BLUE_PATTERN, string).unwrap_or(0),
    }
}

fn is_game_possible(game: &Game, red_max: u32, blue_max: u32, green_max: u32) -> bool {
    for r in &game.rounds {
        if r.red > red_max {
            return false;
        }

        if r.blue > blue_max {
            return false;
        }

        if r.green > green_max {
            return false;
        }
    }

    true
}

fn minimum_set(rounds: &[Round]) -> Round {
    let mut red_min = 0;
    let mut blue_min = 0;
    let mut green_min = 0;

    for r in rounds {
        red_min = max(red_min, r.red);
        blue_min = max(blue_min, r.blue);
        green_min = max(green_min, r.green);
    }

    Round {
        red: red_min,
        blue: blue_min,
        green: green_min,
    }
}

fn main() {
    let input = include_str!("../../inputs/puzzle2_sample.txt");
    // let input = include_str!("../../inputs/puzzle2.txt");

    let games: Vec<Game> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            let rounds: Vec<&str> = parts[1].split(';').collect();

            Game {
                rounds: rounds.iter().map(|r| to_round(r)).collect(),
            }
        })
        .collect();

    let part1: usize = games
        .iter()
        .enumerate()
        .map(|(i, g)| {
            if is_game_possible(g, 12, 14, 13) {
                i + 1
            } else {
                0
            }
        })
        .sum();

    println!("-- Part 1: {}", part1);

    let part2: u32 = games
        .iter()
        .map(|g| {
            let min_set = minimum_set(&g.rounds);

            min_set.red * min_set.blue * min_set.green
        })
        .sum();

    println!("-- Part 2: {}", part2);
}
