use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;

use rust_aoc_2023::utils::iter_neighbors;

lazy_static! {
    static ref NUM_PAT: Regex = Regex::new("(\\d+)").unwrap();
}

#[derive(Debug, Hash, PartialEq)]
struct Match {
    number: u32,
    start: usize,
    end: usize,
}

fn main() {
    // let input = include_str!("../../inputs/puzzle3_sample.txt");
    let input = include_str!("../../inputs/puzzle3.txt");

    let schematic: Vec<&str> = input.lines().collect();

    let mut number_loc_map: HashMap<(i32, i32), Match> = HashMap::new();

    schematic.iter().enumerate().for_each(|(i, &s)| {
        NUM_PAT.captures_iter(s).for_each(|cap| {
            let match_ = cap.get(0).unwrap();
            let value = match_.as_str().parse().unwrap();
            for j in match_.start()..match_.end() {
                number_loc_map.insert(
                    (i as i32, j as i32),
                    Match {
                        number: value,
                        start: match_.start(),
                        end: match_.end(),
                    },
                );
            }
        });
    });

    let mut seen_set: HashSet<(i32, i32)> = HashSet::new();

    let mut part_sum = 0;
    let mut gear_ratio_sum = 0;

    for (i, &line) in schematic.iter().enumerate() {
        for (j, c) in line.char_indices() {
            match &c {
                '.' => {}
                c if c.is_numeric() => {}
                c => {
                    // (i, j) must be a symbol, iterate neighbors
                    let ii = i as i32;
                    let ij = j as i32;

                    let mut neighbors = vec![];
                    let mut gear_seen_set: HashSet<(i32, i32)> = HashSet::new();

                    iter_neighbors(ii, ij).into_iter().for_each(|(x, y)| {
                        if let Some(m) = number_loc_map.get(&(x, y)) {
                            if c == &'*' && !gear_seen_set.contains(&(x, y)) {
                                neighbors.push(m.number);
                                gear_seen_set.insert((x, y));
                                for mj in m.start..m.end {
                                    gear_seen_set.insert((x, mj as i32));
                                }
                            }

                            if !seen_set.contains(&(x, y)) {
                                part_sum += m.number;
                                seen_set.insert((x, y));
                                for mj in m.start..m.end {
                                    seen_set.insert((x, mj as i32));
                                }
                            }
                        }
                    });

                    if neighbors.len() == 2 {
                        gear_ratio_sum += neighbors.first().unwrap() * neighbors.last().unwrap();
                    }
                }
            }
        }
    }

    println!("-- Part 1 Answer: {}", part_sum);
    println!("-- Part 2 Answer: {}", gear_ratio_sum);
}
