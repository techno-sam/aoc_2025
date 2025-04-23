use std::fs;

use char_enum_impl::{char_enum, data_enum};

#[allow(dead_code)]
fn example() -> String {
    return "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
".trim().to_owned();
}

const PART2: bool = true;

fn main() {
    println!("AOC 2024 Day 07");

    let contents = fs::read_to_string("src/bin/day07/input.txt")
        .expect("Failed to read input");
    let contents = contents.trim();

    println!("Part 1: {}", part1(contents));

    if PART2 {
        println!("Part 2: {}", part2(contents));
    }
}

#[test]
fn test_p1() {
    assert_eq!(part1(&example()), 3749);
}

#[test]
fn test_concat() {
    assert_eq!(concat(1, 23), 123);
    assert_eq!(concat(15, 6), 156);
    assert_eq!(concat(72, 90), 7290);
}

#[test]
fn test_p2() {
    if PART2 {
        assert_eq!(part2(&example()), 11387);
    }
}

fn concat(a: usize, b: usize) -> usize {
    let mut mul = 1;
    while b >= mul {
        mul *= 10;
    }
    (a * mul) + b
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[data_enum(fn(usize, usize) -> usize)]
enum Operator {
    Add      = |a, b| a + b,
    Multiply = |a, b| a * b,
    Concat   = concat
}
impl Operator {
    fn next(self) -> Operator {
        match self {
            Operator::Add => Operator::Multiply,
            Operator::Multiply => Operator::Add,
            v => panic!("Invalid part 1 operator {:?}", v)
        }
    }

    fn next_part2(self) -> Operator {
        match self {
            Operator::Add => Operator::Multiply,
            Operator::Multiply => Operator::Concat,
            Operator::Concat => Operator::Add
        }
    }
}

struct PartialEquation {
    target: usize,
    inputs: Vec<usize>
}

impl From<&str> for PartialEquation {
    fn from(value: &str) -> Self {
        let (target, rest) = value.split_once(": ").unwrap();
        let target = target.parse().unwrap();
        let inputs = rest.split(" ").map(|v| v.parse().unwrap()).collect();

        PartialEquation { target, inputs }
    }
}
impl PartialEquation {
    fn validate(&self, ops: &Vec<Operator>) -> bool {
        assert_eq!(ops.len(), self.inputs.len() - 1);
        let mut result = self.inputs[0];
        let mut idx = 0;

        while idx + 1 < self.inputs.len() {
            let op = ops[idx];
            let b = self.inputs[idx + 1];
            idx += 1;

            result = op.value()(result, b);
        }

        result == self.target
    }

    fn can_be_valid(&self, part2: bool) -> bool {
        let mut ops: Vec<_> = std::iter::repeat_n(Operator::Add, self.inputs.len() - 1).collect();

        loop {
            if self.validate(&ops) {
                return true;
            }
            for i in (0..ops.len()).rev() {
                let next = if part2 { ops[i].next_part2() } else { ops[i].next() };
                ops[i] = next;
                if next != Operator::Add {
                    break;
                } else if i == 0 { // we've gone through all the operators
                    return false;
                }
            }
        }
    }
}

fn part1(data: &str) -> usize {
    let eqs: Vec<PartialEquation> = data.split("\n").map(|s| s.into()).collect();

    eqs.iter()
        .filter_map(|eq| if eq.can_be_valid(false) { Some(eq.target) } else { None }).sum()
}

fn part2(data: &str) -> usize {
    let eqs: Vec<PartialEquation> = data.split("\n").map(|s| s.into()).collect();

    eqs.iter()
        .filter_map(|eq| if eq.can_be_valid(true) { Some(eq.target) } else { None }).sum()
}

