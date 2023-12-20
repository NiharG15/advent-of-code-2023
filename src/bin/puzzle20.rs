use std::collections::HashMap;
use std::ops::Not;
use itertools::Itertools;
use rust_aoc_2023::utils::lcm;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low
}

#[derive(Debug)]
enum Module {
    FlipFlop {
        name: String,
        state: bool,
        outputs: Vec<String>
    },
    Conjunction {
        name: String,
        state: bool,
        inputs: HashMap<String, Pulse>,
        outputs: Vec<String>
    },
    Broadcast {
        name: String,
        outputs: Vec<String>
    },
    Button,
    Output
}

impl Module {
    fn from(name: &str, outputs: Vec<String>) -> Self {
        match name {
            "broadcaster" => Self::Broadcast { name: name.to_owned(), outputs },
            s if s.starts_with('&') => Self::Conjunction {
                name: s.trim_start_matches('&').to_string(),
                state: false,
                inputs: HashMap::default(),
                outputs,
            },
            s if s.starts_with('%') => Self::FlipFlop {
                name: s.trim_start_matches('%').to_string(),
                state: false,
                outputs,
            },
            "output" => Self::Output,
            _ => unreachable!()
        }
    }

    fn outputs(&self) -> Vec<String> {
        match &self {
            Module::FlipFlop { name, state, outputs } => outputs.clone(),
            Module::Conjunction { name, state, inputs, outputs } => outputs.clone(),
            Module::Broadcast { name, outputs } => outputs.clone(),
            Module::Button => vec!["broadcaster".to_string()],
            Module::Output => vec![],
        }
    }

    fn name(&self) -> String {
        match &self {
            Module::FlipFlop { name, state, outputs } => name.clone(),
            Module::Conjunction { name, state, inputs, outputs } => name.clone(),
            Module::Broadcast { name, outputs } => name.clone(),
            Module::Button => "button".to_owned(),
            Module::Output => "output".to_owned()
        }
    }

    fn process_pulse(&mut self, input: &str, pulse: Pulse) -> Vec<(String, Pulse)> {
        match self {
            Module::FlipFlop { name, state, outputs } => {
                if let Pulse::Low = pulse {
                    *state = state.not();
                    let output_pulse = if *state { Pulse::High } else { Pulse::Low };
                    outputs.iter().map(|s| (s.clone(), output_pulse)).collect_vec()
                } else {
                    vec![]
                }
            }
            Module::Conjunction { name, state, inputs, outputs } => {
                inputs.insert(input.to_owned(), pulse);
                if inputs.values().all(|v| v == &Pulse::High) {
                    outputs.iter().map(|s| (s.clone(), Pulse::Low)).collect_vec()
                } else {
                    outputs.iter().map(|s| (s.clone(), Pulse::High)).collect_vec()

                }
            }
            Module::Broadcast { name, outputs } => {
                outputs.iter().map(|s| (s.clone(), pulse.clone())).collect_vec()
            }
            Module::Button => vec![("broadcaster".to_owned(), Pulse::Low)],
            Module::Output => vec![]
        }
    }
}

type ModulePulse = (String, Pulse);

fn main() {
    // let input = include_str!("../../inputs/puzzle20_sample.txt");
    let input = include_str!("../../inputs/puzzle20.txt");

    let lines = input.lines().collect_vec();

    let mut modules = HashMap::new();
    let mut name_to_outputs: HashMap<String, Vec<String>> = HashMap::new();

    for line in &lines {
        let (name, outputs) = line.split("->").map(str::trim).collect_tuple().unwrap();
        let outputs = outputs.split(',').map(str::trim).map(ToOwned::to_owned).collect_vec();
        let module = Module::from(name, outputs.clone());
        name_to_outputs.insert(module.name(), outputs.clone());
        modules.insert(module.name(), module);
    }

    for (name, outputs) in &name_to_outputs {
        for s in outputs {
            let module = modules.get_mut(s.as_str());
            if let Some(Module::Conjunction { name: _, state: _, inputs, outputs: _ }) = module {
                inputs.insert(name.clone(), Pulse::Low);
            }
        }
    }

    modules.insert("button".to_owned(), Module::Button);

    let mut count = 1;

    let (mut low_count, mut high_count) = (0, 0);

    dbg!(&modules.get("ql").unwrap());

    // todo: Make generic
    let mut mf = i32::MAX;
    let mut fz = i32::MAX;
    let mut ss = i32::MAX;
    let mut fh = i32::MAX;

    let mut mf_prev = -1;
    let mut fz_prev = -1;
    let mut ss_prev = -1;
    let mut fh_prev = -1;

    while count <= 100000 {
        let mut current_pulses: Vec<(String, String, Pulse)> = vec![("button".to_owned(), "broadcaster".to_owned(), Pulse::Low)];
        while !current_pulses.is_empty() {
            current_pulses.iter().map(|(i, o, p)| p).for_each(|p| {
                match p {
                    Pulse::High => high_count += 1,
                    Pulse::Low => low_count += 1,
                }
            });

            let mut next_pulses: Vec<(String, String, Pulse)> = vec![];
            for (i, o, p) in &current_pulses {
                if o == "output" {
                    continue
                }
                if i == "mf" && p == &Pulse::High {
                    if mf_prev == -1 {
                        mf_prev = count;
                    } else {
                        mf = mf.min(count - mf_prev);
                    }
                }
                if i == "fz" && p == &Pulse::High {
                    if fz_prev == -1 {
                        fz_prev = count;
                    } else {
                        fz = fz.min(count - fz_prev);
                    }
                }
                if i == "ss" && p == &Pulse::High {
                    if ss_prev == -1 {
                        ss_prev = count;
                    } else {
                        ss = ss.min(count - ss_prev);
                    }
                }
                if i == "fh" && p == &Pulse::High {
                    if fh_prev == -1 {
                        fh_prev = count;
                    } else {
                        fh = fh.min(count - fh_prev);
                    }
                }
                let module = modules.get_mut(o);
                if let Some(module) = module {
                    let outputs = module.process_pulse(i, *p).into_iter().map(|(m, pulse)| (o.clone(), m, pulse)).collect_vec();
                    next_pulses.extend_from_slice(&outputs[..]);
                }
            }
            current_pulses.clear();
            current_pulses = next_pulses;
        }

        if count == 1000 {
            dbg!(high_count, low_count);
            println!("-- Part 1 Ans: {}", low_count * high_count);
        }

        count += 1;
    }

    let part2 = lcm(&[mf as u64, fz as u64, ss as u64, fh as u64]);
    println!("-- Part 2 Ans: {part2}");
}