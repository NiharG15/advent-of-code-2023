/// Returns the 8 neighbors of a given x, y coordinate without any bounds checks.
///
/// # Examples
///
/// ```
/// use rust_aoc_2023::utils::iter_neighbors;
///
/// assert_eq!(iter_neighbors(0, 0), [(0, 1), (1, 0), (0, -1), (-1, 0), (-1, -1), (1, 1), (-1, 1), (1, -1)]);
/// ```
pub fn iter_neighbors(x: i32, y: i32) -> [(i32, i32); 8] {
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
/// use rust_aoc_2023::utils::iter_neighbors_bounded;
///
/// let result = iter_neighbors_bounded(5, 5, 6, 8);
/// assert_eq!(result, vec![(5, 6), (5, 4), (4, 5), (4, 4), (4, 6)]);
/// ```
pub fn iter_neighbors_bounded(x: i32, y: i32, max_x: i32, max_y: i32) -> Vec<(i32, i32)> {
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
    .filter(|(x,  y)| {
        x >= &0 && x < &max_x && y >= &0 && y < &max_y
    }).collect()
}
