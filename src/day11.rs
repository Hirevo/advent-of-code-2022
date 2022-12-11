use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

use chumsky::prelude::*;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day11.txt";

type InputType = Vec<Monkey>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Value {
    Old,
    Const(u64),
}

impl Value {
    pub fn evaluate(self, old: u64) -> u64 {
        match self {
            Value::Old => old,
            Value::Const(value) => value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Expression {
    Add(Value, Value),
    Sub(Value, Value),
    Mul(Value, Value),
    Div(Value, Value),
    Mod(Value, Value),
}

impl Expression {
    pub fn evaluate(&self, old: u64) -> u64 {
        match self {
            Expression::Add(lhs, rhs) => lhs.evaluate(old) + rhs.evaluate(old),
            Expression::Sub(lhs, rhs) => lhs.evaluate(old) - rhs.evaluate(old),
            Expression::Mul(lhs, rhs) => lhs.evaluate(old) * rhs.evaluate(old),
            Expression::Div(lhs, rhs) => lhs.evaluate(old) / rhs.evaluate(old),
            Expression::Mod(lhs, rhs) => lhs.evaluate(old) % rhs.evaluate(old),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Monkey {
    pub items: Vec<u64>,
    pub count: usize,
    pub expression: Expression,
    pub divisor: u64,
    pub success_target: usize,
    pub failure_target: usize,
}

fn part1(input: &InputType) -> Result<(), Error> {
    let mut monkeys = input.to_vec();

    // cache to process items without any active borrows into `monkeys`
    let mut items = Vec::new();

    for _ in 0..20 {
        for idx in 0..monkeys.len() {
            let monkey = &mut monkeys[idx];

            std::mem::swap(&mut monkey.items, &mut items);

            let divisor = monkey.divisor;
            let expression = monkey.expression.clone();
            let success_target = monkey.success_target;
            let failure_target = monkey.failure_target;

            monkey.count += items.len();

            for item in items.drain(..) {
                let item = expression.evaluate(item) / 3;

                let target = if item % divisor == 0 {
                    success_target
                } else {
                    failure_target
                };

                monkeys[target].items.push(item);
            }

            items.clear();
        }
    }

    let mut answer: Vec<_> = monkeys.into_iter().map(|monkey| monkey.count).collect();
    answer.sort_unstable();
    let answer = answer
        .into_iter()
        .rev()
        .take(2)
        .fold(1, |acc, count| acc * count);

    println!("part1: {answer}");
    Ok(())
}

fn part2(input: &InputType) -> Result<(), Error> {
    let mut monkeys = input.to_vec();

    let mut items = Vec::new();

    let global_divisor = monkeys
        .iter()
        .map(|monkey| monkey.divisor)
        .fold(1, |acc, divisor| acc * divisor);

    for _ in 0..10_000 {
        for idx in 0..monkeys.len() {
            let monkey = &mut monkeys[idx];

            std::mem::swap(&mut monkey.items, &mut items);

            let divisor = monkey.divisor;
            let expression = monkey.expression.clone();
            let success_target = monkey.success_target;
            let failure_target = monkey.failure_target;

            monkey.count += items.len();

            for item in items.drain(..) {
                let item = expression.evaluate(item) % global_divisor;

                let target = if item % divisor == 0 {
                    success_target
                } else {
                    failure_target
                };

                monkeys[target].items.push(item);
            }

            items.clear();
        }
    }

    let mut answer: Vec<_> = monkeys.into_iter().map(|monkey| monkey.count).collect();
    answer.sort_unstable();
    let answer = answer
        .into_iter()
        .rev()
        .take(2)
        .fold(1, |acc, count| acc * count);

    println!("part1: {answer}");
    Ok(())
}

fn integer<O>() -> impl Parser<char, O, Error = Simple<char>>
where
    O: FromStr,
    O::Err: Debug,
{
    text::int(10).from_str().unwrapped()
}

fn value() -> impl Parser<char, Value, Error = Simple<char>> {
    just("old")
        .map(|_| Value::Old)
        .or(integer::<u64>().map(Value::Const))
}

fn operator() -> impl Parser<char, fn(Value, Value) -> Expression, Error = Simple<char>> {
    (just("+").map(|_| Expression::Add as fn(Value, Value) -> Expression))
        .or(just("-").map(|_| Expression::Sub as fn(Value, Value) -> Expression))
        .or(just("*").map(|_| Expression::Mul as fn(Value, Value) -> Expression))
        .or(just("/").map(|_| Expression::Div as fn(Value, Value) -> Expression))
        .or(just("%").map(|_| Expression::Mod as fn(Value, Value) -> Expression))
}

// not my proudest parser, but it was quick enough to put together
fn monkey() -> impl Parser<char, Monkey, Error = Simple<char>> {
    let id = just("Monkey ")
        .ignore_then(integer::<usize>())
        .then_ignore(just(":"))
        .then_ignore(text::newline());

    let items = just("  Starting items: ")
        .ignore_then(integer::<u64>().separated_by(just(", ")))
        .then_ignore(text::newline());

    let expr = just("  Operation: new = ")
        .ignore_then(value())
        .then_ignore(just(" "))
        .then(operator())
        .then_ignore(just(" "))
        .then(value())
        .then_ignore(text::newline())
        .map(|((lhs, operator), rhs)| operator(lhs, rhs));

    let divisor = just("  Test: divisible by ")
        .ignore_then(integer::<u64>())
        .then_ignore(text::newline());

    let success_target = just("    If true: throw to monkey ")
        .ignore_then(integer::<usize>())
        .then_ignore(text::newline());

    let failure_target = just("    If false: throw to monkey ").ignore_then(integer::<usize>());

    id.then(items.then(expr).then(divisor))
        .then(success_target.then(failure_target))
        .then_ignore(text::newline().or_not().ignore_then(end()))
        .map(
            |((_, ((items, expr), divisor)), (success_target, failure_target))| Monkey {
                items,
                expression: expr,
                divisor,
                success_target,
                failure_target,
                count: 0,
            },
        )
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let input: Result<InputType, _> = input
        .split("\n\n")
        .map(|section| monkey().parse(section))
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
