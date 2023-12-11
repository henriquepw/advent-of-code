use std::{cmp, fs};

struct Map {
    to: u64,
    from: u64,
    range: u64,
}

fn get_maps<'a>(chunks: impl Iterator<Item = &'a str>) -> Vec<Vec<Map>> {
    chunks
        .map(|chunk| {
            let mut map = Vec::new();

            chunk.split("\n").skip(1).for_each(|line| {
                let values = line
                    .split(" ")
                    .map(|l| l.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();

                map.push(Map {
                    to: values[0],
                    from: values[1],
                    range: values[2],
                });
            });

            map
        })
        .collect()
}

fn get_value(from: u64, list: &Vec<Map>) -> u64 {
    for map in list {
        if from >= map.from && from < map.from + map.range {
            return map.to + (from - map.from);
        }
    }

    from.clone()
}

fn part_1(input: &String) -> u64 {
    let mut chunks = input.trim_end().split("\n\n");
    let seeds = chunks.next().unwrap().replace("seeds: ", "");
    let maps = get_maps(chunks);

    seeds
        .split(" ")
        .map(|s| maps.iter().fold(s.parse().unwrap(), get_value))
        .min()
        .unwrap()
}

struct Seed {
    start: u64,
    range: u64,
}

fn get_range(list: &Vec<Map>, from: u64, prev_range: u64) -> (u64, u64) {
    for map in list {
        if from >= map.from && from < map.from + map.range {
            let offset = from - map.from;
            let to = map.to + offset;
            let range = map.range - offset;

            if prev_range == 0 {
                return (to, range);
            }

            return (to, cmp::min(prev_range, range));
        }
    }

    (from.clone(), prev_range)
}

fn part_2(input: &String) -> u64 {
    let mut chunks = input.trim_end().split("\n\n");

    let seed_str = chunks.next().unwrap().replace("seeds: ", "");
    let items: Vec<&str> = seed_str.split(" ").collect();
    let seeds = (1..items.len()).step_by(2).map(|i| Seed {
        start: items[i - 1].parse().unwrap(),
        range: items[i].parse().unwrap(),
    });

    let maps = get_maps(chunks);

    seeds
        .map(|seed| {
            let mut result = 18446744073709551615;
            let mut i = seed.start;
            while i <= seed.start + seed.range - 1 {
                let (value, range) = maps.iter().fold((i, 0), |(from, prev_range), list| {
                    get_range(list, from, prev_range)
                });

                if result > value {
                    result = value;
                }

                i += range;
            }

            result
        })
        .min()
        .unwrap()
}

fn main() {
    let input = fs::read_to_string("src/inputs/05").expect("input not found");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
