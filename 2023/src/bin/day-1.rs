use std::fs;

fn part_1(input: &String) -> u32 {
    let mut total = 0;

    input.split("\n").for_each(|line| {
        let mut first = '0';
        let mut last = '0';
        let mut has_first_num = false;

        line.chars().for_each(|c| {
            if c.is_numeric() {
                if !has_first_num {
                    first = c;
                    has_first_num = true;
                }

                last = c;
            }
        });

        total += format!("{}{}", first, last).parse::<u32>().unwrap();
    });

    total
}

fn calibrate(has_first_num: &mut bool, first: &mut char, last: &mut char, value: char) {
    if !*has_first_num {
        *first = value;
        *has_first_num = true;
    }

    *last = value
}

fn part_2(input: &String) -> u32 {
    let mut total = 0;

    input.split("\n").for_each(|line| {
        let mut first = '0';
        let mut last = '0';
        let mut has_first_num = false;
        let letters: Vec<char> = line.chars().collect();

        for (i, c) in letters.iter().enumerate() {
            if letters.len() > i + 2 {
                let value: String = letters[i..i + 3].iter().collect();

                match value.as_str() {
                    "one" => calibrate(&mut has_first_num, &mut first, &mut last, '1'),
                    "two" => calibrate(&mut has_first_num, &mut first, &mut last, '2'),
                    "six" => calibrate(&mut has_first_num, &mut first, &mut last, '6'),
                    _ => (),
                }
            }

            if letters.len() > i + 3 {
                let value: String = letters[i..i + 4].iter().collect();

                match value.as_str() {
                    "four" => calibrate(&mut has_first_num, &mut first, &mut last, '4'),
                    "five" => calibrate(&mut has_first_num, &mut first, &mut last, '5'),
                    "nine" => calibrate(&mut has_first_num, &mut first, &mut last, '9'),
                    _ => (),
                }
            }

            if letters.len() > i + 4 {
                let value: String = letters[i..i + 5].iter().collect();

                match value.as_str() {
                    "three" => calibrate(&mut has_first_num, &mut first, &mut last, '3'),
                    "seven" => calibrate(&mut has_first_num, &mut first, &mut last, '7'),
                    "eight" => calibrate(&mut has_first_num, &mut first, &mut last, '8'),
                    _ => (),
                }
            }

            if c.is_numeric() {
                calibrate(&mut has_first_num, &mut first, &mut last, *c);
            }
        }

        total += format!("{}{}", first, last).parse::<u32>().unwrap();
    });

    total
}

fn main() {
    let input = fs::read_to_string("src/inputs/01").expect("input not found");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
