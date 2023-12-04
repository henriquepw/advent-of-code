use std::{cmp, fs};

fn get_dice_value(dice: &str, color: &str) -> u32 {
    dice.replace(color, "").trim().parse().unwrap()
}

fn check_possible(dice: &str, color: &str) -> bool {
    let count = get_dice_value(dice, color);

    match color {
        "blue" => count <= 14,
        "green" => count <= 13,
        "red" => count <= 12,
        _ => true,
    }
}

fn part_1(input: &String) -> u32 {
    let mut total = 0;

    input.split("\n").for_each(|line| {
        let (game, rounds) = line.split_once(": ").unwrap();

        let is_valid = rounds.split("; ").all(|round| {
            round.split(", ").all(|dice| match dice {
                s if s.ends_with(" blue") => check_possible(s, "blue"),
                s if s.ends_with(" green") => check_possible(s, "green"),
                s if s.ends_with(" red") => check_possible(s, "red"),
                _ => true,
            })
        });

        if is_valid {
            total += game.replace("Game ", "").parse::<u32>().unwrap();
        }
    });

    total
}

fn part_2(input: &String) -> u32 {
    let mut total = 0;

    input.split("\n").for_each(|line| {
        let (_, rounds) = line.split_once(": ").unwrap();
        let mut blue_value = 0;
        let mut green_value = 0;
        let mut red_value = 0;

        rounds.split("; ").for_each(|round| {
            round.split(", ").for_each(|dice| match dice {
                s if s.ends_with("blue") => {
                    blue_value = cmp::max(blue_value, get_dice_value(s, "blue"));
                }
                s if s.ends_with("green") => {
                    green_value = cmp::max(green_value, get_dice_value(s, "green"));
                }
                s if s.ends_with("red") => {
                    red_value = cmp::max(red_value, get_dice_value(s, "red"));
                }
                _ => (),
            })
        });

        total += blue_value * green_value * red_value;
    });

    total
}

fn main() {
    let input = fs::read_to_string("src/inputs/02").expect("input not found");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
