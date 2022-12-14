use std::cmp::Ordering;
use std::fs;

use chumsky::prelude::*;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day13.txt";

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Int(i32),
    List(Vec<Value>),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a.partial_cmp(b),
            (Value::List(a), Value::List(b)) => {
                for (a, b) in a.iter().zip(b.iter()) {
                    match a.partial_cmp(&b)? {
                        Ordering::Equal => continue,
                        value => return Some(value),
                    }
                }
                a.len().partial_cmp(&b.len())
            }
            (Value::Int(_), Value::List(_)) => Value::List(vec![self.clone()]).partial_cmp(other),
            (Value::List(_), Value::Int(_)) => self.partial_cmp(&Value::List(vec![other.clone()])),
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("infallible")
    }
}

type InputType = Vec<Value>;

fn parser() -> impl Parser<char, Value, Error = Simple<char>> {
    recursive(|value| {
        let number = text::int(10).from_str().unwrapped().map(Value::Int);
        let list = value
            .clone()
            .separated_by(just(','))
            .delimited_by(just('['), just(']'))
            .map(Value::List);
        number.or(list)
    })
}

fn part1(input: &InputType) -> Result<(), Error> {
    let answer: usize = input
        .chunks(2)
        .enumerate()
        .filter(|(_, it)| it[0] < it[1])
        .map(|(idx, _)| idx + 1)
        .sum();

    println!("part1: {answer}");
    Ok(())
}

fn part2(input: &InputType) -> Result<(), Error> {
    let mut input = input.clone();

    let divider_1 = Value::List(vec![Value::List(vec![Value::Int(2)])]);
    let divider_2 = Value::List(vec![Value::List(vec![Value::Int(6)])]);

    input.push(divider_1.clone());
    input.push(divider_2.clone());

    input.sort_unstable();

    let divider_1 = input.iter().position(|it| *it == divider_1).unwrap() + 1;
    let divider_2 = input.iter().position(|it| *it == divider_2).unwrap() + 1;

    let answer = divider_1 * divider_2;

    println!("part2: {answer}");
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let input = input
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| parser().parse(line))
        .collect();

    let input = match input {
        Ok(input) => input,
        Err(err) => {
            eprintln!("{err:?}");
            std::process::exit(1);
        }
    };

    measured!(part1(&input))?;
    measured!(part2(&input))?;

    Ok(())
}
