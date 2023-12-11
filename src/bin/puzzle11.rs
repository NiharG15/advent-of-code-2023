use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use rust_aoc_2023::utils::iter_neighbors4_bounded;

type Pos = (i64, i64);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Node {
    position: Pos,
    cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// todo: Reimplement this using simpler Manhattan Distance
fn main() {
    // let input = include_str!("../../inputs/puzzle11_sample.txt");
    let input = include_str!("../../inputs/puzzle11.txt");

    let grid = input.lines().map(|s| s.chars().collect_vec()).collect_vec();

    let mut grid_part1 = grid.clone();

    let orig_x = grid_part1.len();
    let orig_y = grid_part1[0].len();

    let rows_to_expand = grid_part1
        .iter()
        .positions(|row| row.iter().all(|c| c != &'#'))
        .collect_vec();

    let mut cols_to_expand = vec![];
    for j in 0..orig_y {
        let mut empty = true;
        for i in 0..orig_x {
            if grid_part1[i][j] == '#' {
                empty = false;
                break;
            }
        }
        if empty {
            cols_to_expand.push(j);
        }
    }

    insert_rows(&mut grid_part1, &rows_to_expand);
    insert_cols(&mut grid_part1, &cols_to_expand);

    print_grid(&grid);

    let galaxies: Vec<Pos> = find_galaxies(&grid_part1);

    let mut pair_distances: HashMap<(Pos, Pos), usize> = HashMap::new();

    for i in 0..galaxies.len() {
        let source = galaxies[i];
        let (distances, _) = find_shortest_path_bfs(&grid_part1, source);
        for k in i + 1..galaxies.len() {
            let target = galaxies[k];
            pair_distances.insert((source, target), *distances.get(&target).unwrap());
            pair_distances.insert((target, source), *distances.get(&target).unwrap());
        }
    }

    let sum: usize = pair_distances.values().sum();
    println!("-- Part 1 Ans: {}", sum / 2);

    let grid_part2 = grid.clone();

    let galaxies: Vec<Pos> = find_galaxies(&grid_part2);

    let expanded_rows_set: HashSet<usize> = rows_to_expand.iter().copied().collect();
    let expanded_cols_set: HashSet<usize> = cols_to_expand.iter().copied().collect();

    let mut pair_distances: HashMap<(Pos, Pos), usize> = HashMap::new();
    for i in 0..galaxies.len() {
        let source = galaxies[i];
        let (distances, paths) = find_shortest_path_bfs(&grid_part2, source);
        for k in i + 1..galaxies.len() {
            let target = galaxies[k];

            let original_distance = *distances.get(&target).unwrap();
            let original_path = find_path(&paths, source, target);

            let expanded_distance = expand_distance(original_distance, &original_path, &expanded_rows_set, &expanded_cols_set);

            pair_distances.insert((source, target), expanded_distance);
            pair_distances.insert((target, source), expanded_distance);
        }
    }

    let sum: usize = pair_distances.values().sum();
    println!("-- Part 2 Ans: {}", sum / 2);
}

fn expand_distance(distance: usize, path: &Vec<Pos>, expanded_rows: &HashSet<usize>, expanded_cols: &HashSet<usize>) -> usize {
    let mut expansion_factor = 999_999usize;
    // let mut expansion_factor = 9usize;
    let mut total_expansion = 0;

    for p in path {
        if expanded_rows.contains(&(p.0 as usize)) || expanded_cols.contains(&(p.1 as usize)) {
            total_expansion += expansion_factor;
        }
    }

    for p in path {
        if expanded_rows.contains(&(p.0 as usize)) && expanded_cols.contains(&(p.1 as usize)) {
            total_expansion -= 1;
        }
    }

    distance + total_expansion
}

fn find_path(paths: &HashMap<Pos, Pos>, source: Pos, target: Pos) -> Vec<Pos> {
    let mut curr = target;
    let mut path = vec![];

    while let Some(next) = paths.get(&curr) {
        path.push(curr);
        curr = *next;
    }

    path
}

// or just use damn Manhattan distance :(
fn find_shortest_path_bfs(grid: &[Vec<char>], source: Pos) -> (HashMap<Pos, usize>, HashMap<Pos, Pos>) {
    let max_x = grid.len();
    let max_y = grid[0].len();

    let mut dist: HashMap<Pos, usize> = HashMap::new();
    let mut prev: HashMap<Pos, Pos> = HashMap::new();
    for i in 0..max_x {
        for j in 0..max_y {
            dist.insert((i as i64, j as i64), usize::MAX);
        }
    }

    let mut heap = BinaryHeap::new();
    dist.insert(source, 0);

    heap.push(Node {
        position: source,
        cost: 0,
    });

    while let Some(Node { position, cost }) = heap.pop() {
        if cost > *dist.get(&position).unwrap() {
            continue;
        }

        for neighbor in iter_neighbors4_bounded(position.0, position.1, max_x as i64, max_y as i64) {
            let next = Node { position: neighbor, cost: cost + 1 };

            if next.cost < *dist.get(&neighbor).unwrap() {
                heap.push(next);
                prev.entry(neighbor).and_modify(|e| *e = position).or_insert(position);
                dist.insert(next.position, next.cost);
            }
        }
    }

    (dist, prev)
}

fn find_galaxies(grid: &[Vec<char>]) -> Vec<Pos> {
    let mut galaxies: Vec<Pos> = vec![];

    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'#' {
                galaxies.push((i as i64, j as i64));
            }
        }
    }

    galaxies
}

fn insert_cols(grid: &mut [Vec<char>], cols_to_expand: &[usize]) {
    for c in cols_to_expand.iter().rev() {
        for row in grid.iter_mut() {
            row.insert(*c, '.');
        }
    }
}

fn insert_rows(grid: &mut Vec<Vec<char>>, rows_to_expand: &[usize]) {
    let row_size = grid[0].len();
    let row = vec!['.'; row_size];

    for r in rows_to_expand.iter().rev() {
        grid.insert(*r, row.clone());
    }
}

fn print_grid(grid: &[Vec<char>]) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            print!("{}", grid[i][j]);
        }
        println!();
    }
}
