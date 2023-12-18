use std::isize;

use itertools::Itertools;
use tap::Pipe;

#[derive(Debug)]
struct Pos(isize, isize);

fn main() {
    // let input = include_str!("../../inputs/puzzle18_sample.txt");
    let input = include_str!("../../inputs/puzzle18.txt");

    let lines = input.lines().collect_vec();

    let mut current_pos = Pos(0, 0);
    let mut sum1 = 0;
    let mut sum2 = 0;
    let mut sum_dir = 0;

    let mut current_pos_corr = Pos(0, 0);
    let mut sum1_corr = 0;
    let mut sum2_corr = 0;
    let mut sum_dir_corr = 0;

    for line in lines {
        let (dir, count, color) = line.split_ascii_whitespace().collect_tuple().unwrap();
        let count: isize = count.parse().unwrap();
        let (corrected_count, direction): (isize, usize) = color
            .strip_prefix("(#")
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .pipe(|c| c.split_at(5))
            .pipe(|(hex_len, dir)| {
                (
                    isize::from_str_radix(hex_len, 16).unwrap(),
                    dir.parse().unwrap(),
                )
            });

        let orig_direction = match dir {
            "R" => (1isize, 0isize),
            "L" => (-1, 0),
            "D" => (0, -1),
            "U" => (0, 1),
            _ => unreachable!(),
        };

        let next_pos = Pos(
            current_pos.0 + orig_direction.0 * count,
            current_pos.1 + orig_direction.1 * count,
        );
        // Sum_n(x^iy^i+1 - x^i+1y^i)
        sum1 += current_pos.0 * next_pos.1;
        sum2 += current_pos.1 * next_pos.0;
        sum_dir += count;
        current_pos = next_pos;

        let corr_direction = match direction {
            0 => (1isize, 0isize),
            1 => (0, 1),
            2 => (-1, 0),
            3 => (0, -1),
            _ => unreachable!(),
        };

        let next_pos_corr = Pos(
            current_pos_corr.0 + corr_direction.0 * corrected_count,
            current_pos_corr.1 + corr_direction.1 * corrected_count,
        );
        sum1_corr += current_pos_corr.0 * next_pos_corr.1;
        sum2_corr += current_pos_corr.1 * next_pos_corr.0;
        sum_dir_corr += corrected_count;
        current_pos_corr = next_pos_corr;
    }

    let area1 = (sum1 - sum2).abs() / 2;
    println!("-- Part 1 Ans: {}", area1 + sum_dir / 2 + 1);

    let area2 = (sum1_corr - sum2_corr).abs() / 2;
    println!("-- Part 2 Ans: {}", area2 + sum_dir_corr / 2 + 1);
}
