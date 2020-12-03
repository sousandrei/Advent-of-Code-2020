use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn get_input(input: &str) -> Vec<i32> {
    input
        .split('\n')
        .map(|value| value.parse::<i32>().expect("Error parsing line"))
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(lines: &Vec<i32>) -> i32 {
    let mut lines = lines.clone();
    lines.sort_unstable();

    let mut i = 0;
    let mut j = lines.len() - 1;

    while i != j {
        let result = lines[i] + lines[j];

        if result < 2020 {
            i += 1;
        } else if result > 2020 {
            j -= 1;
        } else {
            return lines[i] * lines[j];
        }
    }

    return 0;
}

#[aoc(day1, part2)]
pub fn part2(lines: &Vec<i32>) -> i32 {
    let mut lines = lines.clone();
    lines.sort_unstable();

    for line in &lines {
        for line2 in &lines {
            for line3 in &lines {
                if line + line2 + line3 == 2020 {
                    return line * line2 * line3;
                }
            }
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<i32> {
        vec![1721, 979, 366, 299, 675, 1456]
    }

    #[test]
    fn parte1_test() {
        let input = input();

        assert_eq!(part1(&input), 1721 * 299);
    }

    #[test]
    fn part2_test() {
        let input = input();

        assert_eq!(part2(&input), 979 * 366 * 675);
    }
}
