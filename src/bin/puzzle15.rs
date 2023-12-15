use indexmap::IndexMap;
use itertools::Itertools;

fn main() {
    // let input = include_str!("../../inputs/puzzle15_sample.txt");
    let input = include_str!("../../inputs/puzzle15.txt");

    let steps = input.split(',').collect_vec();

    let sum: usize = steps
        .iter()
        .map(|s| hash(&s.chars().collect_vec()))
        .sum();

    println!("-- Part 1 Ans: {sum}");

    let mut hashmap: Vec<IndexMap<&'static str, usize>> = vec![IndexMap::new(); 256];

    for step in &steps {
        if step.contains('=') {
            let (name, power_str) = step.split('=').collect_tuple().unwrap();
            let hash = hash(&name.chars().collect_vec());
            let power = power_str.parse().unwrap();
            hashmap[hash]
                .entry(name)
                .and_modify(|e| *e = power)
                .or_insert(power);
        } else {
            // -
            let (name, _) = step.split('-').collect_tuple().unwrap();
            let hash = hash(&name.chars().collect_vec());
            hashmap[hash].shift_remove(&name);
        }
    }

    let mut sum = 0;
    for (i, map) in hashmap.iter().enumerate() {
        sum += map
            .iter()
            .enumerate()
            .map(|(j, e)| (i + 1) * (j + 1) * e.1)
            .sum::<usize>();
    }

    println!("-- Part 2 Ans: {sum}");
}

fn hash(chars: &[char]) -> usize {
    let mut hash: usize = 0;
    for c in chars {
        hash += (*c) as usize;
        hash *= 17;
        hash %= 256;
    }
    hash
}
