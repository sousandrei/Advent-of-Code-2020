use aoc_runner_derive::{aoc, aoc_generator};

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &Vec<i64>, modulii: &Vec<i64>) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

#[aoc_generator(day13)]
pub fn get_input(input: &str) -> Vec<i64> {
    input
        .split(|c| c == '\n' || c == ',')
        .map(|line| line.parse::<i64>().unwrap_or(-1))
        .collect()
}

#[aoc(day13, part1)]
pub fn part1(input: &Vec<i64>) -> i64 {
    let (time, buses) = input.split_at(1);
    let time = time[0];

    let mut next_line = 90;
    let mut eta = time;

    for &b in buses {
        if b < 0 {
            continue;
        }

        let time_next_bus = b - (time % b);

        if time_next_bus < eta {
            eta = time_next_bus;
            next_line = b;
        }
    }

    return next_line * eta;
}

#[aoc(day13, part2)]
pub fn part2(input: &Vec<i64>) -> i64 {
    let (_, buses) = input.split_at(1);

    let mut modulii: Vec<i64> = Vec::new();
    let mut residues: Vec<i64> = Vec::new();

    for (i, &b) in buses.iter().enumerate() {
        if b > 0 {
            modulii.push(b);
            residues.push(b - i as i64);
        }
    }

    return chinese_remainder(&residues, &modulii).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let input = get_input("939\n7,13,x,x,59,x,31,19");
        assert_eq!(part1(&input), 295);
    }

    #[test]
    fn p2_test() {
        let input = get_input("1000509\n1789,37,47,1889");
        assert_eq!(part2(&input), 1202161486);
    }
}
