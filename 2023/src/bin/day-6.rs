use std::fs;

fn part_1(_input: &String) -> u32 {
    0
}

fn part_2(_input: &String) -> u32 {
    0
}

fn main() {
    let input = fs::read_to_string("src/inputs/test").expect("input not found");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
