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

            match count {
                0 => 0,
                1 => 1,
                n => 2 << n - 2,
            }
        })
        .sum()
}

fn part_2(input: &String) -> u32 {
    let lines = input.split("\n");
    let mut copies = vec![1; lines.clone().count()];

    lines.enumerate().for_each(|(index, line)| {
        let lists = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        let winning_numbers = parse_list(lists.0);
        let my_numbers: HashSet<&str> = HashSet::from_iter(parse_list(lists.1));
        let count = winning_numbers.filter(|w| my_numbers.contains(*w)).count();

        for n in 1..count + 1 {
            if index + n < copies.len() {
                copies[index + n] += copies[index];
            }
        }
    });

    copies.iter().sum()
}

fn main() {
    let input = fs::read_to_string("src/inputs/04").expect("input not found");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
