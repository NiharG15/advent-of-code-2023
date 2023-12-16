use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
struct Pos(usize, usize);

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Beam = (Pos, Direction);

fn main() {
    let input = include_str!("../../inputs/puzzle16_sample.txt");
    // let input = include_str!("../../inputs/puzzle16.txt");

    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let energized_tiles = run_beam((Pos(0, 0), Direction::Right), &grid);
    println!("-- Part 1 Ans: {}", energized_tiles.len());

    let mut candidates: Vec<Beam> = vec![];

    let max_x = grid.len();
    let max_y = grid[0].len();
    (0..max_x)
        .map(|i| (Pos(i, 0), Direction::Right))
        .for_each(|c| candidates.push(c));
    (0..max_x)
        .map(|i| (Pos(i, max_y - 1), Direction::Left))
        .for_each(|c| candidates.push(c));
    (0..max_y)
        .map(|i| (Pos(0, i), Direction::Down))
        .for_each(|c| candidates.push(c));
    (0..max_y)
        .map(|i| (Pos(max_x - 1, i), Direction::Up))
        .for_each(|c| candidates.push(c));

    let mut max_energized_tile_count = usize::MIN;

    for c in candidates {
        let energized_tiles = run_beam(c, &grid);
        max_energized_tile_count = max_energized_tile_count.max(energized_tiles.len());
    }

    println!("-- Part 2 Ans: {max_energized_tile_count}");
}

fn run_beam(starting_beam: Beam, grid: &Vec<Vec<char>>) -> HashMap<Pos, usize> {
    let max_x = grid.len();
    let max_y = grid[0].len();
    let mut energized_tiles: HashMap<Pos, usize> = HashMap::new();
    let mut current_beams: Vec<Beam> = vec![starting_beam];
    let mut seen: HashSet<Beam> = HashSet::new();

    while !current_beams.is_empty() {
        let mut next_beams = vec![];
        for b in &current_beams {
            match (grid[b.0 .0][b.0 .1], b.1) {
                ('.' | '|', Direction::Up)
                | ('/', Direction::Right)
                | ('\\', Direction::Left) => {
                    move_beam_up(&mut next_beams, b);
                }
                ('.' | '|', Direction::Down)
                | ('/', Direction::Left)
                | ('\\', Direction::Right) => {
                    move_beam_down(&mut next_beams, b, max_x);
                },
                ('.' | '-', Direction::Left) | ('/', Direction::Down) | ('\\', Direction::Up) => {
                    move_beam_left(&mut next_beams, b);
                }
                ('.' | '-', Direction::Right) | ('/', Direction::Up) | ('\\', Direction::Down) => {
                    move_beam_right(&mut next_beams, b, max_y);
                }
                ('|', Direction::Left | Direction::Right) => {
                    move_beam_up(&mut next_beams, b);
                    move_beam_down(&mut next_beams, b, max_x);
                }
                ('-', Direction::Up | Direction::Down) => {
                    move_beam_right(&mut next_beams, b, max_y);
                    move_beam_left(&mut next_beams, b);
                }
                _ => unreachable!(),
            }
            energized_tiles
                .entry(b.0)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        current_beams.clear();
        for nb in &next_beams {
            if !seen.contains(nb) {
                current_beams.push(*nb);
                seen.insert(*nb);
            }
        }
    }
    energized_tiles
}

fn move_beam_right(next_beams: &mut Vec<(Pos, Direction)>, b: &Beam, max_y: usize) {
    if b.0 .1 + 1 < max_y {
        next_beams.push((Pos(b.0 .0, b.0 .1 + 1), Direction::Right));
    }
}

fn move_beam_left(next_beams: &mut Vec<(Pos, Direction)>, b: &Beam) {
    if b.0 .1 > 0 {
        next_beams.push((Pos(b.0 .0, b.0 .1 - 1), Direction::Left));
    }
}

fn move_beam_down(next_beams: &mut Vec<(Pos, Direction)>, b: &Beam, max_x: usize) {
    if b.0 .0 + 1 < max_x {
        next_beams.push((Pos(b.0 .0 + 1, b.0 .1), Direction::Down));
    }
}

fn move_beam_up(next_beams: &mut Vec<(Pos, Direction)>, b: &Beam) {
    if b.0 .0 > 0 {
        next_beams.push((Pos(b.0 .0 - 1, b.0 .1), Direction::Up));
    }
}
