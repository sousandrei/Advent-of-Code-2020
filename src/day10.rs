use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
pub fn get_input(input: &str) -> Vec<i16> {
    input
        .split("\n")
        .map(|line| line.parse::<i16>().unwrap())
        .collect()
}

#[aoc(day10, part1)]
pub fn part1(code: &Vec<i16>) -> u16 {
    let mut code = code.clone();
    code.sort_unstable();
    let mut j1 = 0;
    let mut j3 = 1;
    let mut last = *code.first().unwrap();

    match last {
        3 => j3 += 1,
        1 => j1 += 1,
        _ => unreachable!(),
    }

    for adp in code {
        if adp == last {
            continue;
        }

        match adp - last {
            3 => j3 += 1,
            1 => j1 += 1,
            _ => unreachable!(),
        }

        last = adp;
    }

    return j1 * j3;
}

#[aoc(day10, part2)]
pub fn part2(code: &Vec<i16>) -> i64 {
    let mut code = code.clone();
    code.sort_unstable();

    let mut seq: Vec<i16> = Vec::new();

    let mut last = 0;
    let mut count = 0;
    let mut inside_seq = true;

    for c in code {
        if c == (last + 1) {
            inside_seq = true;
        }

        if inside_seq {
            if c == (last + 1) {
                count += 1;
            } else {
                inside_seq = false;
                seq.push(count);
                count = 0;
            }
        }

        last = c;
    }
    if inside_seq {
        seq.push(count);
    }

    seq.iter()
        .map(|v| match v {
            4 => 7,
            3 => 4,
            _ => *v,
        })
        .collect::<Vec<i16>>()
        .iter()
        .fold(1, |acc, v| acc * (*v as i64))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3"
    }

    #[test]
    fn p1_test() {
        let input = get_input(input());
        assert_eq!(part1(&input), 220);
    }

    #[test]
    fn p2_test() {
        let input = get_input(input());
        assert_eq!(part2(&input), 19208);
    }
}
