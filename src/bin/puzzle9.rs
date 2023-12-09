use itertools::Itertools;

#[derive(Debug)]
struct Sequence {
    seq: Vec<Vec<i64>>,
}

impl Sequence {
    fn new(v: Vec<i64>) -> Sequence {
        let mut start = vec![v];
        loop {
            if start.last().unwrap().iter().all(|i| i == &0) {
                break;
            }

            start.push(successive_diff(start.last().unwrap()))
        }

        Sequence { seq: start }
    }

    fn predict_next_element(&mut self) -> i64 {
        self.seq.last_mut().unwrap().push(0);
        for i in (0..=(self.seq.len() - 2)).rev() {
            let element = self.seq[i].last().unwrap() + self.seq[i + 1].last().unwrap();
            self.seq[i].push(element);
        }

        *self.seq[0].last().unwrap()
    }
}

fn successive_diff(v: &[i64]) -> Vec<i64> {
    let mut res = Vec::with_capacity(v.len() - 1);
    for i in 1..v.len() {
        res.push(v[i] - v[i - 1]);
    }
    res
}

fn main() {
    // let input = include_str!("../../inputs/puzzle9_sample.txt");
    let input = include_str!("../../inputs/puzzle9.txt");

    let lines = input.lines().collect_vec();

    let ans: i64 = lines
        .iter()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect_vec()
        })
        .map(|v: Vec<i64>| {
            let mut s = Sequence::new(v);
            s.predict_next_element()
        })
        .sum();

    println!("-- Part 1 Ans: {}", ans);

    let ans: i64 = lines
        .iter()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect_vec()
        })
        .map(|v: Vec<i64>| {
            let mut s = Sequence::new(v.iter().rev().cloned().collect_vec());
            s.predict_next_element()
        })
        .sum();

    println!("-- Part 2 Ans: {}", ans);
}
