use std::fs;

use bitvec::vec::BitVec;
use char_enum_impl::char_enum;
use utils::GridMap;

#[allow(dead_code)]
fn example() -> String {
    "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
".trim_matches('\n').to_owned()
}

const PART2: bool = true;

fn main() {
    println!("AOC 2025 Day 07");

    let contents = fs::read_to_string("src/bin/day07/input.txt")
        .expect("Failed to read input");
    let contents = contents.trim_matches('\n');

    println!("Part 1: {}", part1(contents));

    if PART2 {
        println!("Part 2: {}", part2(contents));
    }
}

#[test]
fn test_p1() {
    assert_eq!(part1(&example()), 21);
}

#[test]
fn test_p2() {
    if PART2 {
        assert_eq!(part2(&example()), 40);
    }
}

#[char_enum]
enum Tile {
    Start = 'S',
    Empty = '.',
    Splitter = '^',
}

#[derive(Debug)]
struct Manifold {
    grid: Vec<Vec<Tile>>,
    splitters: Vec<BitVec>
}
impl Manifold {
    fn parse(input: &str) -> Self {
        let grid = utils::parse_grid::<Tile>(input);
        let splitters = grid.row_map(|&tile| tile == Tile::Splitter);
        assert!(grid[0].contains(&Tile::Start), "first row must contain start");

        Self { grid, splitters }
    }

    fn count_splits(&self) -> usize {
        let mut count = 0;
        let mut beams = self.grid[0].iter().map(|&tile| tile == Tile::Start).collect::<BitVec>();

        for splitters in self.splitters.iter().skip(1) {
            let split = beams.clone() & splitters;
            beams ^= &split; // remove all split beams from the beam
            count += split.count_ones();

            beams |= &split[1..];
            beams[1..] |= split;
        }

        count
    }

    fn count_timelines(&self) -> usize {
        let mut beams = self.grid[0].iter().map(|&tile| tile == Tile::Start).collect::<BitVec>();
        let mut count = beams.iter().map(|v| if *v { 1 } else { 0 }).collect::<Vec<usize>>();
        let cols = count.len();

        for splitters in self.splitters.iter().skip(1) {
            let split = beams.clone() & splitters;
            beams ^= &split; // remove all split beams from the beam

            beams |= &split[1..];
            beams[1..] |= &split;

            let mut new_count = count.iter().zip(split.iter()).map(|(&count, split)| if *split { 0 } else { count }).collect::<Vec<usize>>();
            split.iter_ones().for_each(|idx| {
                if idx > 0 {
                    new_count[idx - 1] += count[idx];
                }

                if idx < cols - 1 {
                    new_count[idx + 1] += count[idx];
                }
            });
            count = new_count;
        }

        count.into_iter().sum()
    }
}

fn part1(data: &str) -> usize {
    let manifold = Manifold::parse(data);
    manifold.count_splits()
}

fn part2(data: &str) -> usize {
    let manifold = Manifold::parse(data);
    manifold.count_timelines()
}

