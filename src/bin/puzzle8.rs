use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use rust_aoc_2023::utils::lcm;

lazy_static! {
    static ref TARGET_PAT: Regex = Regex::new("\\((\\w+), (\\w+)\\)").unwrap();
}

fn main() {
    // let input = include_str!("../../inputs/puzzle8_sample.txt");
    let input = include_str!("../../inputs/puzzle8.txt");

    let lines = input.split("\n\n").collect_vec();

    let instructions = lines[0];

    let nodes: HashMap<&str, (&str, &str)> = lines[1]
        .lines()
        .map(|l| {
            if let Some((source, targets)) = l.split('=').map(str::trim).collect_tuple() {
                let target_matches = TARGET_PAT.captures(targets).unwrap();

                (
                    source,
                    (
                        target_matches.get(1).unwrap().as_str(),
                        target_matches.get(2).unwrap().as_str(),
                    ),
                )
            } else {
                unreachable!()
            }
        })
        .collect();

    let mut curr: &str = "AAA";
    let mut steps = 0;

    for c in instructions.chars().cycle() {
        steps += 1;
        curr = match c {
            'L' => nodes.get(curr).unwrap().0,
            'R' => nodes.get(curr).unwrap().1,
            _ => unreachable!(),
        };

        if curr == "ZZZ" {
            break;
        }
    }

    println!("-- Part 1 Answer: {steps}");

    let starting_nodes = nodes
        .keys()
        .filter(|s| s.ends_with('A'))
        .cloned()
        .collect_vec();
    let mut steps_to_z = vec![];

    for node in starting_nodes {
        let mut steps = 0;
        let mut curr = node;
        'inner: for c in instructions.chars().cycle() {
            steps += 1;
            curr = match c {
                'L' => nodes.get(curr).unwrap().0,
                'R' => nodes.get(curr).unwrap().1,
                _ => unreachable!(),
            };
            if curr.ends_with('Z') {
                steps_to_z.push(steps);
                break 'inner;
            }
        }
    }

    println!("-- Part 2 Answer: {:?}", lcm(&steps_to_z));
}


