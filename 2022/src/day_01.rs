use std::fs;

#[allow(dead_code)]
pub fn part_1() -> i32 {
    let elves = fs::read_to_string("./inputs/01.txt").expect("input not found");
    let mut max_calories = 0;

    elves.split("\n\n").for_each(|foods| {
        let elf_calories = foods
            .split("\n")
            .fold(0, |total, food| total + food.parse::<i32>().unwrap_or(0));

        if elf_calories > max_calories {
            max_calories = elf_calories;
        }
    });

    return max_calories;
}

pub fn part_2() -> i32 {
    let elves = fs::read_to_string("./inputs/01.txt").expect("input not found");

    let mut calories: Vec<i32> = elves
        .split("\n\n")
        .map(|foods| {
            foods
                .split("\n")
                .map(|food| food.parse().unwrap_or(0))
                .sum()
        })
        .collect();

    calories.sort();

    return calories[calories.len() - 3..].iter().sum();
}
