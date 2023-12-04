use std::fs;

fn check_possible(value: &str, color: &str) -> bool {
    let count: u32 = value.replace(color, "").trim().parse().unwrap();

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

fn part_2() -> u32 {
    0
}

fn main() {
    let input = fs::read_to_string("src/inputs/02").expect("input not found");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2());
}
