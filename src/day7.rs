use aoc_runner_derive::{aoc, aoc_generator};

use regex::Regex;
use std::collections::BTreeSet;
use std::collections::HashMap;

#[aoc_generator(day7)]
pub fn get_input(input: &str) -> Vec<String> {
    input.split("\n").map(|lines| lines.to_string()).collect()
}

#[aoc(day7, part1)]
pub fn part1(lines: &Vec<String>) -> i32 {
    let rx = Regex::new(r"(?:^|\d )(\w+ \w+) bag").unwrap();

    let mut hm = HashMap::new();

    for cs in lines.into_iter() {
        let mut caps = rx.captures_iter(cs);

        let base = caps.next().unwrap().get(1).unwrap().as_str();

        for cap in caps {
            let cap = cap.get(1).unwrap().as_str();
            let bt = hm.entry(cap).or_insert(BTreeSet::new());

            bt.insert(base);
        }
    }

    let res = recu(
        "shiny gold",
        &hm.get("shiny gold").unwrap(),
        &hm,
        &BTreeSet::new(),
    );

    return (res.len() as i32) - 1;
}

fn recu<'a>(
    name: &'a str,
    bt: &'a BTreeSet<&'a str>,
    hm: &'a HashMap<&'a str, BTreeSet<&'a str>>,
    in_solved: &BTreeSet<&'a str>,
) -> BTreeSet<&'a str> {
    let mut solved = BTreeSet::new();

    if in_solved.contains(name) {
        return solved;
    }

    solved.insert(name);

    for elem in in_solved {
        solved.insert(elem.clone());
    }

    for elem in bt {
        if let Some(elem_bt) = hm.get(elem) {
            solved.append(&mut recu(elem, elem_bt, hm, &solved));
        } else {
            solved.insert(elem.clone());
        }
    }

    return solved;
}

#[aoc(day7, part2)]
pub fn part2(lines: &Vec<String>) -> i32 {
    let rx = Regex::new(r"(?:^|(?:(\d+) ))(\w+ \w+) bag").unwrap();

    let mut hm = HashMap::new();

    for cs in lines.into_iter() {
        let mut caps = rx.captures_iter(cs);

        let base = caps.next().unwrap().get(2).unwrap().as_str();
        let bt = hm.entry(base).or_insert(HashMap::new());

        for cap in caps {
            let amount = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let color = cap.get(2).unwrap().as_str();

            bt.insert(color, amount);
        }
    }

    let res = recu2(&hm.get("shiny gold").unwrap(), &hm);

    return (res as i32) - 1;
}

fn recu2(elem: &HashMap<&str, usize>, hm: &HashMap<&str, HashMap<&str, usize>>) -> usize {
    let mut count = 1;

    for (name, amount) in elem {
        count += amount * recu2(hm.get(name).unwrap(), hm);
    }

    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let input = get_input("light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.");
        assert_eq!(part1(&input), 4);
    }

    #[test]
    fn p2_test() {
        let input = get_input("shiny gold bags contain 2 dark red bags.\ndark red bags contain 2 dark orange bags.\ndark orange bags contain 2 dark yellow bags.\ndark yellow bags contain 2 dark green bags.\ndark green bags contain 2 dark blue bags.\ndark blue bags contain 2 dark violet bags.\ndark violet bags contain no other bags.");
        assert_eq!(part2(&input), 126);
    }
}
