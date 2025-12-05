use std::{fs, ops::RangeInclusive};

use nom::{character::complete, multi::separated_list1, sequence::separated_pair, IResult, Parser};
use utils::parse_complete;

#[allow(dead_code)]
fn example() -> String {
    "
3-5
10-14
16-20
12-18

1
5
8
11
17
32
".trim().to_owned()
}

const PART2: bool = false;

fn main() {
    println!("AOC 2025 Day 05");

    let contents = fs::read_to_string("src/bin/day05/input.txt")
        .expect("Failed to read input");
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
struct FreshRange {
    min: usize,
    max: usize
}
impl FreshRange {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_pair(complete::usize, complete::char('-'), complete::usize)
            .map(|(min, max)| Self { min, max })
            .parse(input)
    }

    fn min(&self) -> usize {
        self.min
    }

    fn max(&self) -> usize {
        self.max
    }

    fn contains(&self, other: usize) -> bool {
        self.min <= other && other <= self.max
    }

    fn merge(&self, other: &FreshRange) -> Option<FreshRange> {
        if self.min > other.max {
            None
        } else if other.min > self.max {
            None
        } else {
            Some(FreshRange { min: self.min.min(other.min), max: self.max.max(other.max) })
        }
    }
}

#[derive(Clone, Debug)]
struct Kitchen {
    fresh: Vec<FreshRange>,
    available: Vec<usize>
}
impl Kitchen {
    fn parse(input: &str) -> IResult<&str, Self> {
        let fresh_parser = separated_list1(complete::line_ending, FreshRange::parse);
        let available_parser = separated_list1(complete::line_ending, complete::usize);
        let full_parser = separated_pair(fresh_parser, nom::bytes::tag("\n\n"), available_parser);

        full_parser.map(|(fresh, available)| Self { fresh, available }).parse(input)
    }

    fn unify_fresh(&mut self) {
        self.fresh.sort_by_key(FreshRange::min);
        self.fresh = self.fresh.iter().fold(vec![], |mut v, fr| {
            let prev = v.last();
            match prev {
                Some(prev) => {
                    match prev.merge(fr) {
                        Some(merged) => {
                            v.pop();
                            v.push(merged);
                        }
                        None => v.push(*fr)
                    }
                }
                None => v.push(*fr)
            }
            v
        });
    }

    fn contains(&self, ingredient: usize) -> bool {
        self.fresh.iter().any(|fr| fr.contains(ingredient))
    }

    fn count_fresh(&self) -> usize {
        self.available.iter().filter(|v| self.contains(**v)).count()
    }
}

fn part1(data: &str) -> usize {
    let mut kitchen = parse_complete(&mut Kitchen::parse, data);
    kitchen.unify_fresh();
    kitchen.count_fresh()
}

fn part2(data: &str) -> usize {
    todo!();
}

