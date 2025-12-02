use std::{fmt::Display, fs};

use nom::{character::complete::{self, char}, multi::many0, sequence::{pair, preceded, separated_pair}, IResult, Parser};

#[allow(dead_code)]
fn example() -> String {
    "
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
".replace("\n", "").trim().to_owned()
}

const PART2: bool = false;

fn main() {
    println!("AOC 2025 Day 02");

    let contents = fs::read_to_string("src/bin/day02/input.txt")
        .expect("Failed to read input");
    let contents = contents.trim();

    println!("Part 1: {}", part1(contents));

    if PART2 {
        println!("Part 2: {}", part2(contents));
    }
}

#[test]
fn test_p1() {
    assert_eq!(part1(&example()), 1227775554);
}

/*#[test]
fn testp1_a() {
    assert_eq!(IdRange::new(12, 22).sum_invalid(), 22)
}*/

#[test]
fn test_p2() {
    if PART2 {
        assert_eq!(part2(&example()), 42);
    }
}

#[derive(Clone, Copy, Debug)]
struct IdRange {
    min: usize,
    max: usize
}
impl IdRange {
    fn new(min: usize, max: usize) -> Self {
        Self { min, max }
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        separated_pair(
            complete::usize,
            char('-'),
            complete::usize
        ).map(|(min, max)| Self::new(min, max))
        .parse(input)
    }

    fn parse_list(input: &str) -> IResult<&str, Vec<Self>> {
        pair(
            Self::parse,
            many0(preceded(char(','), Self::parse))
        ).map(|(first, mut vec)| { vec.insert(0, first); vec })
        .parse(input)
    }

    fn real_max(&self, base: usize) -> usize {
        self.max.min(base * (base - 2))
    }

    fn real_min(&self, exp: u32) -> usize {
        let base = 1 + 10usize.pow(exp);
        self.min.max(base * 10usize.pow(exp - 1))
    }

    fn count_multiples(&self, exp: u32, base: usize) -> usize {
        ((self.real_max(base) / base) + 1).saturating_sub(self.real_min(exp).div_ceil(base))
    }

    fn sum_multiples(&self, exp: u32) -> usize {
        let base = 1 + 10usize.pow(exp);
        if cfg!(test) {
            print!("Multiples[{}]({} -> {}) for {}:", base, self.real_min(exp), self.real_max(base), self)
        }

        let count = self.count_multiples(exp, base);
        let mut sum = 0;
        let mut highest = self.real_max(base) - self.real_max(base) % base;
        while highest >= self.real_min(exp) {
            if cfg!(test) { print!(" {}", highest); }
            sum += highest;
            highest -= base;
        }
        if cfg!(test) { println!("; count {}", count); }
        sum
    }

    fn sum_invalid(&self) -> usize {
        let mut sum = 0;
        let mut exp = 1;
        loop {
            let multiplier = 1 + 10usize.pow(exp);
            if multiplier > self.max {
                break;
            }
            sum += self.sum_multiples(exp);
            exp += 1;
        }
        sum
    }
}
impl Display for IdRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.min, self.max)
    }
}

fn part1(data: &str) -> usize {
    let ranges = {
        let (remainder, ranges) = IdRange::parse_list(data).unwrap();
        assert_eq!(remainder.len(), 0, "Non-empty remainder: '{}'", remainder);
        ranges
    };

    ranges.iter().map(IdRange::sum_invalid).sum()
}

fn part2(data: &str) -> usize {
    todo!();
}

