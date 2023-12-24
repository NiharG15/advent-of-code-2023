use std::ops::{Add, Mul};

use itertools::Itertools;
use z3::{Config, Context, SatResult, Solver};
use z3::ast::{Ast, Int};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Hailstone {
    x: i128,
    y: i128,
    z: i128,
    vx: i128,
    vy: i128,
    vz: i128,
}

impl From<&str> for Hailstone {
    fn from(value: &str) -> Self {
        let parts = value.split('@').collect_vec();
        let ((x, y, z), (vx, vy, vz)) = parts
            .iter()
            .map(|p| {
                p.trim()
                    .split(',')
                    .map(|c| c.trim().parse().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect_tuple()
            .unwrap();
        Hailstone {
            x,
            y,
            z,
            vx,
            vy,
            vz,
        }
    }
}

impl Hailstone {
    fn get_2d_line(&self) -> (i128, i128, i128, i128) {
        let x1 = self.x;
        let y1 = self.y;
        let x2 = self.x + self.vx;
        let y2 = self.y + self.vy;
        (x1, y1, x2, y2)
    }

    fn get_3d_line(&self) -> (i128, i128, i128, i128, i128, i128) {
        let x2 = self.x + self.vx;
        let y2 = self.y + self.vy;
        let z2 = self.z + self.vz;
        (self.x, self.y, self.z, x2, y2, z2)
    }

    fn point_of_2d_intersection(&self, other: &Hailstone) -> Option<(i128, i128)> {
        let (x1, y1, x2, y2) = self.get_2d_line();
        let (x3, y3, x4, y4) = other.get_2d_line();

        let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        if denominator == 0 {
            return None;
        }

        let px_n = (x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4);
        let py_n = (x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4);
        Some((px_n / denominator, py_n / denominator))
    }

    fn is_point_in_the_future_2d(&self, p: (i128, i128)) -> bool {
        let (px, py) = p;
        if px < self.x && self.vx > 0 {
            return false;
        }
        if px > self.x && self.vx < 0 {
            return false;
        }
        if py < self.y && self.vy > 0 {
            return false;
        }
        if py > self.y && self.vy < 0 {
            return false;
        }
        true
    }
}

// const BOUND_MIN: i128 = 7;
// const BOUND_MAX: i128 = 27;

const BOUND_MIN: i128 = 200000000000000;
const BOUND_MAX: i128 = 400000000000000;

fn main() {
    // let input = include_str!("../../inputs/puzzle24_sample.txt");
    let input = include_str!("../../inputs/puzzle24.txt");

    let lines = input.lines().collect_vec();

    let hailstones: Vec<Hailstone> = lines.into_iter().map(Into::into).collect_vec();

    let mut intersection_count_2d = 0;
    for i in 0..hailstones.len() - 1 {
        for j in (i + 1)..hailstones.len() {
            let (h1, h2) = (hailstones[i], hailstones[j]);
            if let Some(intersection) = h1.point_of_2d_intersection(&h2) {
                if intersection.0 >= BOUND_MIN
                    && intersection.0 <= BOUND_MAX
                    && intersection.1 >= BOUND_MIN
                    && intersection.1 <= BOUND_MAX
                    && h1.is_point_in_the_future_2d(intersection)
                    && h2.is_point_in_the_future_2d(intersection)
                {
                    intersection_count_2d += 1;
                }
            }
        }
    }

    println!("-- Part 1 Ans: {intersection_count_2d}");


    let context = Context::new(&Config::new());
    let solver = Solver::new(&context);

    let (x, y, z) = (Int::new_const(&context, "X"), Int::new_const(&context, "Y"), Int::new_const(&context, "Z"));
    let (vx, vy, vz) = (Int::new_const(&context, "VX"), Int::new_const(&context, "VY"), Int::new_const(&context, "VZ"));

    let zero = Int::from_i64(&context, 0);

    for (i, h) in hailstones.iter().enumerate() {
        let ti = Int::new_const(&context, format!("t_{}", i));
        let hx = Int::from_i64(&context, h.x as i64);
        let hy = Int::from_i64(&context, h.y as i64);
        let hz = Int::from_i64(&context, h.z as i64);
        let hvx = Int::from_i64(&context, h.vx as i64);
        let hvy = Int::from_i64(&context, h.vy as i64);
        let hvz = Int::from_i64(&context, h.vz as i64);
        solver.assert(&ti.gt(&zero));
        solver.assert(&(&x).add((&vx).mul(&ti))._eq(&hx.add(hvx.mul(&ti))));
        solver.assert(&(&y).add((&vy).mul(&ti))._eq(&hy.add(hvy.mul(&ti))));
        solver.assert(&(&z).add((&vz).mul(&ti))._eq(&hz.add(hvz.mul(&ti))));
    }

    assert_eq!(solver.check(), SatResult::Sat);

    let model = solver.get_model().unwrap();
    let (x, y, z) = (model.eval(&x, false).unwrap(), model.eval(&y, false).unwrap(), model.eval(&z, false).unwrap());

    println!("-- Part 2 Ans: {}", x.as_i64().unwrap() + y.as_i64().unwrap() + z.as_i64().unwrap());
}
