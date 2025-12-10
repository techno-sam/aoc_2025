use std::fs;

use char_enum_impl::{char_enum, data_enum};
use nom::{character::complete, combinator::map_res, multi::{many1, separated_list1}, sequence::{delimited, separated_pair}, IResult, Parser};
use utils::{parse_complete, DijkstraData, DijkstraNode};

#[allow(dead_code)]
fn example() -> String {
    "
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
".trim_matches('\n').to_owned()
}

const PART2: bool = false;

fn main() {
    println!("AOC 2025 Day 10");

    let contents = fs::read_to_string("src/bin/day10/input.txt")
        .expect("Failed to read input");
    let contents = contents.trim_matches('\n');

    println!("Part 1: {}", part1(contents));

    if PART2 {
        println!("Part 2: {}", part2(contents));
    }
}

#[test]
fn basic_parsing() {
    assert_eq!(Machine::parse("[.##.] (3) (1,3) (2) {3,5,4}"), Ok(("", Machine {
        light_target: LightState { lights: vec![Light::Off, Light::On, Light::On, Light::Off] },
        buttons: vec![
            Button { affected_lights: vec![3] },
            Button { affected_lights: vec![1, 3] },
            Button { affected_lights: vec![2] },
        ],
        joltages: vec![3, 5, 4]
    })));
}

#[test]
fn test_p1() {
    assert_eq!(part1(&example()), 7);
}

#[test]
fn test_p2() {
    if PART2 {
        assert_eq!(part2(&example()), 42);
    }
}

#[derive(Hash)]
#[char_enum]
#[data_enum[bool]]
enum Light {
    On = ('#', true),
    Off = ('.', false),
}
impl Light {
    fn invert(&mut self) {
        *self = match self {
            Light::On => Light::Off,
            Light::Off => Light::On
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct LightState {
    lights: Vec<Light>
}
impl LightState {
    fn parse(input: &str) -> IResult<&str, Self> {
        delimited(complete::char('['), many1(map_res(complete::anychar, Light::try_decode)), complete::char(']'))
            .map(|lights| Self { lights })
            .parse(input)
    }

    fn new_blank(len: usize) -> Self {
        Self { lights: std::iter::repeat_n(Light::Off, len).collect() }
    }

    fn len(&self) -> usize {
        self.lights.len()
    }

    fn with(&self, button: &Button) -> Self {
        let mut ret = self.clone();
        ret.apply(button);
        ret
    }

    fn apply(&mut self, button: &Button) {
        for &idx in &button.affected_lights {
            self.lights[idx].invert();
        }
    }
}
impl DijkstraNode<&Machine> for LightState {
    fn get_connected(&self, context: &&Machine) -> Vec<(Self, usize)> where Self: Sized {
        context.buttons.iter().map(|b| (self.with(b), 1)).collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Button {
    affected_lights: Vec<usize>
}
impl Button {
    fn parse(input: &str) -> IResult<&str, Self> {
        delimited(complete::char('('), separated_list1(complete::char(','), complete::usize), complete::char(')'))
            .map(|affected_lights| Self { affected_lights })
            .parse(input)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Machine {
    light_target: LightState,
    buttons: Vec<Button>,
    joltages: Vec<usize>,
}
impl Machine {
    fn parse(input: &str) -> IResult<&str, Self> {
        let buttons = separated_list1(complete::char(' '), Button::parse);
        let joltages = delimited(complete::char('{'), separated_list1(complete::char(','), complete::usize), complete::char('}'));
        separated_pair(LightState::parse, complete::char(' '), separated_pair(buttons, complete::char(' '), joltages))
            .map(|(light_target, (buttons, joltages))| Self { light_target, buttons, joltages })
            .parse(input)
    }

    fn min_presses(&self) -> usize {
        let d = DijkstraData::dijkstra(LightState::new_blank(self.light_target.len()), self, |s| s == &self.light_target);
        d.best_distance[&self.light_target]
    }
}

struct Manual {
    machines: Vec<Machine>
}
impl Manual {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(complete::line_ending, Machine::parse)
            .map(|machines| Self { machines })
            .parse(input)
    }

    fn min_presses(&self) -> usize {
        self.machines.iter().map(Machine::min_presses).sum()
    }
}

fn part1(data: &str) -> usize {
    let manual = parse_complete(&mut Manual::parse, data);
    manual.min_presses()
}

fn part2(data: &str) -> usize {
    todo!();
}

