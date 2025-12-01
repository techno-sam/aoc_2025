use std::fs;

use nom::multi::many0;
use nom::{IResult, Parser};
use nom::branch::alt;
use nom::sequence::{pair, preceded};
use nom::character::complete::{self, char};

#[allow(dead_code)]
fn example() -> String {
    "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
".trim().to_owned()
}

const PART2: bool = true;

fn main() {
    println!("AOC 2025 Day 01");

    let contents = fs::read_to_string("src/bin/day01/input.txt").expect("Failed to read input");
    let contents = contents.trim();

    println!("Part 1: {}", part1(contents));

    if PART2 {
        println!("Part 2: {}", part2(contents));
    }
}

#[test]
fn test_p1() {
    assert_eq!(part1(&example()), 3);
}

#[test]
fn test_p2() {
    if PART2 {
        assert_eq!(part2(&example()), 6);
    }
}

#[derive(Clone, Copy, Debug)]
struct Lock {
    pos: usize,
    len: usize
}
impl Lock {
    fn new() -> Self {
        Self { pos: 50, len: 100 }
    }

    fn do_move(&mut self, mov: &Movement) {
        match mov {
            Movement::R(r) => self.pos = (self.pos + r) % self.len,
            Movement::L(l) => self.pos = (self.pos + self.len - (l % self.len)) % self.len,
        }
    }

    fn do_move2(&mut self, mov: &Movement) -> usize {
        match mov {
            Movement::R(r) => {
                let pos0 = self.pos;
                self.pos = (self.pos + r) % self.len;

                // we might have done some # of full loops
                // we might be at 0 now.
                // if now!=0 we might have made a partial loop past 0 (pos < pos0)

                let mut zeros = r / self.len;

                if pos0 != 0 && self.pos == 0 {
                    // don't double-count a full loop starting and ending at 0
                    zeros += 1;
                }

                if self.pos < pos0 && pos0 != 0 && self.pos != 0 {
                    zeros += 1;
                }

                zeros
            },
            Movement::L(l) => {
                let pos0 = self.pos;
                self.pos = (self.pos + self.len - (l % self.len)) % self.len;

                // we might have done some # of full loops
                // we might be at 0 now.
                // if now!=0 we might have made a partial loop past 0 (pos < pos0)

                let mut zeros = l / self.len;

                if pos0 != 0 && self.pos == 0 {
                    // don't double-count a full loop starting and ending at 0
                    zeros += 1;
                }

                if self.pos > pos0 && pos0 != 0 && self.pos != 0 {
                    zeros += 1;
                }

                zeros
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Movement {
    R(usize),
    L(usize),
}
impl Movement {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            preceded(char('R'), complete::usize).map(Self::R),
            preceded(char('L'), complete::usize).map(Self::L)
        )).parse(input)
    }

    fn parse_list(input: &str) -> IResult<&str, Vec<Self>> {
        pair(
            Self::parse,
            many0(preceded(char('\n'), Self::parse))
        ).map(|(single, mut vec)| {vec.insert(0, single); vec})
        .parse(input)
    }
}

fn part1(data: &str) -> usize {
    let mut lock = Lock::new();
    let (remainder, movements) = Movement::parse_list(data).unwrap();
    assert_eq!(remainder.len(), 0, "Non-empty remainder '{}'", remainder);

    let mut zeros = 0;
    for movement in &movements {
        lock.do_move(movement);
        if lock.pos == 0 {
            zeros += 1;
        }
    }

    zeros
}

fn part2(data: &str) -> usize {
    let mut lock = Lock::new();
    let (remainder, movements) = Movement::parse_list(data).unwrap();
    assert_eq!(remainder.len(), 0, "Non-empty remainder '{}'", remainder);

    let mut zeros = 0;
    for movement in &movements {
        let inc = lock.do_move2(movement);
        zeros += inc;

        if cfg!(test) {
            println!("{} zeros for {:?}, now at {}", inc, movement, lock.pos);
        }
    }

    zeros
}

