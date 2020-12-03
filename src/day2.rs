use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn run() {
    let input = get_input();
    let lines = parse_lines(&input);

    println!("{}", p1(lines.clone()));
    println!("{}", p2(lines.clone()));
}

pub fn get_input() -> Vec<String> {
    const FILENAME: &str = "data/day2.txt";

    let file = File::open(Path::new(FILENAME)).expect("Error opening file");

    BufReader::new(file).lines().map(|l| l.unwrap()).collect()
}

pub fn parse_lines<'a>(lines: &'a Vec<String>) -> Vec<(usize, usize, char, &'a str)> {
    lines
        .iter()
        .map(|line| {
            let v: Vec<&str> = line.split(": ").collect();

            let args: Vec<&str> = v[0].split(" ").collect();

            let c: char = args[1].parse().unwrap();

            let counts: Vec<&str> = args[0].split('-').collect();

            let min: usize = counts[0].parse().unwrap();
            let max: usize = counts[1].parse().unwrap();

            let s = v[1];

            (min, max, c, s)
        })
        .collect()
}

pub fn p1<'a>(lines: Vec<(usize, usize, char, &'a str)>) -> i32 {
    let mut count = 0;

    for (min, max, c, s) in lines {
        let char_count = s.matches(c).count();

        if char_count >= min && char_count <= max {
            count += 1;
        }
    }

    return count;
}

pub fn p2<'a>(lines: Vec<(usize, usize, char, &'a str)>) -> i32 {
    let mut count = 0;

    for (p1, p2, c, s) in lines {
        let mut cs = s.chars();

        let um = cs.nth(p1 - 1).unwrap() == c;
        let dos = cs.nth(p2 - p1 - 1).unwrap() == c;

        count += ((um) ^ (dos)) as i32;
    }

    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<String> {
        vec![
            "1-3 a: abcde".to_string(),
            "1-3 b: cdefg".to_string(),
            "2-9 c: ccccccccc".to_string(),
        ]
    }

    #[test]
    fn p1_test() {
        let input = input();
        let input = parse_lines(&input);

        assert_eq!(p1(input), 2);
    }

    #[test]
    fn p2_test() {
        let input = input();
        let input = parse_lines(&input);

        assert_eq!(p2(input), 1);
    }
}
