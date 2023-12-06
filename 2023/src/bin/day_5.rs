use std::{collections::HashMap, fs};

struct Map {
    destination: u64,
    source: u64,
    range: u64,
}

fn get_map_value(maps: &HashMap<String, Vec<Map>>, map_name: &str, from: &u64) -> u64 {
    let list = maps.get(map_name).unwrap();
    let mut to = from.clone();

    for map in list {
        if *from >= map.source && *from < map.source + map.range {
            to = map.destination + (from - map.source);
            break;
        }
    }

    to
}

fn part_1(input: &String) -> u64 {
    let mut maps: HashMap<String, Vec<Map>> = HashMap::new();

    let lists = input.split_once("\n\n").unwrap();
    let seeds_str = lists.0.replace("seeds: ", "");
    let seeds = seeds_str.split(" ");

    lists.1.split("\n\n").for_each(|chunk| {
        let mut map_name = String::new();
        chunk.split("\n").enumerate().for_each(|(i, line)| {
            if i == 0 {
                map_name = line.replace(" map:", "");
                maps.insert(map_name.clone(), Vec::new());
            } else {
                let map = maps.get_mut(&map_name).unwrap();
                let values = line
                    .split(" ")
                    .map(|l| l.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();

                map.push(Map {
                    destination: values[0],
                    source: values[1],
                    range: values[2],
                });
            }
        });
    });

    seeds
        .map(|s| {
            let soil = get_map_value(&maps, "seed-to-soil", &s.parse().unwrap());
            let fertilizer = get_map_value(&maps, "soil-to-fertilizer", &soil);
            let water = get_map_value(&maps, "fertilizer-to-water", &fertilizer);
            let light = get_map_value(&maps, "water-to-light", &water);
            let temperature = get_map_value(&maps, "light-to-temperature", &light);
            let humidity = get_map_value(&maps, "temperature-to-humidity", &temperature);

            get_map_value(&maps, "humidity-to-location", &humidity)
        })
        .min()
        .unwrap()
}

fn part_2(_input: &String) -> u64 {
    0
}

fn main() {
    let input = fs::read_to_string("src/inputs/05").expect("input not found");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
