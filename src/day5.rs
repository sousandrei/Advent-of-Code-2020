use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
pub fn get_input(input: &str) -> Vec<String> {
    input
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'F' => "0",
                    'B' => "1",
                    'L' => "0",
                    'R' => "1",
                    _ => {
                        println!("{:#?}", c);
                        panic!("error parsing line")
                    }
                })
                .collect()
        })
        .collect::<Vec<String>>()
}

#[aoc(day5, part1)]
pub fn part1(ps: &Vec<String>) -> i32 {
    let mut ids: Vec<i32> = ps
        .into_iter()
        .map(|p| isize::from_str_radix(p, 2).unwrap() as i32)
        .collect();

    ids.sort_unstable();

    *ids.last().unwrap()
}

#[aoc(day5, part2)]
pub fn part2(ps: &Vec<String>) -> i32 {
    let mut ids: Vec<i32> = ps
        .into_iter()
        .map(|p| isize::from_str_radix(p, 2).unwrap() as i32)
        .collect();

    ids.sort_unstable();

    let last = *ids.last().unwrap();

    let mut contained: bool = false;
    let i_c = &mut ids.into_iter();

    let mut c = i_c.next().unwrap();

    for i in 0..last {
        if c < i {
            c = i_c.next().unwrap();
        }

        if c == i {
            contained = true;
            continue;
        } else {
            if contained {
                return i;
            }
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    fn input() -> String {
        let v = vec!["BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"];

        v.join("\n")
    }

    #[test]
    fn p1_test() {
        let input = get_input(&input());
        assert_eq!(part1(&input), 820);
    }

    // #[test]
    // fn p2_test() {
    //     let input = get_input(&input());
    //     assert_eq!(part2(&input), 2);
    // }
}
