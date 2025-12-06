#!/bin/bash

function exit_me() {
	echo "Directory exists, exiting"
	exit -1
}

echo "Preparing day $1"

mkdir -v src/bin/day$1 || exit_me

cat -v <<EOF > src/bin/day$1/main.rs
use std::fs;

#[allow(dead_code)]
fn example() -> String {
    "

".trim_matches('\n').to_owned()
}

const PART2: bool = false;

fn main() {
    println!("AOC 2025 Day $1");

    let contents = fs::read_to_string("src/bin/day$1/input.txt")
        .expect("Failed to read input");
    let contents = contents.trim_matches('\n');

    println!("Part 1: {}", part1(contents));

    if PART2 {
        println!("Part 2: {}", part2(contents));
    }
}

#[test]
fn test_p1() {
    assert_eq!(part1(&example()), 42);
}

#[test]
fn test_p2() {
    if PART2 {
        assert_eq!(part2(&example()), 42);
    }
}

fn part1(data: &str) -> usize {
    todo!();
}

fn part2(data: &str) -> usize {
    todo!();
}

EOF

mv -v ~/Downloads/input src/bin/day$1/input.txt

echo "Done"
