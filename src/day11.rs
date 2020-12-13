use aoc_runner_derive::{aoc, aoc_generator};

use std::{fmt, write};
use std::{unimplemented, writeln};

#[derive(PartialEq, Clone)]
pub enum Seat {
    Empty,
    Occupied,
    Floor,
    EOL,
}

impl fmt::Debug for Seat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Seat::Empty => write!(f, "L"),
            Seat::Occupied => write!(f, "#"),
            Seat::Floor => write!(f, "."),
            Seat::EOL => write!(f, "|n\n"),
        }
    }
}

#[derive(Clone)]
pub struct Sentadao {
    line_size: i32,
    seating1: Vec<Seat>,
    seating2: Vec<Seat>,
    mem_bool: bool,
    pedantic: bool,
}

impl fmt::Debug for Sentadao {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== Sentadao ===")?;

        writeln!(f, "\nLine Size")?;
        writeln!(f, "{}", self.line_size)?;

        let seating;

        if !self.mem_bool {
            seating = &self.seating2;
        } else {
            seating = &self.seating1;
        }

        writeln!(f, "\nSeating")?;

        for line in seating.split(|seat| *seat == Seat::EOL) {
            for c in line {
                write!(f, "{:?} ", c)?;
            }
            writeln!(f, "|n")?;
        }
        writeln!(f, "================")?;
        Ok(())
    }
}

impl Sentadao {
    fn new(line_size: i32, seating: Vec<Seat>, pedantic: bool) -> Sentadao {
        Sentadao {
            line_size,
            seating1: seating.clone(),
            seating2: seating.clone(),
            mem_bool: true,
            pedantic,
        }
    }

    fn count_around_1(vec: &Vec<Seat>, x: i32, line_size: i32) -> i32 {
        let pos = vec![
            -line_size - 1,
            -line_size,
            -line_size + 1,
            -1,
            1,
            line_size - 1,
            line_size,
            line_size + 1,
        ];
        let mut count = 0;

        for p in pos {
            let check = x + p;
            if (check >= vec.len() as i32) || (check < 0) {
                continue;
            }
            if vec[check as usize] == Seat::Occupied {
                count += 1
            }
        }

        return count;
    }

    fn count_around_2(vec: &Vec<Seat>, x: i32, line_size: i32) -> i32 {
        let mut pos = vec![
            -line_size - 1,
            -line_size,
            -line_size + 1,
            -1,
            1,
            line_size - 1,
            line_size,
            line_size + 1,
        ];
        let mut count = 0;

        for n in 1..vec.len() {
            for p in pos.clone() {
                let check = x + p * n as i32;

                if (check >= vec.len() as i32)
                    || (check < 0)
                    || (vec[check as usize] == Seat::EOL)
                    || (vec[check as usize] == Seat::Empty)
                {
                    pos.retain(|&a| a != p);
                    continue;
                }

                if vec[check as usize] == Seat::Occupied {
                    count += 1;
                    pos.retain(|&a| a != p);
                }
            }
        }

        return count;
    }

    fn new_state(vec: &Vec<Seat>, x: usize, line_size: i32, pedantic: bool) -> Seat {
        let count;

        if pedantic {
            count = Sentadao::count_around_2(vec, x as i32, line_size);
        } else {
            count = Sentadao::count_around_1(vec, x as i32, line_size);
        }

        match vec[x] {
            Seat::EOL => Seat::EOL,
            Seat::Floor => Seat::Floor,
            // se ta vazio em volta, senta
            Seat::Empty => {
                if count == 0 {
                    return Seat::Occupied;
                }

                return Seat::Empty;
            }
            // se tem 4 ou mais sentado, levanta
            Seat::Occupied => {
                if pedantic {
                    if count > 4 {
                        return Seat::Empty;
                    }
                } else {
                    if count > 3 {
                        return Seat::Empty;
                    }
                }

                return Seat::Occupied;
            }
            _ => unimplemented!(),
        }
    }

    fn next(&mut self) -> i32 {
        let mut count = 0;

        let past_seating;
        let curr_seating;

        if !self.mem_bool {
            past_seating = &mut self.seating2;
            curr_seating = &mut self.seating1;
        } else {
            past_seating = &mut self.seating1;
            curr_seating = &mut self.seating2;
        }

        for (i, _c) in past_seating.clone().iter().enumerate() {
            let new_state = Sentadao::new_state(past_seating, i, self.line_size, self.pedantic);

            if new_state != curr_seating[i] {
                curr_seating[i] = new_state;
                count += 1;
            }
        }

        self.mem_bool = !self.mem_bool;

        return count;
    }

    fn get_seats(&self, seat: Seat) -> i32 {
        let seating;

        if !self.mem_bool {
            seating = &self.seating2;
        } else {
            seating = &self.seating1;
        }

        return seating.iter().filter(|x| **x == seat).count() as i32;
    }
}

#[aoc_generator(day11)]
pub fn get_input(input: &str) -> (i32, Vec<Seat>) {
    let line_size = input.chars().position(|c| c == '\n').unwrap() as i32 + 1;

    let chars = input
        .chars()
        .map(|c| match c {
            'L' => Seat::Empty,
            '.' => Seat::Floor,
            '\n' => Seat::EOL,
            _ => unimplemented!(),
        })
        .collect();

    return (line_size, chars);
}

#[aoc(day11, part1)]
pub fn part1(input: &(i32, Vec<Seat>)) -> i32 {
    let (line_size, chars) = input;
    let mut s = Sentadao::new(*line_size, chars.clone(), false);

    let mut changes;

    loop {
        changes = s.next();

        if changes < 1 {
            break;
        }
    }

    return s.get_seats(Seat::Occupied);
}

#[aoc(day11, part2)]
pub fn part2(input: &(i32, Vec<Seat>)) -> i32 {
    let (line_size, chars) = input;
    let mut s = Sentadao::new(*line_size, chars.clone(), true);

    let mut changes;
    let mut runs = 0;

    loop {
        changes = s.next();

        if changes < 1 {
            break;
        }

        runs += 1;
    }

    return s.get_seats(Seat::Occupied);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
    }

    #[test]
    fn p1_test() {
        let input = get_input(input());
        assert_eq!(part1(&input), 37);
    }

    #[test]
    fn p2_test() {
        let input = get_input(input());
        assert_eq!(part2(&input), 26);
    }
}
