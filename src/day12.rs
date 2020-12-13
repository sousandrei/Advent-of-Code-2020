use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
    LEFT,
    RIGHT,
    FORWARD,
}

#[derive(Debug)]
pub struct Instruction {
    dir: Direction,
    amount: i32,
}

#[aoc_generator(day12)]
pub fn get_input(input: &str) -> Vec<Instruction> {
    input
        .split('\n')
        .map(|line| {
            let (dir, amount) = line.split_at(1);

            return Instruction {
                dir: match dir {
                    "N" => Direction::NORTH,
                    "S" => Direction::SOUTH,
                    "E" => Direction::EAST,
                    "W" => Direction::WEST,
                    "L" => Direction::LEFT,
                    "R" => Direction::RIGHT,
                    "F" => Direction::FORWARD,
                    _ => unimplemented!(),
                },
                amount: amount.parse::<i32>().unwrap(),
            };
        })
        .collect()
}

#[aoc(day12, part1)]
pub fn part1(input: &Vec<Instruction>) -> i32 {
    let mut state = 1;
    let directions = [
        Direction::NORTH,
        Direction::EAST,
        Direction::SOUTH,
        Direction::WEST,
    ];

    let mut north = 0;
    let mut east = 0;

    for Instruction { dir, amount } in input {
        let mut dir = *dir;
        let change_state = *amount as usize / 90;

        match dir {
            Direction::RIGHT => state = (state + change_state) % 4,
            Direction::LEFT => state = (state - change_state + 4) % 4,
            Direction::FORWARD => dir = directions[state],
            _ => {}
        };

        match dir {
            Direction::NORTH => north += amount,
            Direction::SOUTH => north -= amount,
            Direction::EAST => east += amount,
            Direction::WEST => east -= amount,
            _ => {}
        };
    }

    return north.abs() + east.abs();
}

#[derive(Debug)]
pub struct Coordinate {
    e: i32,
    n: i32,
}

impl Coordinate {
    fn add(&mut self, dir: Direction, amount: i32) {
        match dir {
            Direction::NORTH => self.n += amount,
            Direction::SOUTH => self.n -= amount,
            Direction::EAST => self.e += amount,
            Direction::WEST => self.e -= amount,
            _ => {}
        };
    }

    fn rot_right(&mut self) {
        let e = self.e;

        self.e = self.n;
        self.n = -e;
    }

    fn rot_left(&mut self) {
        let e = self.e;

        self.e = -self.n;
        self.n = e;
    }

    fn rotate(&mut self, dir: Direction, amount: i32) {
        let amount = amount as usize / 90;

        for _ in 0..amount {
            match dir {
                Direction::RIGHT => self.rot_right(),
                Direction::LEFT => self.rot_left(),
                _ => {}
            };
        }
    }
}

#[aoc(day12, part2)]
pub fn part2(input: &Vec<Instruction>) -> i32 {
    let mut waypoint = Coordinate { n: 1, e: 10 };

    let mut east = 0;
    let mut north = 0;

    for Instruction { dir, amount } in input {
        let dir = *dir;
        let amount = *amount;

        match dir {
            Direction::NORTH => waypoint.add(dir, amount),
            Direction::SOUTH => waypoint.add(dir, amount),
            Direction::EAST => waypoint.add(dir, amount),
            Direction::WEST => waypoint.add(dir, amount),
            Direction::RIGHT => waypoint.rotate(dir, amount),
            Direction::LEFT => waypoint.rotate(dir, amount),
            Direction::FORWARD => {
                east += waypoint.e * amount;
                north += waypoint.n * amount;
            }
        };
    }

    return north.abs() + east.abs();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "F10
N3
F7
R90
F11"
    }

    #[test]
    fn p1_test() {
        let input = get_input(input());
        assert_eq!(part1(&input), 25);
    }

    #[test]
    fn p2_test() {
        let input = get_input(input());
        assert_eq!(part2(&input), 286);
    }
}
