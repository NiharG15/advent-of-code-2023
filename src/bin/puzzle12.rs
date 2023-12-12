use std::collections::HashMap;

use itertools::Itertools;

type Key = (usize, usize, usize);

fn main() {
    // let input = include_str!("../../inputs/puzzle12_sample.txt");
    let input = include_str!("../../inputs/puzzle12.txt");

    let lines = input.lines().collect_vec();

    let data: Vec<(&str, Vec<usize>)> = lines
        .iter()
        .map(|l| {
            let (springs, groups) = l.split_ascii_whitespace().collect_tuple().unwrap();
            (
                springs,
                groups.split(',').map(|s| s.parse().unwrap()).collect_vec(),
            )
        })
        .collect_vec();

    let mut sum = 0;
    for (s, group) in &data {
        let chars = s.chars().collect_vec();
        let mut mem= HashMap::new();
        let count = backtrack_memoized(&chars, 0, group, 0, 0, &mut mem);
        // println!("For s: {}, count: {}", s, count);
        sum += count;
    }

    println!("-- Part 1 Ans: {sum}");

    let mut sum = 0;
    for (s, group) in &data {
        let chars = s.chars().collect_vec();
        let cloned_chars = [chars.clone(), chars.clone(), chars.clone(), chars.clone(), chars.clone()].iter().intersperse(&vec!['?']).flatten().copied().collect_vec();
        let cloned_groups = group.repeat(5).iter().copied().collect_vec();
        let mut mem: HashMap<Key, usize> = HashMap::new();
        let count = backtrack_memoized(&cloned_chars, 0, &cloned_groups, 0, 0, &mut mem);
        // println!("For s: {:?}, {:?} count: {}", cloned_chars, cloned_groups, count);
        sum += count;
    }

    println!("-- Part 2 Ans: {sum}");

}


fn backtrack_memoized(spring: &[char], current_pos: usize, groups: &[usize], current_group_pos: usize, current_group_size: usize, mem: &mut HashMap<Key, usize>) -> usize {
    if mem.contains_key(&(current_pos, current_group_pos, current_group_size)) {
        return *mem.get(&(current_pos, current_group_pos, current_group_size)).unwrap();
    }

    if current_pos == spring.len() {
        if current_group_pos == groups.len() && current_group_size == 0 {
            return 1;
        }

        if current_group_pos == groups.len() - 1 && groups[current_group_pos] == current_group_size {
            return 1;
        }

        return 0;
    }

    let mut answer = 0;
    if spring[current_pos] == '.' || spring[current_pos] == '?' {
        if current_group_size == 0 {
            answer += backtrack_memoized(spring, current_pos + 1, groups, current_group_pos, 0, mem);
        } else if current_group_size > 0 && current_group_pos < groups.len() && groups[current_group_pos] == current_group_size {
            answer += backtrack_memoized(spring, current_pos + 1, groups, current_group_pos + 1, 0, mem);
        }
    }

    if spring[current_pos] == '#' || spring[current_pos] == '?' {
        answer += backtrack_memoized(spring, current_pos + 1, groups, current_group_pos, current_group_size + 1, mem);
    }

    mem.insert((current_pos, current_group_pos, current_group_size), answer);

    answer
}
