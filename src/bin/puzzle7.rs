use std::{collections::HashMap, cmp::Ordering::{Equal, Greater, Less, self}};

use itertools::Itertools;

#[derive(Debug)]
struct Card {
    t: &'static str,
    prio: u32
}

impl Card {
    fn new(t: &str) -> Card {
        match t {
            "A" => Card { t: "A", prio: 0 },
            "K" => Card { t: "K", prio: 1 },
            "Q" => Card { t: "Q", prio: 2 },
            "J" => Card { t: "J", prio: 3 },
            "T" => Card { t: "T", prio: 4 },
            "9" => Card { t: "9", prio: 5 },
            "8" => Card { t: "8", prio: 6 },
            "7" => Card { t: "7", prio: 7 },
            "6" => Card { t: "6", prio: 8 },
            "5" => Card { t: "5", prio: 9 },
            "4" => Card { t: "4", prio: 10 },
            "3" => Card { t: "3", prio: 11 },
            "2" => Card { t: "2", prio: 12 },
            _ => unreachable!()
        }
    }

    fn new_joker(t: &str) -> Card {
        match t {
            "A" => Card { t: "A", prio: 0 },
            "K" => Card { t: "K", prio: 1 },
            "Q" => Card { t: "Q", prio: 2 },
            "J" => Card { t: "J", prio: 13 },
            "T" => Card { t: "T", prio: 4 },
            "9" => Card { t: "9", prio: 5 },
            "8" => Card { t: "8", prio: 6 },
            "7" => Card { t: "7", prio: 7 },
            "6" => Card { t: "6", prio: 8 },
            "5" => Card { t: "5", prio: 9 },
            "4" => Card { t: "4", prio: 10 },
            "3" => Card { t: "3", prio: 11 },
            "2" => Card { t: "2", prio: 12 },
            _ => unreachable!()
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn new(cards: &[Card]) -> HandType {
        let card_types = cards.iter().map(|c| c.t).collect_vec();
        
        let set: HashMap<&str, usize> = card_types.iter().cloned().counts_by(|s| s);

        if set.len() == 1 {
            return HandType::FiveOfAKind;
        }
        
        if set.len() == 2 && set.values().contains(&4) {
            return HandType::FourOfAKind;
        }

        if set.len() == 2 && set.values().contains(&3) {
            return HandType::FullHouse;
        }

        if set.len() == 3 && set.values().contains(&3) {
            return HandType::ThreeOfAKind;
        }

        if set.len() == 3 && set.values().contains(&2) && set.values().contains(&1) {
            return HandType::TwoPair;
        }

        if set.len() == 4 && set.values().contains(&2) && set.values().contains(&1) {
            return HandType::OnePair;
        }

        if set.len() == 5 {
            return HandType::HighCard;
        }

        unreachable!()
    } 

    fn new_joker(cards: &[Card]) -> HandType {
        let card_types = cards.iter().map(|c| c.t).collect_vec();
        
        let set: HashMap<&str, usize> = card_types.iter().cloned().counts_by(|s| s);

        let joker_count: usize = *set.get("J").unwrap_or(&0);

        if joker_count == 0 {
            return HandType::new(cards);
        }

        if set.len() == 1 {
            return HandType::FiveOfAKind;
        }

        if set.len() == 2 {
            // Joker & something else
            return HandType::FiveOfAKind;
        }
        
        if set.len() == 3 && joker_count == 3 {
            return HandType::FourOfAKind;
        }

        if set.len() == 3 && joker_count == 2 {
            return HandType::FourOfAKind;
        }

        if set.len() == 3 && joker_count == 1 && set.values().contains(&3) {
            return HandType::FourOfAKind;
        }

        if set.len() == 3 && joker_count == 1 {
            return HandType::FullHouse;
        }

        if set.len() == 4 && joker_count == 1 {
            return HandType::ThreeOfAKind;
        }

        if set.len() == 4 && joker_count == 2 {
            return HandType::ThreeOfAKind;
        }

        if set.len() == 3 && set.values().contains(&2) && set.values().contains(&1) {
            return HandType::TwoPair;
        }

        if set.len() == 5 && joker_count == 1 {
            return HandType::OnePair;
        }

        unreachable!("{:?}", card_types)
    } 
}

fn compare_cards(c1: &[Card], c2: &[Card]) -> Ordering {
    for (card1, card2) in c1.iter().zip(c2) {
        if card1.t == card2.t {
            continue;
        }

        if card1.prio < card2.prio {
            return Greater;
        } else {
            return Less;
        }
    }
    unreachable!()
}

fn main() {
    // let input = include_str!("../../inputs/puzzle7_sample.txt");
    let input = include_str!("../../inputs/puzzle7.txt");
    
    let lines = input.lines().collect_vec();

    let mut cards_and_scores: Vec<(HandType, Vec<Card>, u32)> = lines.iter().map(|l| {
        let parts = l.split_ascii_whitespace().collect_vec();
        let cards = parts[0].chars().map(|c| Card::new(c.to_string().as_str())).collect_vec();
        (
            HandType::new(&cards),
            cards,
            parts[1].parse().unwrap()
        )
    }).collect_vec();


    cards_and_scores.sort_by(|c1, c2| {
        match c1.0.partial_cmp(&c2.0)
            .unwrap_or(Equal) {
            Equal => {
                compare_cards(&c1.1, &c2.1)
            },
            c => c
        }
    });

    let part1_ans: u32 = cards_and_scores.iter().enumerate().map(|(i, cns)| {
        (i + 1) as u32 * cns.2
    }).sum();

    println!("-- Part 1 Answer: {}", part1_ans);

    let mut cards_and_scores: Vec<(HandType, Vec<Card>, u32)> = lines.iter().map(|l| {
        let parts = l.split_ascii_whitespace().collect_vec();
        let cards = parts[0].chars().map(|c| Card::new_joker(c.to_string().as_str())).collect_vec();
        (
            HandType::new_joker(&cards),
            cards,
            parts[1].parse().unwrap()
        )
    }).collect_vec();


    cards_and_scores.sort_by(|c1, c2| {
        match c1.0.partial_cmp(&c2.0)
            .unwrap_or(Equal) {
            Equal => {
                compare_cards(&c1.1, &c2.1)
            },
            c => c
        }
    });

    let part2_ans: u32 = cards_and_scores.iter().enumerate().map(|(i, cns)| {
        (i + 1) as u32 * cns.2
    }).sum();

    println!("-- Part 2 Answer: {}", part2_ans);
}
