use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref WORKFLOW_PAT: Regex = Regex::new("(\\w+)\\{(.*)\\}").unwrap();
    static ref PART_PAT: Regex = Regex::new("\\{x=(\\d+),m=(\\d+),a=(\\d+),s=(\\d+)}").unwrap();
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum Quality {
    X,
    M,
    A,
    S,
}

impl From<&str> for Quality {
    fn from(value: &str) -> Self {
        match value {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum Operation {
    GreaterThan,
    LessThan,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            ">" => Self::GreaterThan,
            "<" => Self::LessThan,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum Condition {
    Simple(String),
    Complex {
        q: Quality,
        op: Operation,
        threshold: usize,
        result: String,
    },
}

impl Condition {
    fn evaluate(&self, part: &Part) -> bool {
        match self {
            Condition::Simple(_result) => true,
            Condition::Complex {
                q,
                op,
                threshold,
                result: _result,
            } => {
                let quality = match q {
                    Quality::X => part.x,
                    Quality::M => part.m,
                    Quality::A => part.a,
                    Quality::S => part.s,
                };
                match op {
                    Operation::GreaterThan => quality > *threshold,
                    Operation::LessThan => quality < *threshold,
                }
            }
        }
    }

    fn evaluate_set(&self, ps: &PartSet) -> (Option<PartSet>, PartSet, String) {
        match &self {
            &Condition::Complex {
                q,
                op,
                threshold,
                result,
            } => {
                let new_part_set = match q {
                    Quality::X => {
                        let (m, a, s) = (ps.m, ps.a, ps.s);
                        let x = match op {
                            Operation::GreaterThan => ((ps.x.0, *threshold), (*threshold + 1, ps.x.1)),
                            Operation::LessThan => ((*threshold, ps.x.1), (ps.x.0, *threshold - 1)),
                        };
                        (PartSet { x: x.0, m, a, s }, PartSet { x: x.1, m, a, s })
                    }
                    Quality::M => {
                        let (x, a, s) = (ps.x, ps.a, ps.s);
                        let m = match op {
                            Operation::GreaterThan => ((ps.m.0, *threshold), (*threshold + 1, ps.m.1)),
                            Operation::LessThan => ((*threshold, ps.m.1), (ps.m.0, *threshold - 1))
                        };
                        (PartSet { x, m: m.0, a, s }, PartSet { x, m: m.1, a, s })
                    }
                    Quality::A => {
                        let (x, m, s) = (ps.x, ps.m, ps.s);
                        let a = match op {
                            Operation::GreaterThan => ((ps.a.0, *threshold), (*threshold + 1, ps.a.1)),
                            Operation::LessThan => ((*threshold, ps.a.1), (ps.a.0, *threshold - 1)),
                        };
                        (PartSet { x, m, a: a.0, s }, PartSet { x, m, a: a.1, s })
                    }
                    Quality::S => {
                        let (x, m, a) = (ps.x, ps.m, ps.a);
                        let s = match op {
                            Operation::GreaterThan => ((ps.s.0, *threshold), (*threshold + 1, ps.s.1)),
                            Operation::LessThan => ((*threshold, ps.s.1), (ps.s.0, *threshold - 1))
                        };
                        (PartSet { x, m, a, s: s.0 }, PartSet { x, m, a, s: s.1 })
                    }
                };
                (Some(new_part_set.0), new_part_set.1, result.clone())
            }
            Condition::Simple(result) => (None, *ps, result.clone()),
        }
    }
}

impl From<&str> for Condition {
    fn from(value: &str) -> Self {
        if value.contains('>') {
            let parts = value.split('>').collect_vec();
            let (threshold, result) = parts[1].split(':').collect_tuple().unwrap();
            return Self::Complex {
                q: parts[0].into(),
                op: Operation::GreaterThan,
                threshold: threshold.parse().unwrap(),
                result: result.to_owned(),
            };
        } else if value.contains('<') {
            let parts = value.split('<').collect_vec();
            let (threshold, result) = parts[1].split(':').collect_tuple().unwrap();
            return Self::Complex {
                q: parts[0].into(),
                op: Operation::LessThan,
                threshold: threshold.parse().unwrap(),
                result: result.to_owned(),
            };
        }

        Self::Simple(value.to_owned())
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Workflow {
    name: &'static str,
    conditions: Vec<Condition>,
}

impl Workflow {
    fn evaluate(&self, part: &Part) -> String {
        for c in &self.conditions {
            if c.evaluate(part) {
                return match c {
                    Condition::Simple(r) => r.clone(),
                    Condition::Complex {
                        q: _q,
                        op: _op,
                        threshold: _threshold,
                        result,
                    } => result.clone(),
                };
            }
        }

        unreachable!()
    }

    fn evaluate_set(&self, ps: &PartSet) -> Vec<(PartSet, String)> {
        let mut remaining = *ps;
        let mut result_parts = vec![];
        for c in &self.conditions {
            let (rem, success, success_result) = c.evaluate_set(&remaining);
            result_parts.push((success, success_result));
            if let Some(r) = rem {
                remaining = r;
            } else {
                break;
            }
        }

        result_parts
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Debug, Copy, Clone)]
struct PartSet {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl PartSet {
    fn get_count(&self) -> usize {
        (self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1)
    }
}

fn main() {
    // let input = include_str!("../../inputs/puzzle19_sample.txt");
    let input = include_str!("../../inputs/puzzle19.txt");

    let (workflows, parts) = input.split("\n\n").collect_tuple().unwrap();

    let mut workflow_map: HashMap<String, Workflow> = HashMap::new();

    for line in workflows.lines() {
        let parsed = WORKFLOW_PAT.captures(line).unwrap();
        let name = parsed.get(1).unwrap().as_str();
        let workflow = Workflow {
            name,
            conditions: parsed
                .get(2)
                .unwrap()
                .as_str()
                .split(',')
                .map(std::convert::Into::into)
                .collect_vec(),
        };
        workflow_map.insert(name.to_string(), workflow);
    }

    let mut parts_list: Vec<Part> = vec![];

    for line in parts.lines() {
        let parsed = PART_PAT.captures(line).unwrap();
        parts_list.push(Part {
            x: parsed.get(1).unwrap().as_str().parse().unwrap(),
            m: parsed.get(2).unwrap().as_str().parse().unwrap(),
            a: parsed.get(3).unwrap().as_str().parse().unwrap(),
            s: parsed.get(4).unwrap().as_str().parse().unwrap(),
        });
    }

    let mut accepted_parts = vec![];
    for p in &parts_list {
        let mut curr_result = workflow_map.get("in").unwrap().evaluate(p);
        loop {
            let w = workflow_map.get(&curr_result).unwrap();
            curr_result = w.evaluate(p);

            if curr_result == "A" || curr_result == "R" {
                break;
            }
        }

        if curr_result == "A" {
            accepted_parts.push(*p);
        }
    }

    let sum: usize = accepted_parts.iter().map(|p| p.x + p.m + p.a + p.s).sum();

    println!("-- Part 1 Ans: {sum}");

    let mut curr_part_sets = vec![(
        PartSet {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        },
        "in".to_owned(),
    )];

    let mut sum2: usize = 0;

    while !curr_part_sets.is_empty() {
        let mut next_part_sets = vec![];

        for psr in &curr_part_sets {
            let w = workflow_map.get(&psr.1).unwrap();
            next_part_sets.extend_from_slice(&w.evaluate_set(&psr.0));
        }
        curr_part_sets.clear();
        for psr in &next_part_sets {
            dbg!(psr);
            if psr.1 == "A" {
                sum2 += psr.0.get_count();
            }

            if psr.1 != "A" && psr.1 != "R" {
                curr_part_sets.push(psr.clone());
            }
        }
    }

    println!("-- Part 2 Ans: {sum2}");
}
