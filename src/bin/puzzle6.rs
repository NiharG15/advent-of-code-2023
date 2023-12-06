use itertools::Itertools;

fn main() {
    // let input = include_str!("../../inputs/puzzle6_sample.txt");
    let input = include_str!("../../inputs/puzzle6.txt");

    let lines = input.lines().collect_vec();

    let [ref times, ref distances] = lines
        .iter()
        .map(|s| {
            s.split(':')
                .nth(1)
                .unwrap()
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect_vec()
        })
        .collect_vec()[..]
    else {
        unreachable!()
    };

    let ways_to_beat = calc_ways_to_beat(times, distances);

    let ans1: u128 = ways_to_beat.iter().product();

    println!("-- Part 1 Answer: {}", ans1);

    let [time, distance] = lines
        .iter()
        .map(|s| {
            s.split(':')
                .nth(1)
                .unwrap()
                .replace(' ', "")
                .parse()
                .unwrap()
        })
        .collect_vec()[..]
    else {
        unreachable!()
    };

    let ways_to_beat = calc_ways_to_beat(&[time], &[distance]);

    let ans2: u128 = ways_to_beat.iter().product();
    println!("-- Part 2 Answer: {}", ans2);
}

fn calc_ways_to_beat(times: &[u128], distances: &[u128]) -> Vec<u128> {
    let mut ways_to_beat: Vec<u128> = vec![];

    for (i, &t) in times.iter().enumerate() {
        let ds: Vec<u128> = time_to_distances(t);

        ways_to_beat.push(ds.iter().filter(|&d| d > &distances[i]).count() as u128);
    }
    ways_to_beat
}

fn time_to_distances(time: u128) -> Vec<u128> {
    (1..time)
        .map(|t| {
            let accel = t;
            let time_rem = time - t;
            accel * time_rem
        })
        .collect()
}
