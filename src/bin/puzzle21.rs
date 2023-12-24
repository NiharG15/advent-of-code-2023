use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    // let input = include_str!("../../inputs/puzzle21_sample.txt");
    let input = include_str!("../../inputs/puzzle21.txt");

    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();


    let mut starting_position = (0, 0);
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'S' {
                starting_position = (i, j);
            }
        }
    }

    let part1_count = bfs(&grid, starting_position, 64);
    println!("-- Part 1 Ans: {part1_count}");

    /*
    The number of nodes added at each 65th step forms a quadratic sequence.
    Get the first 3 terms of the sequence by running bfs unbounded:
         3882, 34441, 95442
     Using the formula for nth term in a quadratic sequence, we get fofn.
     fofn at n = 202300 is the Part 2 answer.
     */
    bfs_unbounded(&grid, starting_position, 5000);
    let part2_count = fofn(26501365 / 131);
    println!("-- Part 2 Ans: {part2_count}");
}

fn bfs(grid: &Vec<Vec<char>>, starting_position: (usize, usize), max_steps: usize) -> usize {
    let max_x = grid.len();
    let max_y = grid[0].len();

    let mut new_grid = grid.clone();

    let mut current_nodes = vec![(starting_position, 0)];
    let mut step_count = 0;

    let mut visited_steps: HashSet<((usize, usize), usize)> = HashSet::new();

    while !current_nodes.is_empty() && step_count < max_steps {
        let mut next_nodes = vec![];

        for (position, step) in current_nodes {
            for neighbor in iter_neighbors4_bounded_usize(position.0, position.1, max_x, max_y) {
                if grid[neighbor.0][neighbor.1] == '#' {
                    continue;
                }
                if visited_steps.contains(&(neighbor, step + 1)) {
                    continue;
                }
                next_nodes.push((neighbor, step + 1));
                visited_steps.insert((neighbor, step + 1));
                new_grid[neighbor.0][neighbor.1] = 'O';
            }
        }

        // println!("After step {step_count}");
        // print_grid(&new_grid);
        // println!("At step {step_count} found nodes: {}", next_nodes.len());
        current_nodes = next_nodes;
        step_count += 1;
    }

    current_nodes.len()
}

fn bfs_unbounded(grid: &Vec<Vec<char>>, starting_position: (usize, usize), max_steps: usize) -> usize {
    let max_x = grid.len();
    let max_y = grid[0].len();

    let mut current_nodes = vec![((starting_position.0 as isize, starting_position.1 as isize), 0)];
    let mut step_count = 1;

    let mut visited_steps: HashSet<((isize, isize), usize)> = HashSet::new();

    let mut last_diff: isize = 0;

    let goal = 26501365;

    while !current_nodes.is_empty() && step_count <= max_steps {
        let mut next_nodes = vec![];

        for (position, step) in &current_nodes {
            for neighbor in iter_neighbors4_unbounded(position.0, position.1) {
                let mut translated_x = (neighbor.0.abs() % max_x as isize) as usize;
                let mut translated_y = (neighbor.1.abs() % max_y as isize) as usize;
                if neighbor.0 < 0 && translated_x != 0 {
                    translated_x = max_x - translated_x;
                }
                if neighbor.1 < 0 && translated_y != 0 {
                    translated_y = max_y - translated_y;
                }

                if grid[translated_x][translated_y] == '#' {
                    continue;
                }

                if visited_steps.contains(&(neighbor, step + 1)) {
                    continue;
                }
                next_nodes.push((neighbor, step + 1));
                visited_steps.insert((neighbor, step + 1));
            }
        }


        if step_count % 131 == goal % 131 {
            println!("At step {step_count} found nodes: {}", next_nodes.len());
            let diff = next_nodes.len() - current_nodes.len();
            println!("diff: {diff}");
            println!("diff2: {}", diff as isize - last_diff);
            last_diff = diff as isize;
        }
        current_nodes = next_nodes;
        step_count += 1;
    }

    current_nodes.len()
}

fn fofn(n: i32) -> u64 {
    let n = n as u64;
    let a0 = 3882_u64;
    let a1 = 34441_u64;
    let a2 = 95442_u64;

    let b0 = a0;
    let b1 = a1 - a0;
    let b2 = a2 - a1;

    b0 + b1 * n + (n * (n - 1) / 2) * (b2 - b1)
}


pub fn iter_neighbors4_bounded_usize(x: usize, y: usize, max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    [
        (x, y + 1),
        (x, y.wrapping_sub(1)),
        (x.wrapping_sub(1), y),
        (x + 1, y),
    ]
        .into_iter()
        .filter(|(nx, ny)| *nx < max_x && *ny < max_y)
        .unique()
        .collect_vec()
}

pub fn iter_neighbors4_unbounded(x: isize, y: isize) -> Vec<(isize, isize)> {
    [
        (x, y + 1),
        (x, y - 1),
        (x - 1, y),
        (x + 1, y),
    ]
        .into_iter()
        .collect_vec()
}