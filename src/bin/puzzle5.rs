fn main() {
    // let input = include_str!("../../inputs/puzzle5_sample.txt");
    let input = include_str!("../../inputs/puzzle5.txt");

    let lines = input.split("\n\n").collect::<Vec<_>>();

    let seeds: Vec<i64> = lines[0]
        .split(':')
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let seed_to_soil_entries = entries_to_map(&string_to_entries(lines[1]));
    let soil_to_fertilizer_entries = entries_to_map(&string_to_entries(lines[2]));
    let fertilizer_to_water_entries = entries_to_map(&string_to_entries(lines[3]));
    let water_to_light_entries = entries_to_map(&string_to_entries(lines[4]));
    let light_to_temp_entries = entries_to_map(&string_to_entries(lines[5]));
    let temp_to_humid_entries = entries_to_map(&string_to_entries(lines[6]));
    let humid_to_location_entries = entries_to_map(&string_to_entries(lines[7]));

    let part_1_ans = seeds
        .iter()
        .map(|seed| {
            let soil = seed_to_soil_entries.get(*seed);
            let fert = soil_to_fertilizer_entries.get(soil);
            let water = fertilizer_to_water_entries.get(fert);
            let light = water_to_light_entries.get(water);
            let temp = light_to_temp_entries.get(light);
            let humid = temp_to_humid_entries.get(temp);
            humid_to_location_entries.get(humid)
        })
        .min()
        .unwrap();

    println!("-- Part 1 Answer: {part_1_ans}");

    let part_2_ans = &seeds
        .chunks(2)
        .flat_map(|seed| {
            let seed_start = seed[0];
            let seed_range = seed[1];
            let soils = seed_to_soil_entries.get_ranged(seed_start, seed_range);
            let ferts = soils
                .iter()
                .flat_map(|&(ss, sr)| soil_to_fertilizer_entries.get_ranged(ss, sr))
                .collect::<Vec<_>>();
            let waters = ferts
                .iter()
                .flat_map(|&(fs, fr)| fertilizer_to_water_entries.get_ranged(fs, fr))
                .collect::<Vec<_>>();
            let lights = waters
                .iter()
                .flat_map(|&(ws, wr)| water_to_light_entries.get_ranged(ws, wr))
                .collect::<Vec<_>>();
            let temps = lights
                .iter()
                .flat_map(|&(ls, lr)| light_to_temp_entries.get_ranged(ls, lr))
                .collect::<Vec<_>>();
            let humids = temps
                .iter()
                .flat_map(|&(ts, tr)| temp_to_humid_entries.get_ranged(ts, tr))
                .collect::<Vec<_>>();
            humids
                .iter()
                .flat_map(|&(hs, hr)| humid_to_location_entries.get_ranged(hs, hr))
                .collect::<Vec<_>>()
        })
        .min_by_key(|v| v.0)
        .unwrap();

    println!("-- Part 2 Answer: {}", part_2_ans.0);
}

fn string_to_entries(line: &str) -> Vec<Vec<i64>> {
    line.lines().collect::<Vec<_>>()[1..]
        .iter()
        .map(|s| {
            s.split_ascii_whitespace()
                .map(|s1| s1.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn entries_to_map(entries: &Vec<Vec<i64>>) -> RangedMapList {
    let mut maps = vec![];

    for entry in entries {
        let source_start = entry[1];
        let destination_start = entry[0];
        let range = entry[2];
        maps.push(RangedMap {
            source_start,
            destination_start,
            range,
        });
    }

    RangedMapList::new_sorted(maps)
}

#[derive(Debug)]
struct RangedMap {
    source_start: i64,
    destination_start: i64,
    range: i64,
}

impl RangedMap {
    fn get(&self, val: i64) -> Option<i64> {
        if val >= self.source_start && val < self.source_start + self.range {
            return Some(self.destination_start + (val - self.source_start));
        }
        None
    }
}

struct RangedMapList {
    maps: Vec<RangedMap>,
}

impl RangedMapList {
    fn new_sorted(mut maps: Vec<RangedMap>) -> RangedMapList {
        maps.sort_by(|m1, m2| m1.source_start.cmp(&m2.source_start));
        RangedMapList { maps }
    }

    fn get(&self, val: i64) -> i64 {
        self.maps
            .iter()
            .map(|m| m.get(val))
            .filter(Option::is_some)
            .nth(0)
            .unwrap_or(Some(val))
            .unwrap()
    }

    fn get_ranged(&self, val_start: i64, range: i64) -> Vec<(i64, i64)> {
        let mut new_ranges = vec![];
        let mut curr_val = val_start;
        let val_max = val_start + range - 1;
        dbg!(val_start); dbg!(range);
        for m in &self.maps {
            dbg!(&m);
            let source_max = m.source_start + m.range - 1;
            if curr_val > source_max {
                // Current value is greater than this maps max, skip it.
                continue;
            }
            if m.source_start > val_max || curr_val > val_max {
                // Explored the entire range, break.
                break;
            }
            if m.source_start > curr_val {
                // Range or part of it, starts before this map.
                new_ranges.push((curr_val, m.source_start - curr_val + 1));
            }

            let delta = dbg!(m.destination_start - m.source_start);
            let new_val_start = dbg!(curr_val.max(m.source_start) + delta);
            let new_val_max = dbg!(val_max.min(source_max) + delta);
            new_ranges.push((new_val_start, new_val_max - new_val_start + 1));
            dbg!(&new_ranges);
            curr_val = dbg!(source_max.min(val_max) + 1);
        }

        if curr_val < val_max {
            new_ranges.push((curr_val, val_max - curr_val + 1));
        }

        new_ranges
    }
}
