use std::collections::HashSet;
use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day03.txt";

type InputType = Vec<Vec<u8>>;

fn part1(input: &InputType) -> Result<(), Error> {
    let answer = input
        .iter()
        .map(|line| {
            let (half_1, half_2) = line.split_at(line.len() / 2);

            let set_1: HashSet<_> = half_1.iter().copied().collect();
            let set_2: HashSet<_> = half_2.iter().copied().collect();

            set_1.intersection(&set_2).next().map_or(0u64, |value| {
                if value.is_ascii_lowercase() {
                    (*value - b'a' + 1).into()
                } else {
                    (*value - b'A' + 27).into()
                }
            })
        })
        .sum::<u64>();

    println!("part1: {answer}");
    Ok(())
}

fn part2(input: &InputType) -> Result<(), Error> {
    let answer = input
        .chunks(3)
        .map(|group| {
            group
                .iter()
                .map(|line| line.iter().copied().collect::<HashSet<u8>>())
                .reduce(|acc, set| acc.intersection(&set).copied().collect())
                .expect("empty input")
                .into_iter()
                .next()
                .map_or(0u64, |value| {
                    if value.is_ascii_lowercase() {
                        (value - b'a' + 1).into()
                    } else {
                        (value - b'A' + 27).into()
                    }
                })
        })
        .sum::<u64>();

    println!("part2: {answer}");
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let input: InputType = input.lines().map(|line| line.bytes().collect()).collect();

    measured!(part1(&input))?;
    measured!(part2(&input))?;

    Ok(())
}
