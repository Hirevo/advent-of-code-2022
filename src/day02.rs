use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day02.txt";

type InputType = Vec<(u8, u8)>;

fn part1(input: &InputType) -> Result<(), Error> {
    let answer: i32 = input
        .iter()
        .map(|(opponent, r#move)| {
            let r#move = [b'X', b'Y', b'Z']
                .iter()
                .position(|it| it == r#move)
                .expect("invalid input") as i32;

            let opponent = [b'A', b'B', b'C']
                .iter()
                .position(|it| it == opponent)
                .expect("invalid input") as i32;

            match (r#move - opponent).rem_euclid(3) {
                0 => 3 + r#move + 1,
                1 => 6 + r#move + 1,
                2 => 0 + r#move + 1,
                _ => unreachable!(),
            }
        })
        .sum();

    println!("part1: {answer}");
    Ok(())
}

fn part2(input: &InputType) -> Result<(), Error> {
    let answer: i32 = input
        .iter()
        .map(|(opponent, outcome)| {
            let outcome = [b'Y', b'Z', b'X']
                .iter()
                .position(|it| it == outcome)
                .expect("invalid input") as i32;

            let opponent = [b'A', b'B', b'C']
                .iter()
                .position(|it| it == opponent)
                .expect("invalid input") as i32;

            let r#move = (opponent + outcome).rem_euclid(3);

            match outcome {
                0 => 3 + r#move + 1,
                1 => 6 + r#move + 1,
                2 => 0 + r#move + 1,
                _ => unreachable!(),
            }
        })
        .sum();

    println!("part2: {answer}");
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let input: InputType = input
        .split("\n")
        .filter_map(|it| {
            let (a, b) = it.split_once(" ")?;
            let &a = a.as_bytes().first()?;
            let &b = b.as_bytes().first()?;
            Some((a, b))
        })
        .collect();

    measured!(part1(&input))?;
    measured!(part2(&input))?;

    Ok(())
}
