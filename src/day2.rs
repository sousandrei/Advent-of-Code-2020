use aoc_runner_derive::{aoc, aoc_generator};

pub struct Input {
    min: usize,
    max: usize,
    c: char,
    s: String,
}

fn parse_input(input: &str) -> Vec<Input> {
    input
        .split('\n')
        .map(|line| {
            let v: Vec<&str> = line.split(": ").collect();

            let args: Vec<&str> = v[0].split(" ").collect();

            let c: char = args[1].parse().unwrap();

            let counts: Vec<&str> = args[0].split('-').collect();

            let min: usize = counts[0].parse().unwrap();
            let max: usize = counts[1].parse().unwrap();

            let s = v[1];

            Input {
                min,
                max,
                c,
                s: s.to_string(),
            }
        })
        .collect()
}

#[aoc_generator(day2)]
pub fn get_input(input: &str) -> Vec<Input> {
    parse_input(input)
}

#[aoc(day2, part1)]
pub fn part1(lines: &Vec<Input>) -> i32 {
    let mut count = 0;

    for Input { min, max, c, s } in lines {
        let char_count = s.matches(*c).count();

        if char_count >= *min && char_count <= *max {
            count += 1;
        }
    }

    return count;
}

#[aoc(day2, part2)]
pub fn part2(lines: &Vec<Input>) -> i32 {
    let mut count = 0;

    for Input { min, max, c, s } in lines {
        let mut cs = s.chars();

        let um = cs.nth(min - 1).unwrap() == *c;
        let dos = cs.nth(max - min - 1).unwrap() == *c;

        count += ((um) ^ (dos)) as i32;
    }

    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "1-3 a: abcde\n\
1-3 b: cdefg\n\
2-9 c: ccccccccc"
    }

    #[test]
    fn part1_test() {
        let input = parse_input(input());
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn parte2_test() {
        let input = parse_input(input());
        assert_eq!(part2(&input), 1);
    }
}
