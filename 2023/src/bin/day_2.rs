use std::{cmp, fs};

fn parse_dice(dice: &str) -> (u32, &str) {
    let (value, color) = dice.split_once(" ").unwrap();
    let count: u32 = value.parse().unwrap();

    (count, color)
}

fn part_1(input: &String) -> u32 {
    input
        .split("\n")
        .enumerate()
        .map(|(i, line)| {
            let (_, rounds) = line.split_once(": ").unwrap();

            let is_valid = rounds.split("; ").all(|round| {
                round.split(", ").all(|dice| {
                    let (count, color) = parse_dice(dice);
                    match color {
                        "blue" => count <= 14,
                        "green" => count <= 13,
                        "red" => count <= 12,
                        _ => true,
                    }
                })
            });

            match is_valid {
                true => u32::try_from(i).unwrap() + 1,
                false => 0,
            }
        })
        .sum()
}

fn part_2(input: &String) -> u32 {
    input
        .split("\n")
        .map(|line| {
            let (_, rounds) = line.split_once(": ").unwrap();
            let mut blue = 0;
            let mut green = 0;
            let mut red = 0;

            rounds.split("; ").for_each(|round| {
                round.split(", ").for_each(|dice| {
                    let (count, color) = parse_dice(dice);
                    match color {
                        "blue" => blue = cmp::max(blue, count),
                        "green" => green = cmp::max(green, count),
                        "red" => red = cmp::max(red, count),
                        _ => (),
                    }
                })
            });

            blue * green * red
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("src/inputs/02").expect("input not found");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
