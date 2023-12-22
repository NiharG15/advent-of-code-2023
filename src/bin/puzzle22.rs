use std::cmp::{max, min};
use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Coord2 {
    x: usize,
    y: usize,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

impl Coord {
    fn xy(&self) -> Coord2 {
        Coord2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl From<&str> for Coord {
    fn from(value: &str) -> Self {
        let (x, y, z) = value
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Coord { x, y, z }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Brick {
    start: Coord,
    end: Coord,
}

impl Brick {
    fn get_all_xy_coords(&self) -> Vec<Coord2> {
        let mut result = vec![];
        let start_xy = self.start.xy();
        let end_xy = self.end.xy();
        let (low_y, high_y) = (min(start_xy.y, end_xy.y), max(start_xy.y, end_xy.y));
        let (low_x, high_x) = (min(start_xy.x, end_xy.x), max(start_xy.x, end_xy.x));
        for x in low_x..=high_x {
            for y in low_y..=high_y {
                result.push(Coord2 { x, y });
            }
        }
        result
    }
}

impl From<&str> for Brick {
    fn from(value: &str) -> Self {
        let (start, end) = value.split('~').collect_tuple().unwrap();
        Brick {
            start: start.into(),
            end: end.into(),
        }
    }
}

fn main() {
    // let input = include_str!("../../inputs/puzzle22_sample.txt");
    let input = include_str!("../../inputs/puzzle22.txt");

    let lines = input.lines().collect_vec();

    let mut bricks: Vec<Brick> = lines.into_iter().map(Into::into).collect_vec();

    bricks.sort_by(|b1, b2| b1.start.z.cmp(&b2.start.z));

    let max_x = bricks
        .iter()
        .map(|b| max(b.start.x, b.end.x))
        .max()
        .unwrap();
    let max_y = bricks
        .iter()
        .map(|b| max(b.start.y, b.end.y))
        .max()
        .unwrap();

    let (collapsed_bricks, _) = drop_bricks(&bricks, max_x, max_y);

    let mut collapse_count_sum = 0;
    let mut no_collapse_count_sum = 0;
    for b in &collapsed_bricks {
        let mut new_bricks = collapsed_bricks.clone();
        new_bricks.remove(new_bricks.iter().position(|x| x == b).unwrap());

        let (_, collapse_count) = drop_bricks(&new_bricks, max_x, max_y);
        collapse_count_sum += collapse_count;
        if collapse_count == 0 {
            no_collapse_count_sum += 1;
        }
    }

    println!("-- Part 1 Ans: {no_collapse_count_sum}");
    println!("-- Part 2 Ans: {collapse_count_sum}");
}

fn drop_bricks(bricks: &Vec<Brick>, max_x: usize, max_y: usize) -> (Vec<Brick>, i32) {
    let mut collapsed_bricks = vec![];
    let mut collapsed_count = 0;
    let mut tallest_so_far_map: HashMap<Coord2, usize> = HashMap::new();

    for i in 0..=max_x {
        for j in 0..=max_y {
            tallest_so_far_map.insert(Coord2 { x: i, y: j }, 0);
        }
    }

    for b in bricks {
        let xy_coords = b.get_all_xy_coords();
        let tallest_z_so_far = xy_coords
            .iter()
            .filter_map(|xy| tallest_so_far_map.get(xy).copied())
            .max()
            .unwrap();
        let new_z_delta = min(b.start.z, b.end.z) - tallest_z_so_far - 1;
        let collapsed_brick = Brick {
            start: Coord {
                z: b.start.z.wrapping_sub(new_z_delta),
                ..b.start
            },
            end: Coord {
                z: b.end.z.wrapping_sub(new_z_delta),
                ..b.end
            },
        };
        if collapsed_brick.start.z != b.start.z {
            collapsed_count += 1;
        }
        for xy in &xy_coords {
            tallest_so_far_map.insert(*xy, max(collapsed_brick.start.z, collapsed_brick.end.z));
        }
        collapsed_bricks.push(collapsed_brick);
    }
    (collapsed_bricks, collapsed_count)
}
