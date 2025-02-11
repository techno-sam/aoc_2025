use std::fs;

use regex::Regex;

#[allow(dead_code)]
fn example() -> String {
    return "
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
".trim().to_owned();
}

const PART2: bool = false;

fn main() {
    println!("AOC 2024 Day 03");

    let contents = &fs::read_to_string("src/bin/day03/input.txt")
        .expect("Failed to read input");

    println!("Part 1: {}", part1(contents));

    if PART2 {
        println!("Part 2: {}", part2(contents));
    }
}

#[test]
fn test_p1() {
    assert_eq!(part1(&example()), 161);
}

#[test]
fn test_p2() {
    if PART2 {
        assert_eq!(part2(&example()), 42);
    }
}

fn part1(data: &String) -> usize {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    return data.split("\n")
        .map(|s| {
            let total: usize = re.captures_iter(s)
                .map(|c| c.extract())
                .map(|(_, [a, b])| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
                .map(|(a, b)| a * b)
                .sum();

            return total;
        })
        .sum();
}

fn part2(data: &String) -> usize {
    todo!();
}

