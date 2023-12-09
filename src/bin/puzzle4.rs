use std::collections::{HashMap, HashSet};

fn main() {
    // let input = include_str!("../../inputs/puzzle4_sample.txt");
    let input = include_str!("../../inputs/puzzle4.txt");

    let card_strings: Vec<&str> = input.lines().collect();

    let mut part_1_ans = 0;

    let mut cards_counts: HashMap<usize, u32> = HashMap::new();
    let max_len = card_strings.len();

    for (i, &card_str) in card_strings.iter().enumerate() {
        cards_counts.entry(i).and_modify(|n| *n += 1).or_insert(1);

        let numbers: Vec<Vec<u32>> = card_str
            .split(':')
            .nth(1)
            .iter()
            .flat_map(|s| s.split('|'))
            .map(|s| {
                s.split_ascii_whitespace()
                    .map(|s1| s1.parse().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let winning_numbers: HashSet<u32> = numbers[0].iter().copied().collect::<HashSet<_>>();

        let your_numbers: HashSet<u32> = numbers[1].iter().copied().collect::<HashSet<_>>();

        let winning_count = winning_numbers.intersection(&your_numbers).count() as u32;

        if winning_count > 0 {
            part_1_ans += 2u32.pow(winning_count - 1);
        }

        let winning_factor: u32 = *cards_counts.get(&i).unwrap_or(&1);

        for j in (i + 1)..=std::cmp::min(i + winning_count as usize, max_len - 1) {
            cards_counts
                .entry(j)
                .and_modify(|n| *n += winning_factor)
                .or_insert(winning_factor);
        }
    }

    println!("-- Part 1 Answer: {part_1_ans}");
    println!("-- Part 2 Answer: {}", cards_counts.values().sum::<u32>());
}
