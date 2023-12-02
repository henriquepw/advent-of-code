use std::fs;

fn part_1() -> u32 {
    let inputs = fs::read_to_string("src/inputs/01.txt").expect("input not found");
    let mut total = 0;

    inputs.split("\n").for_each(|line| {
        let mut head = '0';
        let mut tail = '0';
        let mut has_first_num = false;

        line.chars().for_each(|c| {
            if c.is_numeric() {
                if !has_first_num {
                    head = c;
                    has_first_num = true;
                }

                tail = c;
            }
        });

        total += format!("{}{}", head, tail).parse::<u32>().unwrap();
    });

    total
}

fn part_2() -> u32 {
    let inputs = fs::read_to_string("src/inputs/01.txt").expect("input not found");
    let total = 0;

    inputs.split("\n").for_each(|line| {
        line.chars().for_each(|c| {
           todo!()
        })
    });

    total
}

fn main() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}
