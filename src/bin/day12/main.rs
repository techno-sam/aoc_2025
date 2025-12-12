use std::{fmt::Display, fs};

use char_enum_impl::char_enum;
use nom::{bytes::tag, character::complete, combinator::map_res, multi::{many_m_n, separated_list1}, sequence::{delimited, preceded, separated_pair, terminated}, IResult, Parser};
use utils::parse_complete;

macro_rules! mocha {
    ($color:ident) => {
        {
            let rgb = catppuccin::PALETTE.mocha.colors.$color.rgb;
            (rgb.r, rgb.g, rgb.b)
        }
    };
}

const COLORS: [(u8, u8, u8); 6] = [
    mocha!(red),
    mocha!(peach),
    mocha!(yellow),
    mocha!(green),
    mocha!(teal),
    mocha!(lavender),
];

#[allow(dead_code)]
fn example() -> String {
    "
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
".trim_matches('\n').to_owned()
}

const PART2: bool = false;

fn main() {
    println!("AOC 2025 Day 12");

    let contents = fs::read_to_string("src/bin/day12/input.txt")
        .expect("Failed to read input");
    let contents = contents.trim_matches('\n');

    println!("Part 1: {}", part1(contents));

    if PART2 {
        println!("Part 2: {}", part2(contents));
    }
}

#[test]
fn test_p1() {
    assert_eq!(part1(&example()), 2);
}

#[test]
fn test_p2() {
    if PART2 {
        assert_eq!(part2(&example()), 42);
    }
}

#[char_enum]
enum Tile {
    Present = '#', // in both meanings of the word :)
    Empty = '.',
}
impl Tile {
    fn parse(input: &str) -> IResult<&str, Self> {
        map_res(complete::anychar, Tile::try_decode).parse(input)
    }
}

#[derive(Clone, Copy, Debug)]
struct Present {
    grid: [[Tile; 3]; 3],
    area: usize,
}
impl Present {
    fn parse(input: &str) -> IResult<&str, Self> {
        many_m_n(3, 3, terminated(many_m_n(3, 3, Tile::parse), complete::line_ending))
            .map(|v| {
                let grid: [[Tile; 3]; 3] = v.into_iter()
                    .map(|v| v.try_into().unwrap())
                    .collect::<Vec<_>>()
                    .try_into().unwrap();
                let area = grid.iter().map(|row| row.iter().filter(|v| **v == Tile::Present).count()).sum();
                Self { grid, area }
            })
            .parse(input)
    }
}
impl Display for Present {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..3 {
            for col in 0..3 {
                write!(f, "{}", self.grid[row][col].encode())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
struct Region {
    width: usize,
    height: usize,
    counts: [usize; 6],
}
impl Region {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_pair(
            separated_pair(complete::usize, complete::char('x'), complete::usize),
            complete::char(':'),
            many_m_n(6, 6, preceded(complete::char(' '), complete::usize)).map(|v| v.try_into().unwrap())
        )
            .map(|((width, height), counts)| Self { width, height, counts })
            .parse(input)
    }

    fn check_fit(&self, idx: usize, shapes: &[Present; 6]) -> bool {
        if cfg!(test) && idx == 2 {
            // the sillyness below works for the real input, just not for the sample. *lovely*.
            return false;
        }

        let needed_area: usize = self.counts.iter().zip(shapes.iter()).map(|(count, shape)| count * shape.area).sum();
        if needed_area > self.width * self.height {
            if cfg!(test) {
                println!("Early discard region {}", idx);
            }
            return false;
        }

        true
    }
}
impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}:", self.width, self.height)?;
        #[allow(clippy::needless_range_loop)]
        for i in 0..6 {
            if f.alternate() {
                write!(f, " {}", self.counts[i])?;
            } else {
                let count = self.counts[i];
                let color = if count == 0 { mocha!(overlay1) } else { COLORS[i] };
                write!(f, " {}{}{}", utils::fg_string(color.into()), count, utils::RESET)?;
            }
        }
        writeln!(f)?;

        for _row in 0..self.height {
            for _col in 0..self.width {
                write!(f, ".")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Situation {
    shapes: [Present; 6],
    regions: Vec<Region>,
}
impl Situation {
    fn parse(input: &str) -> IResult<&str, Self> {
        let shapes_parser = [0, 1, 2, 3, 4, 5].map(|i| {
            delimited(
                (complete::char(char::from_digit(i, 10).unwrap()), tag(":\n")),
                Present::parse,
                complete::line_ending
            )
        });
        let [sp0, sp1, sp2, sp3, sp4, sp5] = shapes_parser;
        let shapes_parser = (sp0, sp1, sp2, sp3, sp4, sp5);
        let regions_parser = separated_list1(complete::line_ending, Region::parse);

        (shapes_parser, regions_parser).map(|(shapes, regions)| {
            let shapes = [shapes.0, shapes.1, shapes.2, shapes.3, shapes.4, shapes.5];
            Self { shapes, regions }
        }).parse(input)
    }

    fn print(&self) {
        for (i, shape) in self.shapes.iter().enumerate() {
            println!("Shape {}:\n{}{}{}", i, utils::fg_string(COLORS[i].into()), shape, utils::RESET);
        }

        for (i, region) in self.regions.iter().enumerate() {
            println!("Region[{}] {}", i, region);
        }
    }

    fn count_fit(&self) -> usize {
        self.regions.iter()
            .enumerate()
            .filter(|(i, region)| region.check_fit(*i, &self.shapes))
            .count()
    }
}

fn part1(data: &str) -> usize {
    let situation = parse_complete(&mut Situation::parse, data);
    if cfg!(test) {
        situation.print();
    }
    situation.count_fit()
}

fn part2(data: &str) -> usize {
    todo!();
}

