use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::BTreeSet;

#[aoc_generator(day6)]
pub fn get_input(input: &str) -> Vec<String> {
    input.split("\n\n").map(|lines| lines.to_string()).collect()
}

#[aoc(day6, part1)]
pub fn part1(ps: &Vec<String>) -> i32 {
    let count: usize = ps
        .into_iter()
        .map(|cs| {
            let mut set = BTreeSet::new();

            for c in cs.replace("\n", "").chars() {
                set.insert(c);
            }

            return set.len();
        })
        .sum();

    return count as i32;
}

#[aoc(day6, part2)]
pub fn part2(ps: &Vec<String>) -> i32 {
    let count: usize = ps
        .into_iter()
        .map(|cs| {
            let lines = cs.split('\n');

            let mut counts = lines.map(|line| {
                let mut set = BTreeSet::new();

                for c in line.replace("\n", "").chars() {
                    set.insert(c);
                }

                return set;
            });

            let intersec = counts.clone().fold(counts.next().unwrap(), |acc, c| {
                let s = acc.intersection(&c);

                let mut bt = BTreeSet::new();

                for c in s {
                    bt.insert(c.clone());
                }

                return bt;
            });

            return intersec.len();
        })
        .sum();

    return count as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> String {
        "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb".to_string()
    }

    #[test]
    fn p1_test() {
        let input = get_input(&input());
        assert_eq!(part1(&input), 11);
    }

    #[test]
    fn p2_test() {
        let input = get_input(&input());
        assert_eq!(part2(&input), 6);
    }
}
