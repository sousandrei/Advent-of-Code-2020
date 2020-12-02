use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn run() {
    let lines = get_input();

    println!("{}", p1(&lines));
    println!("{}", p2(&lines));
}

pub fn get_input() -> Vec<i32> {
    const FILENAME: &str = "data/day1.txt";
    let file = File::open(Path::new(FILENAME)).expect("Error opening file");
    let lines = BufReader::new(file).lines();

    let lines: Vec<i32> = lines
        .map(|value| {
            value
                .expect("Error getting line")
                .parse::<i32>()
                .expect("Error parsing line")
        })
        .collect();

    return lines;
}

pub fn p1(lines: &Vec<i32>) -> i32 {
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

pub fn p2(lines: &Vec<i32>) -> i32 {
    let mut lines = lines.clone();
    lines.sort_unstable();

    //OH wow, disgusting
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

    #[test]
    fn p1_test() {
        let input = vec![1721, 979, 366, 299, 675, 1456];

        assert_eq!(p1(&input), 1721 * 299);
    }

    #[test]
    fn p2_test() {
        let input = vec![1721, 979, 366, 299, 675, 1456];

        assert_eq!(p2(&input), 979 * 366 * 675);
    }
}
