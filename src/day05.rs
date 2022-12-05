use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day05.txt";

type StacksType = Vec<Vec<char>>;
type InstrsType = Vec<Instruction>;

#[derive(Debug, Clone, PartialEq)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn part1(stacks: &StacksType, instructions: &InstrsType) -> Result<(), Error> {
    let mut stacks: StacksType = stacks.iter().cloned().collect();

    for instr in instructions {
        for _ in 0..instr.count {
            if let Some(ch) = stacks[instr.from].pop() {
                stacks[instr.to].push(ch);
            }
        }
    }

    let answer: String = stacks.into_iter().flat_map(|mut it| it.pop()).collect();

    println!("part1: {answer}");
    Ok(())
}

fn part2(stacks: &StacksType, instructions: &InstrsType) -> Result<(), Error> {
    let mut stacks: StacksType = stacks.iter().cloned().collect();
    let mut cache = Vec::new();

    for instr in instructions {
        let idx = stacks[instr.from]
            .len()
            .checked_sub(instr.count)
            .unwrap_or_default();
        cache.extend(stacks[instr.from].drain(idx..));
        stacks[instr.to].extend(cache.drain(..));

        // The following would have been the dream:
        // `stacks[instr.to].extend(stacks[instr.from].drain(idx..));`
        // We wouldn't have needed an intermediate buffer at all.
        //
        // Sadly, we can't yet easily prove to the compiler that the
        // two simultaneous mutable borrows of `stacks` are disjoint,
        // so it conservatively rejects that code.
    }

    let answer: String = stacks.into_iter().flat_map(|mut it| it.pop()).collect();

    println!("part2: {answer}");
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let (stacks, instructions) = input.split_once("\n\n").expect("invalid input");

    let stacks = {
        let rows = {
            let mut rows: Vec<_> = stacks.lines().collect();
            rows.pop();
            rows
        };

        let stack_count = {
            let len = rows.iter().map(|it| it.len()).max().expect("invalid input");
            (len - 3) / 4 + 1
        };

        let mut stacks = vec![vec![]; stack_count];
        for idx in (0..rows.len()).rev() {
            let row = &rows[idx];
            for idx in 0..stack_count {
                let Some(ch) = row.chars().nth(idx * 4 + 1) else {
                    continue
                };

                if ch != ' ' {
                    stacks[idx].push(ch);
                }
            }
        }

        stacks
    };

    let instructions = instructions
        .lines()
        .map(|line| {
            let mut iter = line
                .split_ascii_whitespace()
                .skip(1)
                .step_by(2)
                .flat_map(|it| it.parse());

            Instruction {
                count: iter.next().expect("invalid input"),
                from: iter.next().expect("invalid input") - 1,
                to: iter.next().expect("invalid input") - 1,
            }
        })
        .collect();

    measured!(part1(&stacks, &instructions))?;
    measured!(part2(&stacks, &instructions))?;

    Ok(())
}
