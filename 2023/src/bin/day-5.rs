use std::fs;

struct Map {
    to: u64,
    from: u64,
    range: u64,
}

fn get_map_value(from: u64, list: &Vec<Map>) -> u64 {
    for map in list {
        if from >= map.from && from < map.from + map.range {
            return map.to + (from - map.from);
        }
    }

    from.clone()
}

fn part_1(input: &String) -> u64 {
    let mut maps: Vec<Vec<Map>> = vec![];
    let mut chunks = input.trim_end().split("\n\n");
    let seeds = chunks.next().unwrap().replace("seeds: ", "");

    chunks.for_each(|chunk| {
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

        maps.push(map);
    });

    seeds
        .split(" ")
        .map(|s| maps.iter().fold(s.parse().unwrap(), get_map_value))
        .min()
        .unwrap()
}

// struct Seed {
//     to: u64,
//     range: u64,
// }

fn part_2(_input: &String) -> u64 {
    // let mut maps: HashMap<String, Vec<Map>> = HashMap::new();
    // let lists = input.split_once("\n\n").unwrap();

    // let seed_str = lists.0.replace("seeds: ", "");
    // let items: Vec<&str> = seed_str.split(" ").collect();
    // let seeds = (1..items.len()).step_by(2).map(|i| Seed {
    //     to: items[i - 1].parse().unwrap(),
    //     range: items[i].parse().unwrap(),
    // });

    // lists.1.split("\n\n").for_each(|chunk| {
    //     let mut map_name = String::new();
    //     chunk.split("\n").enumerate().for_each(|(i, line)| {
    //         if i == 0 {
    //             map_name = line.replace(" map:", "");
    //             maps.insert(map_name.clone(), Vec::new());
    //         } else {
    //             let map = maps.get_mut(&map_name).unwrap();
    //             let values = line
    //                 .split(" ")
    //                 .map(|l| l.parse::<u64>().unwrap())
    //                 .collect::<Vec<u64>>();

    //             map.push(Map {
    //                 to: values[0],
    //                 from: values[1],
    //                 range: values[2],
    //             });
    //         }
    //     });
    // });

    // let seed_to_soil = maps.get("seed-to-soil").unwrap();
    // let mut my_seed_to_soil: Vec<Map> = vec![];

    // seeds.for_each(|seed| {
    //     let to = seed.to;
    //     let from = 0;
    //     let range = seed.range;

    //     my_seed_to_soil.push(Map { to, from, range })
    // });
    0
}

fn main() {
    let input = fs::read_to_string("src/inputs/05").expect("input not found");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
