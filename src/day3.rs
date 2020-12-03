use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn run() {
    let input = get_input();

    println!("{}", p1(input.clone()));
    println!("{}", p2(input.clone()));
}

pub fn get_input() -> Vec<String> {
    const FILENAME: &str = "data/day3.txt";

    let file = File::open(Path::new(FILENAME)).expect("Error opening file");

    BufReader::new(file).lines().map(|l| l.unwrap()).collect()
}

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

pub fn p1(lines: Vec<String>) -> i32 {
    let leni = lines.len() as i32;
    let lenj = lines[0].len() as i32;

    let mut count = 0;

    for (i, j) in tobo(leni, lenj, 1, 3) {
        let char = lines[i as usize].chars().collect::<Vec<char>>()[j as usize];
        if char == '#' {
            count += 1;
        }
    }

    count
}

pub fn p2(lines: Vec<String>) -> i64 {
    let leni = lines.len() as i32;
    let lenj = lines[0].len() as i32;

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
        assert_eq!(p1(input), 7);
    }

    #[test]
    fn p2_test() {
        let input = input();
        assert_eq!(p2(input), 336);
    }
}
