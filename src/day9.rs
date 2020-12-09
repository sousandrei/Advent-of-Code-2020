use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::{BTreeSet, VecDeque};

#[cfg(not(test))]
const THRESHOLD: usize = 25;

#[cfg(test)]
const THRESHOLD: usize = 5;

struct SafaQueue {
    threshold: usize,
    set: BTreeSet<i64>,
    values: VecDeque<i64>,
}

#[allow(dead_code)]
impl SafaQueue {
    pub fn new(threshold: usize) -> SafaQueue {
        SafaQueue {
            threshold,
            set: BTreeSet::new(),
            values: VecDeque::new(),
        }
    }

    fn push(&mut self, value: i64) {
        self.values.push_front(value);
        self.set.insert(value);
    }

    fn pop(&mut self) {
        let value = self.values.pop_back().unwrap();
        self.set.remove(&value);
    }

    fn contain_sum(&mut self, value: i64) -> bool {
        let v: Vec<_> = self.set.iter().collect();

        let mut i = 0;
        let mut j = v.len() - 1;

        while i != j {
            let result = v[i] + v[j];

            if result < value {
                i += 1;
            } else if result > value {
                j -= 1;
            } else {
                return true;
            }
        }

        return false;
    }

    fn insert(&mut self, value: i64) -> bool {
        if self.values.len() < self.threshold {
            self.push(value);
            return true;
        }

        if self.contain_sum(value) {
            self.pop();
            self.push(value);
            return true;
        }

        return false;
    }
}

#[aoc_generator(day9)]
pub fn get_input(input: &str) -> Vec<i64> {
    input
        .split("\n")
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(code: &Vec<i64>) -> i64 {
    let mut sq = SafaQueue::new(THRESHOLD);

    for c in code {
        if !sq.insert(*c) {
            return *c;
        }
    }

    return 0;
}

fn encryption_weakness_factory_builder_aurora_java(code: &Vec<i64>, value: i64) -> i64 {
    let mut i = 0;
    let mut j = 0;
    let mut sum = 0; // sum(v[i..j])

    while j < code.len() {
        if sum < value {
            sum += code[j];
            j += 1;
        } else if sum > value {
            sum -= code[i];
            i += 1;
        } else {
            return code[i..j].iter().min().unwrap() + code[i..j].iter().max().unwrap();
        }
    }

    return 0;
}

#[aoc(day9, part2)]
pub fn part2(code: &Vec<i64>) -> i64 {
    let mut sq = SafaQueue::new(THRESHOLD);

    for c in code {
        if !sq.insert(*c) {
            return encryption_weakness_factory_builder_aurora_java(code, *c);
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576"
    }

    #[test]
    fn p1_test() {
        let input = get_input(input());
        assert_eq!(part1(&input), 127);
    }

    #[test]
    fn p2_test() {
        let input = get_input(input());
        assert_eq!(part2(&input), 62);
    }
}
