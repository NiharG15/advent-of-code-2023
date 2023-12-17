use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

type Pos = (usize, usize);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Node {
    position: Pos,
    cost: usize,
    direction: (i32, i32),
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

fn main() {
    // let input = include_str!("../../inputs/puzzle17_sample.txt");
    let input = include_str!("../../inputs/puzzle17.txt");

    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let cost = find_shortest_path_bfs(&grid, (0, 0), 1, 3);

    println!("-- Part 1 Ans: {cost}");

    let cost = find_shortest_path_bfs(&grid, (0, 0), 4, 10);

    println!("-- Part 2 Ans: {cost}");
}

fn find_shortest_path_bfs(grid: &[Vec<char>], source: Pos, min_dist: usize, max_dist: usize) -> usize {
    let max_x = grid.len();
    let max_y = grid[0].len();

    let mut dist: HashMap<(Pos, (i32, i32)), usize> = HashMap::new();
    let mut prev: HashMap<Pos, Pos> = HashMap::new();


    let mut heap = BinaryHeap::new();

    heap.push(Node {
        position: source,
        cost: 0,
        direction: (0, 0),
    });

    while let Some(Node {
        position,
        cost,
        direction,
    }) = heap.pop()
    {
        if position == (max_x - 1, max_y - 1) {
            return cost;
        }

        if dist.get(&(position, direction)).is_some_and(|&c| cost > c) {
            continue;
        }

        for (dx, dy) in [(0, -1), (0, 1), (1, 0), (-1, 0)] {

            if direction == (dx, dy) || direction == (-dx, -dy) {
                continue;
            }

            let mut next_cost = cost;

            for d in 1..=max_dist as i32 {
                let neighbor = ((position.0 as i32 + dx * d) as usize, (position.1 as i32 + dy * d) as usize);
                if neighbor.0 >= max_x || neighbor.1 >= max_y {
                    continue;
                }

                next_cost += grid[neighbor.0][neighbor.1].to_digit(10).unwrap() as usize;
                let next = Node {
                    position: neighbor,
                    cost: next_cost,
                    direction: (dx, dy),
                };

                if min_dist <= d as usize && next.cost < *dist.get(&(neighbor, (dx, dy))).unwrap_or(&usize::MAX) {
                    heap.push(next);
                    prev.entry(neighbor)
                        .and_modify(|e| *e = position)
                        .or_insert(position);
                    dist.insert((next.position, next.direction), next.cost);
                }
            }
        }
    }

    0
}
