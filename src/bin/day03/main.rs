use std::{fs, ops::Range};

#[allow(dead_code)]
fn example() -> String {
    "
987654321111111
811111111111119
234234234234278
818181911112111
".trim().to_owned()
}

const PART2: bool = false;

fn main() {
    println!("AOC 2025 Day 03");

    let contents = fs::read_to_string("src/bin/day03/input.txt")
        .expect("Failed to read input");
    let contents = contents.trim();

    println!("Part 1: {}", part1(contents));

    if PART2 {
        println!("Part 2: {}", part2(contents));
    }
}

#[test]
fn test_p1() {
    assert_eq!(part1(&example()), 357);
}

#[test]
fn test_p2() {
    if PART2 {
        assert_eq!(part2(&example()), 42);
    }
}

#[derive(Clone, Debug)]
struct Bank {
    batteries: Vec<u8>
}
impl Bank {
    fn parse(input: &str) -> Self {
        let batteries = input.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
        Self { batteries }
    }

    fn parse_lines(input: &str) -> Vec<Self> {
        input.lines().map(Self::parse).collect()
    }

    fn highest_in_range(&self, range: Range<usize>) -> usize {
        let mut best = 0;
        let mut best_idx = 0;
        for i in range {
            let v = self.batteries[i];
            if v > best {
                best = v;
                best_idx = i;
            }

            if v == 9 {
                break;
            }
        }

        best_idx
    }

    fn max_joltage(&self) -> usize {
        let i10 = self.highest_in_range(0..self.batteries.len() - 1);
        let i01 = self.highest_in_range((i10 + 1)..self.batteries.len());

        let joltage = 10*(self.batteries[i10] as usize) + (self.batteries[i01] as usize);

        if cfg!(test) {
            print!("Max joltage for ");
            self.batteries.iter().for_each(|v| print!("{}", v));
            println!(": {}", joltage);
        }

        joltage
    }
}

fn part1(data: &str) -> usize {
    let banks = Bank::parse_lines(data);

    banks.iter().map(Bank::max_joltage).sum()
}

fn part2(data: &str) -> usize {
    todo!();
}

