use std::{collections::HashSet, fmt::Display, fs};

use nom::{character::complete::{self, char}, multi::many0, sequence::{pair, preceded, separated_pair}, IResult, Parser};

#[allow(dead_code)]
fn example() -> String {
    "
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
".replace("\n", "").trim().to_owned()
}

const PART2: bool = true;

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
        assert_eq!(part2(&example()), 4174379265);
    }
}

#[test]
fn test_invalidator() {
    assert_eq!(Invalidator::new(1, 2).divisor(), 11);
    assert_eq!(Invalidator::new(1, 3).divisor(), 111);

    assert_eq!(Invalidator::new(2, 2).divisor(), 101);
    assert_eq!(Invalidator::new(2, 3).divisor(), 10101);

    assert_eq!(Invalidator::new(4, 2).divisor(), 10001);
    assert_eq!(Invalidator::new(4, 3).divisor(), 100010001);
}

#[derive(Clone, Copy, Debug)]
pub struct Invalidator {
    part_len: u32,
    repeats: u32
}
impl Invalidator {
    fn new(part_len: u32, repeats: u32) -> Self {
        assert!(repeats >= 2, "invalidator must repeat");
        Self { part_len, repeats }
    }

    fn divisor(&self) -> usize {
        // a 1 for every repeat, separated by (part_len - 1) zeros
        let mul = 10usize.pow(self.part_len);
        let mut out = 1;

        for _ in 1..self.repeats {
            out *= mul;
            out += 1;
        }

        out
    }

    fn max(&self) -> usize {
        // example for part_len 3 and repeats 2:
        // divisor = 1001
        // maximum legal = 1001 * 999 = 999999
        let mul = 10usize.pow(self.part_len);
        self.divisor() * (mul - 1)
    }

    fn min(&self) -> usize {
        // example for part_len 3 and repeats 2:
        // divisor = 1001
        // minimum legal = 1001 * 100 = 100100
        self.divisor() * 10usize.pow(self.part_len - 1)
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

    /* PART 1 */

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
        if cfg!(test) { println!(); }

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

    /* PART 2 */

    fn collect_part2(&self, inv: Invalidator, invalid: &mut HashSet<usize>) {
        let min = self.min.max(inv.min());
        let max = self.max.min(inv.max());
        let divisor = inv.divisor();

        if cfg!(test) {
            print!("Multiples[[{} {}] {}]({} -> {}) for {}:", inv.part_len, inv.repeats, divisor, min, max, self)
        }

        let mut highest = max - max % divisor;
        while highest >= min {
            if cfg!(test) { print!(" {}", highest); }
            invalid.insert(highest);
            highest -= divisor;
        }
        if cfg!(test) { println!(); }
    }

    fn sum_part2(&self) -> usize {
        if cfg!(test) { println!(); }

        let mut invalid = HashSet::<usize>::new();
        'Outer: for exp in 1.. {
            'Inner: for repeats in 2.. {
                let inv = Invalidator::new(exp, repeats);

                if inv.min() > self.max {
                    if repeats == 2 {
                        break 'Outer;
                    } else {
                        break 'Inner;
                    }
                }

                if inv.max() < self.min {
                    continue 'Inner;
                }

                self.collect_part2(inv, &mut invalid);
            }
        }

        invalid.into_iter().sum()
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
    let ranges = {
        let (remainder, ranges) = IdRange::parse_list(data).unwrap();
        assert_eq!(remainder.len(), 0, "Non-empty remainder: '{}'", remainder);
        ranges
    };

    ranges.iter().map(IdRange::sum_part2).sum()
}

