use std::{collections::HashMap, fs};

use utils::{make_grid, parse_grid, print_grid, Color, GridMap, Point, Style, StyledChar};

#[allow(dead_code)]
fn example() -> String {
    return "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
".trim().to_owned();
}

#[allow(dead_code)]
fn example2() -> String {
    return "
........
.A.A....
........
...A....
........
........
........
........
".trim().to_owned();
}

const PART2: bool = false;

fn main() {
    println!("AOC 2024 Day 08");

    let contents = fs::read_to_string("src/bin/day08/input.txt")
        .expect("Failed to read input");
    let contents = contents.trim();

    println!("Part 1: {}", part1(contents));

    if PART2 {
        println!("Part 2: {}", part2(contents));
    }
}

#[test]
fn test_p1_a() {
    assert_eq!(part1(&example()), 14);
}

#[test]
fn test_p1_b() {
    assert_eq!(part1(&example2()), 3);
}

#[test]
fn test_p2() {
    if PART2 {
        assert_eq!(part2(&example()), 42);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tile {
    Empty,
    Antenna(char)
}
impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            c => Self::Antenna(c)
        }
    }
}
impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Empty => '.',
            Tile::Antenna(c) => c
        }
    }
}

#[derive(Clone)]
struct Map {
    rows: usize,
    cols: usize,
    grid: Vec<Vec<Tile>>,
    antennas: HashMap<char, Vec<Point<usize>>>
}
impl Map {
    fn of(grid: Vec<Vec<Tile>>) -> Map {
        let mut antennas = HashMap::new();
        let rows = grid.len();
        let cols = grid[0].len();

        for r in 0..rows {
            for c in 0..cols {
                if let Tile::Antenna(id) = grid[r][c] {
                    let coords = match antennas.get_mut(&id) {
                        Some(v) => v,
                        None => {
                            antennas.insert(id, vec![]);
                            antennas.get_mut(&id).unwrap()
                        }
                    };
                    coords.push((r, c).into());
                }
            }
        }

        Map { rows, cols, grid, antennas }
    }

    fn node_count(&self) -> usize {
        let mut antinodes = make_grid(self.rows, self.cols, false);
        let mut count = 0;

        for coords in self.antennas.values() {
            for i in 0..coords.len() {
                for j in i+1..coords.len() {
                    let a = coords[i];
                    let b = coords[j];

                    let a = a.map(|v| v as isize);
                    let b = b.map(|v| v as isize);

                    let node_a = b*2 - a; // b + (b - a)
                    let node_b = a*2 - b; // a - (b - a)

                    assert_ne!(node_a, a);
                    assert_ne!(node_a, b);
                    assert_ne!(node_b, a);
                    assert_ne!(node_b, b);

                    if node_a.a >= 0 && node_a.a < (self.rows as isize) && node_a.b >= 0 && node_a.b < (self.cols as isize) {
                        let node_a = node_a.map(|v| v as usize);
                        if !antinodes[node_a] {
                            antinodes[node_a] = true;
                            count += 1;
                        }
                    }

                    if node_b.a >= 0 && node_b.a < (self.rows as isize) && node_b.b >= 0 && node_b.b < (self.cols as isize) {
                        let node_b = node_b.map(|v| v as usize);
                        if !antinodes[node_b] {
                            antinodes[node_b] = true;
                            count += 1;
                        }
                    }
                }
            }
        }

        // print antinodes for debug purposes
        let mut dbg_grid: Vec<Vec<StyledChar>> = self.grid.grid_map(|v| StyledChar::of((*v).into()));
        let antinode_style = Style::fg(Some(Color::rgb(255, 0, 120)));
        for r in 0..self.rows {
            for c in 0..self.cols {
                if antinodes[r][c] {
                    dbg_grid[r][c].style.merge(&antinode_style);
                }
            }
        }

        let black_text = Style::fg(Some(Color::BLACK));
        for (&id, coords) in self.antennas.iter() {
            let style = {
                let mut style = Style::bg(Some(Color::random_from_seed(id as usize)));
                style.merge(&black_text);
                style
            };
            for c in coords {
                dbg_grid[*c].style.merge(&style);
            }
        }

        print_grid(&dbg_grid);

        count
    }
}

fn part1(data: &str) -> usize {
    println!();
    let grid: Vec<Vec<Tile>> = parse_grid(data);
    let map = Map::of(grid);
    map.node_count()
}

fn part2(data: &str) -> usize {
    todo!();
}

