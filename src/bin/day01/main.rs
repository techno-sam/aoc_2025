use std::fs;

use nom::multi::many0;
use nom::{IResult, Parser};
use nom::branch::alt;
use nom::sequence::{preceded, terminated};
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

const PART2: bool = false;

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
        assert_eq!(part2(&example()), 42);
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
        many0(terminated(Self::parse, char('\n'))).parse(input)
    }
}

fn part1(data: &str) -> usize {
    let mut lock = Lock::new();
    let movements = Movement::parse_list(data).unwrap().1;

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
    todo!();
}

