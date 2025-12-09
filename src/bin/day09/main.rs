use std::fs;

use nom::{character::complete, multi::separated_list1, sequence::separated_pair, IResult, Parser};
use utils::parse_complete;

#[allow(dead_code)]
fn example() -> String {
    "
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
".trim_matches('\n').to_owned()
}

const PART2: bool = false;

fn main() {
    println!("AOC 2025 Day 09");

    let contents = fs::read_to_string("src/bin/day09/input.txt")
        .expect("Failed to read input");
    let contents = contents.trim_matches('\n');

    println!("Part 1: {}", part1(contents));

    if PART2 {
        println!("Part 2: {}", part2(contents));
    }
}

#[test]
fn test_p1() {
    assert_eq!(part1(&example()), 50);
}

#[test]
fn test_p2() {
    if PART2 {
        assert_eq!(part2(&example()), 42);
    }
}

type Point = utils::Point<usize>;

fn area(a: &Point, b: &Point) -> usize {
    (1 + a.a.max(b.a) - a.a.min(b.a)) * (1 + a.b.max(b.b) - a.b.min(b.b))
}

struct Map {
    red_tiles: Vec<Point>
}
impl Map {
    fn parse_point(input: &str) -> IResult<&str, Point> {
        separated_pair(complete::usize, complete::char(','), complete::usize).map(|(a, b)| Point { a, b }).parse(input)
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(complete::line_ending, Self::parse_point).map(|red_tiles| Self { red_tiles }).parse(input)
    }

    fn max_area(&self) -> usize {
        let mut max_area = 0;

        for i in 0..self.red_tiles.len() {
            for j in i+1..self.red_tiles.len() {
                max_area = max_area.max(area(&self.red_tiles[i], &self.red_tiles[j]));
            }
        }

        max_area
    }
}

fn part1(data: &str) -> usize {
    let map = parse_complete(&mut Map::parse, data);
    map.max_area()
}

fn part2(data: &str) -> usize {
    todo!();
}

