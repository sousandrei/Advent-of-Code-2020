use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::BTreeSet;
use std::fmt;

#[derive(PartialEq)]
pub enum Command {
    Acc,
    Jmp,
    Nop,
}
pub struct Instr {
    cmd: Command,
    value: i16,
}

impl fmt::Debug for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cmd = match &self.cmd {
            Command::Acc => String::from("acc"),
            Command::Jmp => String::from("jmp"),
            _ => String::from("nop"),
        };

        f.debug_struct("Instr")
            .field("x", &cmd)
            .field("y", &self.value)
            .finish()
    }
}

#[aoc_generator(day8)]
pub fn get_input(input: &str) -> Vec<Instr> {
    input
        .split("\n")
        .map(|line| {
            let words: Vec<&str> = line.split(' ').collect();

            Instr {
                cmd: match words[0] {
                    "acc" => Command::Acc,
                    "jmp" => Command::Jmp,
                    _ => Command::Nop,
                },
                value: words[1].parse::<i16>().unwrap(),
            }
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn part1(code: &Vec<Instr>) -> i32 {
    let mut lines = BTreeSet::new();

    let mut pc: i16 = 0;
    let mut acc: i32 = 0;

    loop {
        if !lines.insert(pc) {
            return acc;
        }

        match code[pc as usize].cmd {
            Command::Acc => acc += code[pc as usize].value as i32,
            Command::Jmp => pc += code[pc as usize].value - 1,
            _ => {}
        };

        pc += 1;
    }
}

#[aoc(day8, part2)]
pub fn part2(code: &Vec<Instr>) -> i32 {
    for i in 0..code.len() {
        let mut lines = BTreeSet::new();

        let mut acc: i32 = 0;
        let mut pc: i16 = 0;

        loop {
            if pc as usize == code.len() {
                return acc;
            }

            if !lines.insert(pc) {
                break;
            }

            if i == (pc as usize) {
                match code[pc as usize].cmd {
                    Command::Acc => acc += code[pc as usize].value as i32,
                    Command::Jmp => {}
                    _ => pc += code[pc as usize].value - 1,
                };
            } else {
                match code[pc as usize].cmd {
                    Command::Acc => acc += code[pc as usize].value as i32,
                    Command::Jmp => pc += code[pc as usize].value - 1,
                    _ => {}
                };
            }

            pc += 1;
        }
    }

    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6"
    }

    #[test]
    fn p1_test() {
        let input = get_input(input());
        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn p2_test() {
        let input = get_input(input());
        assert_eq!(part2(&input), 8);
    }
}
