use std::{collections::HashSet, fs};

fn parse_list(list: &str) -> impl Iterator<Item = &str> {
    list.split(" ").filter(|w| *w != "")
}

fn part_1(input: &String) -> u32 {
    input
        .split("\n")
        .map(|line| {
            let lists = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
            let winning_numbers = parse_list(lists.0);
            let my_numbers: HashSet<&str> = HashSet::from_iter(parse_list(lists.1));
            let count = winning_numbers.filter(|w| my_numbers.contains(*w)).count();

            if count == 0 {
                return 0;
            }

            u32::pow(2, count as u32 - 1)
        })
        .sum()
}

fn part_2(_input: &String) -> u32 {
    0
}

fn main() {
    let input = fs::read_to_string("src/inputs/04").expect("input not found");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
