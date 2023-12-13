use std::ops::{Div, Mul, Rem};
use itertools::Itertools;
use num_traits::Zero;

/// Returns the 8 neighbors of a given x, y coordinate without any bounds checks.
///
/// # Examples
///
/// ```
/// use rust_aoc_2023::utils::iter_neighbors8;
///
/// assert_eq!(iter_neighbors8(0, 0), [(0, 1), (1, 0), (0, -1), (-1, 0), (-1, -1), (1, 1), (-1, 1), (1, -1)]);
/// ```
pub fn iter_neighbors8(x: i32, y: i32) -> [(i32, i32); 8] {
    [
        (0i32, 1),
        (1, 0),
        (0, -1i32),
        (-1, 0),
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
    ]
        .map(|(di, dj)| (x + di, y + dj))
}

/// Returns the (upto) 8 neighbors of a given x, y coordinate with bounds
///   checks against `max_x` and `max_y` along with 0, filtering out any neighbors that are out of bounds.
///
/// # Examples
///
/// ```
/// use rust_aoc_2023::utils::iter_neighbors_bounded8;
///
/// let result = iter_neighbors_bounded8(5, 5, 6, 8);
/// assert_eq!(result, vec![(5, 6), (5, 4), (4, 5), (4, 4), (4, 6)]);
/// ```
pub fn iter_neighbors_bounded8(x: i32, y: i32, max_x: i32, max_y: i32) -> Vec<(i32, i32)> {
    [
        (0i32, 1),
        (1, 0),
        (0, -1i32),
        (-1, 0),
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
    ]
        .map(|(di, dj)| (x + di, y + dj))
        .into_iter()
        .filter(|(x, y)| {
            x >= &0 && x < &max_x && y >= &0 && y < &max_y
        }).collect()
}


/// Calculate LCM of a list of numbers
///
/// # Examples
///
/// ```
/// use rust_aoc_2023::utils::lcm;
///
/// assert_eq!(lcm(&[1, 3, 5]), 15);
/// assert_eq!(lcm(&[1.0, 3.0, 4.0]), 12.0);
/// ```
pub fn lcm<T>(nums: &[T]) -> T
    where
        T: Copy + PartialEq + Mul<Output=T> + Div<Output=T> + Zero + Rem<Output=T>
{
    if nums.len() == 1 {
        return nums[0];
    }

    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}


/// Calculate the GCD of two numbers.
///
/// # Examples
///
/// ```
/// use rust_aoc_2023::utils::gcd;
///
/// assert_eq!(gcd(15, 18), 3);
/// ```
pub fn gcd<T>(a: T, b: T) -> T
    where
        T: Rem<Output=T> + Copy + Zero
{
    if b.is_zero() {
        return a;
    }
    gcd(b, a % b)
}

/// Iterate 4 neighbors around point (x, y).
///
/// # Examples
///
/// ```
/// use rust_aoc_2023::utils::iter_neighbors4;
///
/// assert_eq!(iter_neighbors4(1, 1), [(1, 2), (1, 0), (0, 1), (2, 1)]);
/// ```
pub fn iter_neighbors4(x: i64, y: i64) -> [(i64, i64); 4] {
    [
        (0, 1),
        (0, -1),
        (-1, 0),
        (1, 0),
    ]
        .map(|(dx, dy)| (x + dx, y + dy))
}

/// Iterate 4 neighbors around point (x, y) with bounds check.
///
/// # Examples
///
/// ```
/// use rust_aoc_2023::utils::iter_neighbors4_bounded;
///
/// assert_eq!(iter_neighbors4_bounded(1, 1,  4, 2), [(1, 0), (0, 1), (2, 1)]);
/// ```
pub fn iter_neighbors4_bounded(x: i64, y: i64, max_x: i64, max_y: i64) -> Vec<(i64, i64)> {
    iter_neighbors4(x, y)
        .into_iter()
        .filter(|(nx, ny)| *nx >= 0 && *nx < max_x && *ny >= 0 && *ny < max_y)
        .collect_vec()
}

/// Print a grid to stdout. Assumes that an mxn grid is passed in.
pub fn print_grid(grid: &[Vec<char>]) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            print!("{}", grid[i][j]);
        }
        println!();
    }
}
