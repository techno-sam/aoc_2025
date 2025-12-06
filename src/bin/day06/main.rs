use std::fs;

use char_enum_impl::{char_enum, data_enum};
use nom::{character::complete, combinator::map_res, multi::separated_list1, sequence::{delimited, separated_pair}, IResult, Parser};
use utils::parse_complete;

#[allow(dead_code)]
fn example() -> String {
    "
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
".trim().to_owned()
}

const PART2: bool = false;

fn main() {
    println!("AOC 2025 Day 06");

    let contents = fs::read_to_string("src/bin/day06/input.txt")
        .expect("Failed to read input");
    let contents = contents.trim();

    println!("Part 1: {}", part1(contents));

    if PART2 {
        println!("Part 2: {}", part2(contents));
    }
}

#[test]
fn parsing_inputs() {
    assert_eq!(InputData::parse_inputs("1 2 3\n4 5 6"), Ok(("", vec![vec![1, 2, 3], vec![4, 5, 6]])));
    assert_eq!(InputData::parse_inputs("1  2 3\n4 5    6"), Ok(("", vec![vec![1, 2, 3], vec![4, 5, 6]])), "Irregular spacing");
    assert_eq!(InputData::parse_inputs(" 1  2 3\n4 5    6"), Ok(("", vec![vec![1, 2, 3], vec![4, 5, 6]])), "Leading space");
    assert_eq!(InputData::parse_inputs("1  2 3\n4 5    6 "), Ok(("", vec![vec![1, 2, 3], vec![4, 5, 6]])), "Trailing space");
    assert_eq!(InputData::parse_inputs(" 1  2 3\n4 5    6 "), Ok(("", vec![vec![1, 2, 3], vec![4, 5, 6]])), "Leading and trailing space");
    assert_eq!(InputData::parse_inputs(" 1  2 3 \n 4 5    6 "), Ok(("", vec![vec![1, 2, 3], vec![4, 5, 6]])), "Leading and trailing and internal space");
}

#[test]
fn parsing_ops() {
    assert_eq!(Op::parse("+"), Ok(("", Op::Add)));
    assert_eq!(Op::parse("*"), Ok(("", Op::Mul)));
    assert_eq!(InputData::parse_ops("* + + * * + *"), Ok(("", vec![Op::Mul, Op::Add, Op::Add, Op::Mul, Op::Mul, Op::Add, Op::Mul])));
    assert_eq!(InputData::parse_ops(" * + + * * + *"), Ok(("", vec![Op::Mul, Op::Add, Op::Add, Op::Mul, Op::Mul, Op::Add, Op::Mul])), "Leading space");
    assert_eq!(InputData::parse_ops("* + + * * + * "), Ok(("", vec![Op::Mul, Op::Add, Op::Add, Op::Mul, Op::Mul, Op::Add, Op::Mul])), "Trailing space");
    assert_eq!(InputData::parse_ops(" * + + * * + * "), Ok(("", vec![Op::Mul, Op::Add, Op::Add, Op::Mul, Op::Mul, Op::Add, Op::Mul])), "Leading and trailing space");
}

#[test]
fn test_p1() {
    assert_eq!(part1(&example()), 4277556);
}

#[test]
fn test_p2() {
    if PART2 {
        assert_eq!(part2(&example()), 42);
    }
}

#[derive(Clone, Copy, Debug)]
struct Operation {
    op: fn(usize, usize) -> usize,
    identity: usize,
}

#[char_enum]
#[data_enum[Operation]]
enum Op {
    Add = ('+', Operation { op: |a, b| a + b, identity: 0 }),
    Mul = ('*', Operation { op: |a, b| a * b, identity: 1 }),
}
impl Op {
    fn parse(input: &str) -> IResult<&str, Op> {
        map_res(complete::anychar, Op::try_decode).parse(input)
    }
}

#[derive(Clone, Debug)]
struct InputData {
    inputs: Vec<Vec<usize>>,
    ops: Vec<Op>
}
impl InputData {
    fn parse_ops(input: &str) -> IResult<&str, Vec<Op>> {
        delimited(
            complete::space0,
            separated_list1(complete::space1, Op::parse),
            complete::space0,
        ).parse(input)
    }

    fn parse_inputs(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
        separated_list1(
            complete::line_ending,
            delimited(
                complete::space0,
                separated_list1(complete::space1, complete::usize),
                complete::space0,
            )
        ).parse(input)
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        separated_pair(
            Self::parse_inputs,
            complete::line_ending,
            Self::parse_ops
        ).map(|(inputs, ops)| Self { inputs, ops }).parse(input)
    }
}

#[derive(Clone, Debug)]
struct HomeworkColumn {
    inputs: Vec<usize>,
    op: Op
}
impl HomeworkColumn {
    fn empty(op: Op) -> Self {
        Self { inputs: vec![], op }
    }

    fn calculate(&self) -> usize {
        let op = self.op.value();
        self.inputs.iter().fold(op.identity, |a, b| (op.op)(a, *b))
    }
}

#[derive(Clone, Debug)]
struct Homework {
    columns: Vec<HomeworkColumn>
}
impl Homework {
    fn grand_total(&self) -> usize {
        self.columns.iter().map(HomeworkColumn::calculate).sum()
    }
}

impl From<InputData> for Homework {
    fn from(value: InputData) -> Self {
        let columns = value.ops.into_iter()
            .map(HomeworkColumn::empty)
            .enumerate()
            .map(|(i, mut col)| {
                value.inputs.iter()
                    .for_each(|v| col.inputs.push(v[i]));
                col
            })
            .collect();

        Self { columns }
    }
}

fn part1(data: &str) -> usize {
    let homework: Homework = parse_complete(&mut InputData::parse, data).into();
    homework.grand_total()
}

fn part2(data: &str) -> usize {
    todo!();
}

