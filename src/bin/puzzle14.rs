use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Rounded,
    Cube,
    Empty,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '#' => Cell::Cube,
            'O' => Cell::Rounded,
            '.' => Cell::Empty,
            _ => unreachable!(),
        }
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            match self {
                Cell::Empty => ".",
                Cell::Rounded => "O",
                Cell::Cube => "#",
            }
        ))
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

type Pos = (usize, usize);

fn main() {
    // let input = include_str!("../../inputs/puzzle14_sample.txt");
    let input = include_str!("../../inputs/puzzle14.txt");

    let mut grid = input
        .lines()
        .map(|l| l.chars().map(Cell::from).collect_vec())
        .collect_vec();

    let mut rounded_rock_row_counts: HashMap<usize, usize> = HashMap::new();

    let max_x = grid.len();
    let max_y = grid[0].len();

    for i in 0..max_x {
        for j in 0..max_y {
            if grid[i][j] == Cell::Rounded {
                let final_row = roll_north_along_column(&mut grid, i, j);
                rounded_rock_row_counts
                    .entry(final_row)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
    }

    let sum: usize = rounded_rock_row_counts
        .iter()
        .map(|(k, v)| v * (max_x - k))
        .sum();

    println!("-- Part 1 Ans: {sum}");

    let mut grid = input
        .lines()
        .map(|l| l.chars().map(Cell::from).collect_vec())
        .collect_vec();

    let mut state_set: HashMap<Vec<Pos>, usize> = HashMap::new();
    let mut cycles = 0;

    let mut cycle_count = 0;

    loop {
        perform_cycle(&mut grid, max_x, max_y);
        cycles += 1;

        let state = get_grid_state(&grid);

        if state_set.contains_key(&state) {
            cycle_count = cycles - state_set.get(&state).unwrap();
            break;
        }

        state_set.insert(state, cycles);
    }
    let remaining = (1_000_000_000 - cycles) % cycle_count;

    for i in 0..remaining {
        perform_cycle(&mut grid, max_x, max_y);
    }

    let part2_ans: usize = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == Cell::Rounded)
                .map(move |(j, _)| (i, j))
        })
        .counts_by(|p| p.0)
        .iter()
        .map(|(k, v)| v * (max_x - k))
        .sum();

    println!("-- Part 2 Ans: {part2_ans}");
}

fn perform_cycle(grid: &mut Vec<Vec<Cell>>, max_x: usize, max_y: usize) {
    for i in 0..max_x {
        for j in 0..max_y {
            if grid[i][j] == Cell::Rounded {
                roll_north_along_column(grid, i, j);
            }
        }
    }
    for i in 0..max_x {
        for j in 0..max_y {
            if grid[i][j] == Cell::Rounded {
                roll_west_along_row(grid, i, j);
            }
        }
    }
    for i in (0..max_x).rev() {
        for j in 0..max_y {
            if grid[i][j] == Cell::Rounded {
                roll_south_along_column(grid, i, j);
            }
        }
    }
    for i in 0..max_x {
        for j in (0..max_y).rev() {
            if grid[i][j] == Cell::Rounded {
                roll_east_along_row(grid, i, j);
            }
        }
    }
}

fn roll_north_along_column(grid: &mut Vec<Vec<Cell>>, row: usize, column: usize) -> usize {
    let mut current_row = row as isize;

    while current_row > 0 && grid[(current_row - 1) as usize][column] == Cell::Empty {
        // dbg!(current_row, column);
        // dbg!(grid[current_row as usize][column]);
        current_row -= 1;
    }

    if current_row >= 0 && grid[current_row as usize][column] == Cell::Empty {
        grid[current_row as usize][column] = grid[row][column];
        grid[row][column] = Cell::Empty;
        return current_row as usize;
    }

    row
}

fn roll_west_along_row(grid: &mut Vec<Vec<Cell>>, row: usize, column: usize) -> usize {
    let mut current_col = column as isize;

    while current_col > 0 && grid[row][(current_col - 1) as usize] == Cell::Empty {
        current_col -= 1;
    }

    if current_col >= 0 && grid[row][current_col as usize] == Cell::Empty {
        grid[row][current_col as usize] = grid[row][column];
        grid[row][column] = Cell::Empty;
        return current_col as usize;
    }

    column
}

fn roll_south_along_column(grid: &mut Vec<Vec<Cell>>, row: usize, column: usize) -> usize {
    let mut current_row = row;

    while current_row + 1 < grid.len() && grid[current_row + 1][column] == Cell::Empty {
        // dbg!(current_row, column);
        // dbg!(grid[current_row as usize][column]);
        current_row += 1;
    }

    if current_row < grid.len() && grid[current_row][column] == Cell::Empty {
        grid[current_row][column] = grid[row][column];
        grid[row][column] = Cell::Empty;
        return current_row;
    }

    row
}

fn roll_east_along_row(grid: &mut Vec<Vec<Cell>>, row: usize, column: usize) -> usize {
    let mut current_col = column;

    while current_col + 1 < grid.len() && grid[row][current_col + 1] == Cell::Empty {
        current_col += 1;
    }

    if current_col < grid.len() && grid[row][current_col] == Cell::Empty {
        grid[row][current_col] = grid[row][column];
        grid[row][column] = Cell::Empty;
        return current_col;
    }

    column
}

fn get_grid_state(grid: &[Vec<Cell>]) -> Vec<Pos> {
    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(j, c)| **c == Cell::Rounded)
                .map(move |(j, c)| (i, j))
        })
        .collect_vec()
}

fn print_grid(grid: &Vec<Vec<Cell>>) {
    for row in grid {
        for c in row.iter() {
            print!("{c}");
        }
        println!();
    }
}
