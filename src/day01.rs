use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day01.txt";

type InputType = Vec<Vec<u64>>;

fn part1(input: &InputType) -> Result<(), Error> {
    let answer: u64 = input
        .iter()
        .map(|it| it.iter().sum())
        .max()
        .expect("empty input");

    println!("part1: {answer}");
    Ok(())
}

fn part2(input: &InputType) -> Result<(), Error> {
    let mut answer: Vec<u64> = input.iter().map(|it| it.iter().sum()).collect();

    answer.sort();

    let answer: u64 = answer.iter().rev().take(3).sum();

    println!("part2: {answer}");
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let input: InputType = input
        .split("\n\n")
        .map(|it| {
            it.trim()
                .split('\n')
                .map(|it| it.trim().parse())
                .collect::<Result<_, _>>()
                .map_err(Error::from)
        })
        .collect::<Result<_, Error>>()?;

    measured!(part1(&input))?;
    measured!(part2(&input))?;

    Ok(())
}
