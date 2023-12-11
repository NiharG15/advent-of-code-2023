use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use rust_aoc_2023::utils::{iter_neighbors4, iter_neighbors4_bounded};

type Pos = (i64, i64);

const NORTH: Pos = (-1i64, 0i64);
const SOUTH: Pos = (1, 0);
const WEST: Pos = (0, -1);
const EAST: Pos = (0, 1);

fn main() {
    // let input = include_str!("../../inputs/puzzle10_sample.txt");
    let input = include_str!("../../inputs/puzzle10.txt");

    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let max_x = grid.len() as i64;
    let max_y = grid[0].len() as i64;

    let starting_position = grid.iter().enumerate().map(|(i, row)| {
        let s_idx = row.iter().position(|c| c == &'S');
        (i, s_idx)
    })
        .filter(|(_, s_opt)| s_opt.is_some())
        .map(|(i, s_opt)| (i as i64, s_opt.unwrap() as i64))
        .last().unwrap();

    let mut path: HashSet<Pos> = HashSet::new();
    path.insert(starting_position);

    let (dist, pipes) = dfs_queue(&grid, starting_position, max_x, max_y);

    println!("-- Part 1 Ans: {}", dist / 2);

    let mut grid2 = vec![];

    for _ in 0..max_x {
        grid2.push(vec!['*'; max_y as usize]);
    }

    for p in &pipes {
        grid2[p.0 as usize][p.1 as usize] = 'P';
    }

    print_grid(&grid2, max_x, max_y);

    let pipe_set: HashSet<Pos> = pipes.iter().copied().collect();


    let ptm = find_points_to_mark(starting_position, &pipes);

    for point in ptm {
        mark(&mut grid2, point, &pipe_set);
    }

    print_grid(&grid2, max_x, max_y);

    let count: usize = grid2.iter().map(|row| row.iter().filter(|c| c == &&'X').count()).sum();

    // By visual inspection -- flood fill marked the outer points.
    // So inner points = total points - pipe path points - outer points.
    let inner_point_count = (max_x * max_y) as usize - count - pipes.len();
    println!("-- Part 2 Ans: {inner_point_count}");
}

fn print_grid(grid: &[Vec<char>], max_x: i64, max_y: i64) {
    for i in 0..max_x {
        for j in 0..max_y {
            print!("{}", grid[i as usize][j as usize]);
        }
        println!();
    }
}

fn find_points_to_mark(starting_position: Pos, pipes: &Vec<Pos>) -> Vec<Pos> {
    let mut prev = starting_position;
    let mut points_to_mark = vec![];

    for i in (0..pipes.len() - 1).rev() {
        let curr = pipes[i];
        // Algorithm from Reddit - Mark only the points on the right side of the loop while
        // following the path.
        match (curr.0 - prev.0, curr.1 - prev.1) {
            (1, 0) => {
                // x│
                // x↓
                points_to_mark.push((curr.0, curr.1 - 1));
                points_to_mark.push((curr.0 - 1, curr.1 - 1));
            }
            (0, 1) => {
                // -→
                // xx
                points_to_mark.push((curr.0 + 1, curr.1));
                points_to_mark.push((curr.0 + 1, curr.1 - 1));
            }
            (-1, 0) => {
                // ↑x
                // │x
                points_to_mark.push((curr.0, curr.1 + 1));
                points_to_mark.push((curr.0 + 1, curr.1 + 1));
            }
            (0, -1) => {
                // xx
                // ←-
                points_to_mark.push((curr.0 - 1, curr.1));
                points_to_mark.push((curr.0 - 1, curr.1 + 1));
            }
            (_, _) => {}
        }
        prev = curr;
    }

    points_to_mark
}

fn mark(grid: &mut Vec<Vec<char>>, current: Pos, pipes: &HashSet<Pos>) {
    if current.0 < 0 || current.0 >= grid.len() as i64 || current.1 < 0 || current.1 >= grid[0].len() as i64 {
        return;
    }

    if pipes.contains(&current) {
        return;
    }

    if grid[current.0 as usize][current.1 as usize] == 'X' {
        return;
    }

    grid[current.0 as usize][current.1 as usize] = 'X';

    for next in iter_neighbors4(current.0, current.1) {
        mark(grid, next, pipes);
    }
}

// todo: Handle cycles
fn dfs_queue(grid: &[Vec<char>], starting_position: Pos, max_x: i64, max_y: i64) -> (i64, Vec<Pos>) {
    let mut queue: VecDeque<(Pos, Pos, i64)> = VecDeque::new();

    let mut max_dist_so_far = i64::MIN;
    queue.push_front((starting_position, starting_position, 0));

    let mut parent_map: HashMap<Pos, Pos> = HashMap::new();
    let mut target = (0, 0);

    while let Some((current, previous, distance)) = queue.pop_front() {
        for neighbor in iter_neighbors4_bounded(current.0, current.1, max_x, max_y) {
            let curr = grid[current.0 as usize][current.1 as usize];
            let next = grid[neighbor.0 as usize][neighbor.1 as usize];
            if neighbor != previous
                && !queue.iter().any(|st| st.0 == neighbor)
                && is_compatible(curr, next, (neighbor.0 - current.0, neighbor.1 - current.1)) {
                if next == 'S' {
                    max_dist_so_far = max_dist_so_far.max(distance + 1);
                    target = current;
                    break;
                }
                // println!("Current: {current:?}: {curr}. Next: {neighbor:?}: {next}");
                parent_map.insert(neighbor, current);
                queue.push_front((neighbor, current, distance + 1));
            }
        }
    }

    let mut curr = target;
    let mut pipes = vec![target];
    while let Some(next) = parent_map.get(&curr) {
        curr = *next;
        pipes.push(curr);
    }


    (max_dist_so_far, pipes)
}

fn is_compatible(current: char, next: char, direction: Pos) -> bool {
    match (current, direction, next) {
        (_, _, 'S') => true,
        ('S' | '|' | 'L' | 'J', NORTH, '|' | '7' | 'F') => true,
        ('S' | '|' | '7' | 'F', SOUTH, '|' | 'L' | 'J') => true,
        ('S' | '-' | 'L' | 'F', EAST, '-' | 'J' | '7') => true,
        ('S' | '-' | 'J' | '7', WEST, '-' | 'L' | 'F') => true,
        (_, _, _) => false
    }
}