use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref NUM_TO_WORD_MAP: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("one", "1");
        map.insert("two", "2");
        map.insert("three", "3");
        map.insert("four", "4");
        map.insert("five", "5");
        map.insert("six", "6");
        map.insert("seven", "7");
        map.insert("eight", "8");
        map.insert("nine", "9");
        map
    };
}

fn main() {
    // let input = include_str!("../../inputs/puzzle1_sample.txt");
    let input = include_str!("../../inputs/puzzle1.txt");

    let lines: Vec<&str> = input.lines().collect();

    let sum: u32 = lines
        .iter()
        .map(|line| {
            let numbers: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();

            let number: u32 = format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
                .parse()
                .unwrap();

            number
        })
        .sum();

    println!("-- Part 1 Answer: {}", sum);

    let sum2: u32 = lines
        .iter()
        // .map(|line| pre_process(&line))
        .map(|line| {
            let mut indices = NUM_TO_WORD_MAP.iter().flat_map(|e| {
                line.match_indices(e.0)
                .chain(line.match_indices(e.1))
            })
            .collect::<Vec<_>>();

            indices.sort_by(|e1, e2| e1.0.cmp(&e2.0));

            let first = maybe_word_to_num(indices.first().unwrap().1);
            let last = maybe_word_to_num(indices.last().unwrap().1);

            let number: u32 = format!("{}{}", first, last)
                .parse()
                .unwrap();

            number
        })
        .sum();

    println!("-- Part 2 Answer: {}", sum2);
}

fn maybe_word_to_num(maybe_word: &str) -> &str {
    NUM_TO_WORD_MAP.get(maybe_word).unwrap_or(&maybe_word)
}
