use std::{collections::HashMap, fmt::Display, fs, ops::{Add, AddAssign}};

use nom::{character::complete, multi::separated_list1, sequence::separated_pair, IResult, Parser};
use utils::parse_complete;

// hint van Steef: Topological Sort

#[allow(dead_code)]
fn example() -> String {
    "
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
".trim_matches('\n').to_owned()
}

#[allow(dead_code)]
fn example2() -> String {
    "
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
".trim_matches('\n').to_owned()
}

const PART2: bool = true;

fn main() {
    println!("AOC 2025 Day 11");

    let contents = fs::read_to_string("src/bin/day11/input.txt")
        .expect("Failed to read input");
    let contents = contents.trim_matches('\n');

    println!("Part 1: {}", part1(contents));

    if PART2 {
        println!("Part 2: {}", part2(contents));
    }
}

#[test]
fn test_p1() {
    assert_eq!(part1(&example()), 5);
}

#[test]
fn test_p2() {
    if PART2 {
        assert_eq!(part2(&example2()), 2);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Label([char; 3]);
impl Label {
    fn parse(input: &str) -> IResult<&str, Self> {
        (nom::character::anychar, nom::character::anychar, nom::character::anychar)
            .map(|(a, b, c)| Self([a, b, c]))
            .parse(input)
    }
}
impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.0[0], self.0[1], self.0[2])
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct PathCounter {
    through_both: usize,
    through_fft: usize,
    through_dac: usize,
    through_neither: usize
}
impl PathCounter {
    fn total(self) -> usize {
        self.through_both + self.through_fft + self.through_dac + self.through_neither
    }
}
impl Add for PathCounter {
    type Output = PathCounter;

    fn add(self, rhs: Self) -> Self::Output {
        PathCounter {
            through_both: self.through_both + rhs.through_both,
            through_fft: self.through_fft + rhs.through_fft,
            through_dac: self.through_dac + rhs.through_dac,
            through_neither: self.through_neither + rhs.through_neither,
        }
    }
}
impl AddAssign for PathCounter {
    fn add_assign(&mut self, rhs: Self) {
        self.through_both += rhs.through_both;
        self.through_fft += rhs.through_fft;
        self.through_dac += rhs.through_dac;
        self.through_neither += rhs.through_neither;
    }
}

#[derive(Clone, Debug)]
struct Node {
    connections: Vec<Label>,
    /// the number of paths from here, to the target
    counts: PathCounter,
}
impl Node {
    fn new(connections: Vec<Label>) -> Self {
        Self { connections, counts: PathCounter::default() }
    }

    fn out() -> Self {
        Self {
            connections: Vec::new(),
            counts: PathCounter {
                through_neither: 1,
                ..PathCounter::default()
            }
        }
    }
}

#[derive(Clone, Debug)]
struct TopoRack {
    nodes: HashMap<Label, Node>,
    heights: Vec<(Label, usize)>,
    root: Label,
}
impl TopoRack {
    const YOU: Label = Label(['y', 'o', 'u']);
    const SVR: Label = Label(['s', 'v', 'r']);
    const OUT: Label = Label(['o', 'u', 't']);
    const DAC: Label = Label(['d', 'a', 'c']);
    const FFT: Label = Label(['f', 'f', 't']);

    fn parse_connection(input: &str) -> IResult<&str, (Label, Vec<Label>)> {
        separated_pair(Label::parse, nom::bytes::tag(": "), separated_list1(complete::char(' '), Label::parse)).parse(input)
    }

    fn parser<'a>(root: Label) -> impl Parser<&'a str, Output = Self, Error: nom::error::ParseError<&'a str> + std::fmt::Debug> {
        move |input: &'a str| Self::parse(input, root)
    }

    fn parse(input: &str, root: Label) -> IResult<&str, Self> {
        let (remainder, connections) = separated_list1(complete::line_ending, Self::parse_connection).parse(input)?;
        let mut nodes: HashMap<Label, Node> = connections.into_iter()
            .map(|(key, labels)| (key, Node::new(labels)))
            .collect();
        nodes.insert(Self::OUT, Node::out());

        let mut heights: HashMap<Label, usize> = HashMap::new();
        heights.insert(Self::OUT, 0);

        fn calculate_height(node: Label, nodes: &HashMap<Label, Node>, heights: &mut HashMap<Label, usize>) -> usize {
            if let Some(&height) = heights.get(&node) {
                height
            } else {
                let height = nodes.get(&node).unwrap_or_else(|| panic!("{} exists", node))
                    .connections.iter()
                    .map(|child| calculate_height(*child, nodes, heights) + 1)
                    .max().unwrap_or(0);
                heights.insert(node, height);
                height
            }
        }
        calculate_height(root, &nodes, &mut heights);

        let mut heights: Vec<(Label, usize)> = heights.into_iter().collect();
        heights.sort_unstable_by_key(|(_, height)| *height);

        Ok((remainder, Self { nodes, heights, root }))
    }

    fn print_heights(&self) {
        for (lbl, height) in &self.heights {
            println!("{} {}", lbl, height);
        }
    }

    fn count(&mut self) -> PathCounter {
        for (lbl, _) in &self.heights {
            let mut total = PathCounter::default();

            for child in &self.nodes.get(lbl).unwrap().connections {
                let child_counter = self.nodes.get(child).unwrap().counts;
                total += child_counter;
            }

            match *lbl {
                Self::DAC => {
                    total.through_both += total.through_fft;
                    total.through_fft = 0;

                    total.through_dac += total.through_neither;
                    total.through_neither = 0;
                }
                Self::FFT => {
                    total.through_both += total.through_dac;
                    total.through_dac = 0;

                    total.through_fft += total.through_neither;
                    total.through_neither = 0;
                }
                _ => {}
            }

            self.nodes.get_mut(lbl).unwrap().counts += total;
        }

        self.nodes.get(&self.root).unwrap().counts
    }
}

fn part1(data: &str) -> usize {
    let mut rack = parse_complete(&mut TopoRack::parser(TopoRack::YOU), data);
    if cfg!(test) {
        rack.print_heights();
    }
    rack.count().total()
}

fn part2(data: &str) -> usize {
    let mut rack = parse_complete(&mut TopoRack::parser(TopoRack::SVR), data);
    if cfg!(test) {
        rack.print_heights();
    }
    rack.count().through_both
}

