use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day10.txt";

type InputType = Vec<Instruction>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Noop,
    AddX(i32),
}

struct Cpu {
    x: i32,
    pc: usize,
    cycle: i32,
    program: Vec<Instruction>,
    pending: Option<(i32, Instruction)>,
}

impl Cpu {
    pub fn new(program: Vec<Instruction>) -> Self {
        Self {
            program,
            x: 1,
            pc: 0,
            cycle: 0,
            pending: None,
        }
    }

    pub fn step(&mut self) -> bool {
        self.cycle += 1;

        if let Some((target, instr)) = self.pending.clone() {
            if target == self.cycle {
                match instr {
                    Instruction::Noop => {}
                    Instruction::AddX(value) => {
                        self.x += value;
                    }
                }

                self.pending = None;
            } else {
                return true;
            }
        }

        let Some(instr) = self.program.get(self.pc).cloned() else {
            return false;
        };

        match instr {
            Instruction::Noop => {}
            Instruction::AddX(value) => {
                self.pending = Some((self.cycle + 2, Instruction::AddX(value)));
            }
        }

        self.pc += 1;

        true
    }

    fn x(&self) -> i32 {
        self.x
    }

    fn cycle(&self) -> i32 {
        self.cycle
    }
}

fn part1(input: &InputType) -> Result<(), Error> {
    let mut answer = 0;
    let mut cpu = Cpu::new(input.to_vec());

    while cpu.step() {
        if cpu.cycle().checked_sub(20).unwrap_or(1) % 40 == 0 {
            answer += cpu.cycle() * cpu.x();
        }
    }

    println!("part1: {answer}");
    Ok(())
}

fn part2(input: &InputType) -> Result<(), Error> {
    let mut answer = vec![b' '; 6 * 40];
    let mut cpu = Cpu::new(input.to_vec());

    while cpu.step() {
        let x = (cpu.cycle() - 1) % 40;
        let y = (cpu.cycle() - 1) / 40;

        if (cpu.x() - x).abs() < 2 {
            answer[(y * 40 + x) as usize] = b'#';
        }
    }

    println!("part2:");
    for row in answer.chunks(40) {
        let row: String = row.iter().copied().map(char::from).collect();
        println!("{}", row);
    }

    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let input = input
        .lines()
        .map(|line| {
            let line: Vec<_> = line.split(' ').collect();
            match line.as_slice() {
                ["noop"] => Instruction::Noop,
                ["addx", value] => {
                    let value = value
                        .parse()
                        .expect("invalid argument for `addx` instruction");
                    Instruction::AddX(value)
                }
                [instr, args @ ..] => {
                    panic!("invalid instruction `{instr}` with args: {args:?}");
                }
                [] => {
                    panic!("missing instruction");
                }
            }
        })
        .collect();

    measured!(part1(&input))?;
    measured!(part2(&input))?;

    Ok(())
}
