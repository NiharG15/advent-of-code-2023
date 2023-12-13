use std::cmp::Ordering::{Equal, Greater, Less};
use itertools::Itertools;
use rust_aoc_2023::utils;
use utils::print_grid;

fn main() {
    // let input = include_str!("../../inputs/puzzle13_sample.txt");
    let input = include_str!("../../inputs/puzzle13.txt");

    let patterns = input.split("\n\n").collect_vec();

    let mut col_sum = 0;
    let mut row_sum = 0;

    let mut smudge_col_sum = 0;
    let mut smudge_row_sum = 0;

    for p in &patterns {
        let grid = p.lines().map(|l| l.chars().collect_vec()).collect_vec();

        print_grid(&grid);
        let col = find_column_mirrored(&grid);
        println!("Mirrored at col: {col:?}");
        if let Some(c) = col {
            col_sum += c + 1;
        }

        let col_smudge = find_column_mirrored_with_smudge(&grid);
        println!("Mirrored with smudge at col: {col_smudge:?}");
        if let Some(c) = col_smudge {
            smudge_col_sum += c + 1;
        }


        let row = find_row_mirrored(&grid);
        println!("Mirrored at row: {row:?}");
        if let Some(r) = row {
            row_sum += r + 1;
        }

        let row_smudge = find_row_mirrored_with_smudge(&grid);
        println!("Mirrored with smudge at row: {row_smudge:?}");
        if let Some(r) = row_smudge {
            smudge_row_sum += r + 1;
        }
    }

    println!("-- Part 1 Ans: {}", col_sum + 100 * row_sum);
    println!("-- Part 2 Ans: {}", smudge_col_sum + 100 * smudge_row_sum);
}

fn find_column_mirrored_with_smudge(grid: &[Vec<char>]) -> Option<usize> {
    let cols = grid[0].len();

    for j in 0..cols - 1 {
        let mut l = j as isize;
        let mut r = j + 1;
        let mut nequal_count = 0;
        let mut smudge_found = false;
        while l >= 0 && r < cols {
            let equal_with_smudge = cols_equal_with_smudge(grid, l as usize, r);
            if !equal_with_smudge.0 {
                nequal_count += 1;
            }
            if equal_with_smudge.1.is_some() {
                smudge_found = true;
            }
            l -= 1;
            r += 1;
        }

        if nequal_count == 1 && smudge_found {
            return Some(j);
        }
    }

    None
}

fn find_column_mirrored(grid: &[Vec<char>]) -> Option<usize> {
    let cols = grid[0].len();

    for j in 0..cols - 1 {
        let mut l = j as isize;
        let mut r = j + 1;
        let mut mirrored = true;
        'w: while l >= 0 && r < cols {
            let equal_with_smudge = cols_equal_with_smudge(grid, l as usize, r);
            if !equal_with_smudge.0 {
                mirrored = false;
                break 'w;
            }
            l -= 1;
            r += 1;
        }

        if mirrored {
            return Some(j);
        }
    }

    None
}

fn find_row_mirrored_with_smudge(grid: &[Vec<char>]) -> Option<usize> {
    let rows = grid.len();

    for i in 0..rows - 1 {
        let mut t = i as isize;
        let mut b = i + 1;
        let mut nequal_count = 0;
        let mut smudge_found = false;
        while t >= 0 && b < rows {
            let equal_with_smudge = rows_equal_with_smudge(grid, t as usize, b);
            if !equal_with_smudge.0 {
                nequal_count += 1;
            }

            if equal_with_smudge.1.is_some() {
                smudge_found = true;
            }

            t -= 1;
            b += 1;
        }

        if nequal_count == 1 && smudge_found {
            return Some(i);
        }
    }

    None
}

fn find_row_mirrored(grid: &[Vec<char>]) -> Option<usize> {
    let rows = grid.len();

    for i in 0..rows - 1 {
        let mut t = i as isize;
        let mut b = i + 1;
        let mut mirrored = true;
        'w: while t >= 0 && b < rows {
            let equal_with_smudge = rows_equal_with_smudge(grid, t as usize, b);
            if !equal_with_smudge.0 {
                mirrored = false;
                break 'w;
            }

            t -= 1;
            b += 1;
        }

        if mirrored {
            return Some(i);
        }
    }

    None
}

fn cols_equal_with_smudge(grid: &[Vec<char>], c1: usize, c2: usize) -> (bool, Option<usize>) {
    let mut nequal_count = 0;
    let mut nequal_indices = vec![];
    for (i, row) in grid.iter().enumerate() {
        if row[c1] != row[c2] {
            nequal_count += 1;
            nequal_indices.push(i);
        }
    }

    match nequal_count.cmp(&1) {
        Equal => (false, Some(nequal_indices[0])),
        Greater => (false, None),
        Less => (true, None)
    }
}

fn rows_equal_with_smudge(grid: &[Vec<char>], r1: usize, r2: usize) -> (bool, Option<usize>) {
    let row1 = &grid[r1];
    let row2 = &grid[r2];
    let mut nequal_count = 0;
    let mut nequal_indices = vec![];

    for ((i, a), b) in row1.iter().enumerate().zip(row2.iter()) {
        if a != b {
            nequal_count += 1;
            nequal_indices.push(i);
        }
    }

    match nequal_count.cmp(&1) {
        Equal => (false, Some(nequal_indices[0])),
        Greater => (false, None),
        Less => (true, None)
    }
}