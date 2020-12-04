use aoc_runner_derive::{aoc, aoc_generator};

pub struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }
}

#[aoc_generator(day4)]
pub fn get_input(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|lines| {
            let mut p = Passport::new();

            for line in lines.split(|c| c == '\n' || c == ' ') {
                let parts: Vec<&str> = line.split(':').collect();

                match parts[0] {
                    "byr" => p.byr = Some(parts[1].to_owned()),
                    "iyr" => p.iyr = Some(parts[1].to_owned()),
                    "eyr" => p.eyr = Some(parts[1].to_owned()),
                    "hgt" => p.hgt = Some(parts[1].to_owned()),
                    "hcl" => p.hcl = Some(parts[1].to_owned()),
                    "ecl" => p.ecl = Some(parts[1].to_owned()),
                    "pid" => p.pid = Some(parts[1].to_owned()),
                    "cid" => p.cid = Some(parts[1].to_owned()),
                    _ => panic!("oopsie"),
                }
            }

            p
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(ps: &Vec<Passport>) -> i32 {
    let mut count = 0;

    for p in ps {
        if p.byr.is_none()
            || p.iyr.is_none()
            || p.eyr.is_none()
            || p.hgt.is_none()
            || p.hcl.is_none()
            || p.ecl.is_none()
            || p.pid.is_none()
        {
            continue;
        }

        count += 1;
    }

    println!("part1: {}", count);
    return count;
}

#[aoc(day4, part2)]
pub fn part2(ps: &Vec<Passport>) -> i64 {
    let mut count = 0;

    for p in ps {
        if p.byr.is_none()
            || p.iyr.is_none()
            || p.eyr.is_none()
            || p.hgt.is_none()
            || p.hcl.is_none()
            || p.ecl.is_none()
            || p.pid.is_none()
        {
            continue;
        }

        if let Some(byr) = &p.byr {
            if byr.len() != 4 {
                continue;
            }

            let byr: i32 = byr.parse().unwrap();
            if byr < 1920 || byr > 2002 {
                continue;
            }
        }

        if let Some(iyr) = &p.iyr {
            if iyr.len() != 4 {
                continue;
            }

            let iyr: i32 = iyr.parse().unwrap();
            if iyr < 2010 || iyr > 2020 {
                continue;
            }
        }

        if let Some(eyr) = &p.eyr {
            if eyr.len() != 4 {
                continue;
            }

            let eyr: i32 = eyr.parse().unwrap();
            if eyr < 2020 || eyr > 2030 {
                continue;
            }
        }

        if let Some(hgt) = &p.hgt {
            if hgt.contains("cm") {
                let hgt: i32 = hgt.split("cm").collect::<String>().parse().unwrap();
                if hgt < 150 || hgt > 193 {
                    continue;
                }
            } else if hgt.contains("in") {
                let hgt: i32 = hgt.split("in").collect::<String>().parse().unwrap();
                if hgt < 59 || hgt > 76 {
                    continue;
                }
            } else {
                continue;
            }
        }

        if let Some(v) = &p.hcl {
            if v.get(0..1).unwrap() != "#" {
                continue;
            }

            if v.len() != 7 {
                continue;
            }

            for c in v.chars() {
                if !c.is_ascii_hexdigit() {
                    continue;
                }
            }
        }

        match p.ecl.as_ref().unwrap().as_str() {
            "amb" => {}
            "blu" => {}
            "brn" => {}
            "gry" => {}
            "grn" => {}
            "hzl" => {}
            "oth" => {}
            _ => continue,
        }

        if let Some(pid) = &p.pid {
            if pid.len() != 9 {
                continue;
            }

            for c in pid.chars() {
                if !c.is_ascii_digit() {
                    continue;
                }
            }

            if let Err(_) = pid.parse::<i32>() {
                continue;
            }
        }

        count += 1;
    }

    println!("part2: {}", count);
    return count;
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    fn input() -> String {
        let v = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
            "byr:1937 iyr:2017 cid:147 hgt:183cm",
            "",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
            "hcl:#cfa07d byr:1929",
            "",
            "hcl:#ae17e1 iyr:2013",
            "eyr:2024",
            "ecl:brn pid:760753108 byr:1931",
            "hgt:179cm",
            "",
            "hcl:#cfa07d eyr:2025 pid:166559648",
            "iyr:2011 ecl:brn hgt:59in",
        ];

        v.join("\n")
    }

    #[test]
    fn p1_test() {
        let input = get_input(&input());
        assert_eq!(part1(&input), 2);
    }

    // #[test]
    // fn p2_test() {
    //     let input = get_input(&input());
    //     assert_eq!(part2(&input), 2);
    // }
}
