use std::fs;

use regex::Regex;

#[allow(dead_code)]
fn example() -> String {
    return "
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
".trim().to_owned();
}

#[allow(dead_code)]
fn example2() -> String {
    return "
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
".trim().to_owned();
}

const PART2: bool = true;

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
        assert_eq!(part2(&example2()), 48);
    }
}

#[test]
fn test_p2_b() {
    if PART2 {
        assert_eq!(part2(" mul(1,2)don't()badstuffmul(30,10)

stillbadmul(2,100)do()mul(2,4) "), 10);
    }
}

fn part1(data: &str) -> usize {
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

fn part2(data: &str) -> usize {
    let re = Regex::new(r"(mul)\((\d+),(\d+)\)|(do\(\))|(don't\(\))").unwrap();

    let mut enabled = true;

    return data.split("\n")
        .map(|s| {
            let mut total = 0;

            for capture in re.captures_iter(s) {
                if let Some(_) = capture.get(1) { // mul(a,b)
                    if enabled {
                        let a = capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
                        let b = capture.get(3).unwrap().as_str().parse::<usize>().unwrap();
                        total += a * b;
                    }
                } else if let Some(_) = capture.get(4) { // do()
                    enabled = true;
                } else if let Some(_) = capture.get(5) { // don't()
                    enabled = false;
                }
            }

            return total;
        })
        .sum();
}

