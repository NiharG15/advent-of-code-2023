use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down
}

fn main() {
    // let input = include_str!("../../inputs/puzzle23_sample.txt");
    let input = include_str!("../../inputs/puzzle23.txt");

    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let part1 = dfs(&grid, true);

    println!("-- Part 1 Ans: {part1}");

    let part2 = dfs(&grid, false); // extremely slow, consider the running output.
    println!("-- Part 2 Ans: {part2}");
}

fn dfs(grid: &Vec<Vec<char>>, consider_slopes: bool) -> usize {
    let max_x = grid.len();
    let max_y = grid[0].len();

    let starting_position = (0usize, grid[0].iter().position(|c| c == &'.').unwrap());
    let ending_position = (max_x - 1, grid.last().unwrap().iter().position(|c| c == &'.').unwrap());

    let mut current_nodes = VecDeque::new();
    current_nodes.push_front((starting_position, HashSet::new()));

    let mut max_dist = 0usize;
    let mut max_path = HashSet::new();

    while let Some((node, path)) = current_nodes.pop_front() {
        let (x, y)= node;
        for (neighbor, dir) in [((x, y + 1), Direction::Right), ((x, y.wrapping_sub(1)), Direction::Left), ((x + 1, y), Direction::Down), ((x.wrapping_sub(1), y), Direction::Up)] {
            if neighbor.0 >= max_x || neighbor.1 >= max_y {
                continue;
            }
            let next_char = grid[neighbor.0][neighbor.1];
            if next_char == '#' {
                continue;
            }
            if consider_slopes {
                if next_char == '>' && dir != Direction::Right {
                    continue;
                }
                if next_char == '<' && dir != Direction::Left {
                    continue;
                }
                if next_char == '^' && dir != Direction::Up {
                    continue;
                }
                if next_char == 'v' && dir != Direction::Down {
                    continue;
                }
            }

            if neighbor == ending_position && path.len() >= max_dist {
                max_dist = max_dist.max(path.len() + 1);
                println!("Max found {max_dist}");
                max_path = path.clone();
                max_path.insert(neighbor);
            }

            if path.contains(&neighbor) {
                continue;
            }
            let mut new_path: HashSet<(usize, usize)> = path.clone();
            new_path.insert(neighbor);
            current_nodes.push_front((neighbor, new_path));
        }
    }

    max_dist
}