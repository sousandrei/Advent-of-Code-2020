use aoc_runner_derive::{aoc, aoc_generator};

struct IterData {
    i: i32,
    j: i32,
    leni: i32,
    lenj: i32,
    si: i32,
    sj: i32,
}

impl Iterator for IterData {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        self.i = self.i + self.si;
        self.j = (self.j + self.sj) % self.lenj;

        if self.i >= self.leni {
            return None;
        }

        Some((self.i, self.j))
    }
}

#[aoc_generator(day3)]
pub fn get_input(input: &str) -> Vec<String> {
    input.split('\n').map(|line| line.to_string()).collect()
}

fn tobo(leni: i32, lenj: i32, si: i32, sj: i32) -> IterData {
    IterData {
        i: 0,
        j: 0,
        leni,
        lenj,
        si,
        sj,
    }
}

#[aoc(day3, part1)]
pub fn part1(lines: &Vec<String>) -> i32 {
    let mut leni = 322;
    let mut lenj = 31;

    if cfg!(test) {
        leni = 11;
        lenj = 11;
    }

    let mut count = 0;

    for (i, j) in tobo(leni, lenj, 1, 3) {
        let char = lines[i as usize].chars().collect::<Vec<char>>()[j as usize];
        if char == '#' {
            count += 1;
        }
    }

    count
}

#[aoc(day3, part2)]
pub fn part2(lines: &Vec<String>) -> i64 {
    let mut leni = 322;
    let mut lenj = 31;

    if cfg!(test) {
        leni = 11;
        lenj = 11;
    }

    let steps = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut result = 1;
    for (si, sj) in steps {
        let mut count = 0;

        for (i, j) in tobo(leni, lenj, sj, si) {
            let char = lines[i as usize].chars().collect::<Vec<char>>()[j as usize];
            if char == '#' {
                count += 1;
            }
        }

        result *= count;
    }

    result
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    fn input() -> Vec<String> {
        let v = vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ];

        v.into_iter().map(|i| i.to_string()).collect()
    }

    #[test]
    fn p1_test() {
        let input = input();
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn p2_test() {
        let input = input();
        assert_eq!(part2(&input), 336);
    }
}
