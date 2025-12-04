use std::fs;

use char_enum_impl::char_enum;

#[allow(dead_code)]
fn example() -> String {
    "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
".trim().to_owned()
}

const PART2: bool = false;

fn main() {
    println!("AOC 2025 Day 04");

    let contents = fs::read_to_string("src/bin/day04/input.txt")
        .expect("Failed to read input");
    let contents = contents.trim();

    println!("Part 1: {}", part1(contents));

    if PART2 {
        println!("Part 2: {}", part2(contents));
    }
}

#[test]
fn test_p1() {
    assert_eq!(part1(&example()), 13);
}

#[test]
fn test_p2() {
    if PART2 {
        assert_eq!(part2(&example()), 42);
    }
}

#[derive(Clone, Copy, Debug)]
#[char_enum]
enum Tile {
    Paper = '@',
    Floor = '.'
}

#[derive(Clone, Debug)]
struct Field {
    tiles: Vec<Vec<Tile>>,
    adjacencies: Vec<Vec<usize>>,
    height: usize,
    width: usize
}
impl Field {
    fn parse(input: &str) -> Self {
        let tiles: Vec<Vec<_>> = input.lines().map(|l| l.chars().map(Tile::decode).collect()).collect();
        let height = tiles.len();
        let width = tiles[0].len();
        assert_eq!(tiles.iter().map(|v| v.len()).sum::<usize>(), width * height);

        let adjacencies = tiles.iter().map(|v| v.iter().map(|_| 0).collect()).collect();

        Self { tiles, adjacencies, height, width }
    }

    fn calculate_adjacencies(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Tile::Floor = self.tiles[y][x] {
                    continue;
                }

                let y = y as isize;
                let x = x as isize;

                for yo in -1..=1 {
                    for xo in -1..=1 {
                        if yo == 0 && xo == 0 {
                            continue;
                        }

                        let y = y + yo;
                        let x = x + xo;

                        if y < 0 || x < 0 {
                            continue;
                        }

                        let y = y as usize;
                        let x = x as usize;

                        if y >= self.height || x >= self.width {
                            continue;
                        }

                        self.adjacencies[y][x] += 1;
                    }
                }
            }
        }

        for y in 0..self.height {
            for x in 0..self.height {
                if let Tile::Floor = self.tiles[y][x] {
                    self.adjacencies[y][x] = usize::MAX;
                }
            }
        }
    }

    fn count_accessible(&self) -> usize {
        self.adjacencies.iter().map(|line| line.iter().filter(|v| **v < 4).count()).sum()
    }
}

fn part1(data: &str) -> usize {
    let field = {
        let mut field = Field::parse(data);
        field.calculate_adjacencies();
        field
    };

    field.count_accessible()
}

fn part2(data: &str) -> usize {
    todo!();
}

