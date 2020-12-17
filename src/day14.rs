use aoc_runner_derive::{aoc, aoc_generator};

use regex::Regex;
use std::collections::HashMap;
use std::isize;

#[derive(Debug, Clone, Copy)]
pub struct Operation {
    addr: i64,
    value: i64,
}

#[derive(Debug, Clone)]
pub struct Maskops {
    maskor: i64,
    maskand: i64,
    operations: Vec<Operation>,
}

#[aoc_generator(day14)]
pub fn get_input(input: &str) -> Vec<Maskops> {
    let rx = Regex::new(r"(?m)(?:mask = ([10X]{36})\n)|(?:mem\[(\d+)\] = (\d+))").unwrap();

    let caps = rx.captures_iter(input);

    let mut segment = Maskops {
        maskand: 0,
        maskor: 0,
        operations: Vec::new(),
    };
    let mut segments: Vec<Maskops> = Vec::new();

    for cap in caps {
        if cap.get(2).is_none() {
            segments.push(segment.to_owned());

            let mask = cap.get(1).unwrap().as_str();

            segment = Maskops {
                maskand: isize::from_str_radix(&mask.replace("X", "1"), 2).unwrap_or(-1) as i64,
                maskor: isize::from_str_radix(&mask.replace("X", "0"), 2).unwrap_or(-1) as i64,
                operations: Vec::new(),
            };

            continue;
        }

        let op = Operation {
            addr: cap.get(2).unwrap().as_str().parse().unwrap(),
            value: cap.get(3).unwrap().as_str().parse().unwrap(),
        };

        segment.operations.push(op);
    }

    segments.push(segment.to_owned());
    segments.remove(0);

    return segments;
}

#[aoc(day14, part1)]
pub fn part1(input: &Vec<Maskops>) -> i64 {
    let mut memory = HashMap::new();

    for m_ops in input {
        for ops in &m_ops.operations {
            memory.insert(ops.addr, (ops.value & m_ops.maskand) | m_ops.maskor);
        }
    }

    return memory.values().sum();
}

#[aoc(day14, part2)]
pub fn part2(input: &Vec<Maskops>) -> i64 {
    let mut memory: HashMap<i64, i64> = HashMap::new();

    for m_ops in input {
        for ops in &m_ops.operations {
            let addr = ops.addr | m_ops.maskor;

            let maskx = m_ops.maskand ^ m_ops.maskor;

            let mut xor = maskx + 1;

            for _ in 0..2_u16.pow(maskx.count_ones()) {
                xor = (xor - 1) & maskx;

                memory.insert(addr ^ xor, ops.value);
            }
        }
    }

    return memory.values().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let input = get_input(
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0",
        );
        assert_eq!(part1(&input), 165);
    }

    #[test]
    fn p2_test() {
        let input = get_input(
            "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1",
        );
        assert_eq!(part2(&input), 208);
    }
}
