use std::{fs::{self, File}, io::Write, path::Path};

use char_enum_impl::{char_enum, data_enum};
use nom::{character::complete, multi::separated_list1, sequence::separated_pair, IResult, Parser};
use utils::{make_grid, parse_complete};

#[allow(dead_code)]
fn example() -> String {
    "
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
".trim_matches('\n').to_owned()
}

const PART2: bool = true;

fn main() {
    println!("AOC 2025 Day 09");

    let contents = fs::read_to_string("src/bin/day09/input.txt")
        .expect("Failed to read input");
    let contents = contents.trim_matches('\n');

    println!("Part 1: {}", part1(contents));

    if PART2 {
        println!("Part 2: {}", part2(contents));
    }
}

#[test]
fn test_p1() {
    assert_eq!(part1(&example()), 50);
}

#[test]
fn test_p2() {
    if PART2 {
        assert_eq!(part2(&example()), 24);
    }
}

type Point = utils::Point<usize>;
type Compactor = utils::Compactor<usize>;

fn area(a: Point, b: Point) -> usize {
    (1 + a.x.max(b.x) - a.x.min(b.x)) * (1 + a.y.max(b.y) - a.y.min(b.y))
}

#[char_enum]
#[data_enum[(u8, u8, u8)]]
enum Tile {
    Red = ('#', (255, 0, 0)),
    Green = ('X', (0, 255, 0)),
    Fill = ('x', (0, 190, 0)),
    Floor = ('.', (200, 200, 200)),
}
impl Tile {
    fn print(self) {
        let (r, g, b) = self.value();
        print!("{}", utils::colorize(&format!("{}", self.encode()), r, g, b));
    }

    fn save_grid(grid: &Vec<Vec<Self>>, fname: &Path) -> std::io::Result<()> {
        let mut file = File::create(fname)?;
        file.write_all("P3\n".as_bytes())?;
        file.write_all(format!("{} {}\n", grid[0].len(), grid.len()).as_bytes())?;
        file.write_all("255\n".as_bytes())?;

        for row in grid {
            for tile in row {
                let (r, g, b) = tile.value();
                file.write_all(format!("{} {} {}\n", r, g, b).as_bytes())?;
            }
        }

        Ok(())
    }
}


struct Map {
    red_tiles: Vec<Point>,
    compactor: Compactor,
    compacted_tiles: Vec<Point>,
}
impl Map {
    fn parse_point(input: &str) -> IResult<&str, Point> {
        separated_pair(complete::usize, complete::char(','), complete::usize).map(|(a, b)| Point { x: a, y: b }).parse(input)
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(complete::line_ending, Self::parse_point).map(|red_tiles| {
            let x0 = red_tiles.iter().map(|p| p.x).min().unwrap();
            let y0 = red_tiles.iter().map(|p| p.y).min().unwrap();
            let p0 = Point { x: x0, y: y0 };
            let red_tiles: Vec<Point> = red_tiles.into_iter().map(|p| p - p0).collect();

            let compactor = {
                let mut compactor = Compactor::new();
                red_tiles.iter().for_each(|t| compactor.add_key_point(*t));
                compactor
            };
            let compacted_tiles = red_tiles.iter().map(|t| compactor.compact(*t)).collect();

            Self { red_tiles, compactor, compacted_tiles }
        }).parse(input)
    }

    fn max_area(&self) -> usize {
        let mut max_area = 0;

        for i in 0..self.red_tiles.len() {
            for j in i+1..self.red_tiles.len() {
                max_area = max_area.max(area(self.red_tiles[i], self.red_tiles[j]));
            }
        }

        max_area
    }

    #[allow(clippy::needless_range_loop)]
    fn build_grid(&self, compact: bool) -> Vec<Vec<Tile>> {
        let tiles = if compact { &self.compacted_tiles } else { &self.red_tiles };
        let width = tiles.iter().map(|p| p.x).max().unwrap() + 1;
        let height = tiles.iter().map(|p| p.y).max().unwrap() + 1;

        let mut grid = make_grid(height, width, Tile::Floor);

        for i in 0..tiles.len() {
            let j = (i + 1) % tiles.len();

            let Point { x: xa, y: ya } = tiles[i];
            let Point { x: xb, y: yb } = tiles[j];

            if xa == xb {
                for y in ya.min(yb)..=ya.max(yb) {
                    grid[y][xa] = Tile::Green;
                }
            } else if ya == yb {
                for x in xa.min(xb)..=xa.max(xb) {
                    grid[ya][x] = Tile::Green;
                }
            } else {
                panic!("Invalid pair of tiles: must be orthoganlly colinear");
            }
        }

        tiles.iter().for_each(|t| grid[*t] = Tile::Red);

        if compact || cfg!(test) {
            for row in &mut grid {
                let mut left_was_in_floor = true;
                let mut left_ok = false;
                for x in 0..row.len() {
                    match row[x] {
                        Tile::Red | Tile::Green => {
                            if left_was_in_floor {
                                left_ok = !left_ok;
                                left_was_in_floor = false;
                            }
                        }
                        Tile::Fill => {
                            left_was_in_floor = true;
                        }
                        Tile::Floor => {
                            left_was_in_floor = true;

                            if left_ok {
                                let mut right_was_in_floor = true;
                                let mut right_ok = false;
                                for &tile1 in row.iter().skip(x) {
                                    match tile1 {
                                        Tile::Red | Tile::Green => {
                                            if right_was_in_floor {
                                                right_was_in_floor = false;
                                                right_ok = !right_ok;
                                            }
                                        }

                                        Tile::Fill | Tile::Floor => {
                                            right_was_in_floor = true;
                                        }
                                    }
                                }

                                if right_ok {
                                    row[x] = Tile::Fill;
                                }
                            }
                        }
                    }
                }
            }
        }

        grid
    }

    fn print_part2(&self, grid: &Vec<Vec<Tile>>, ppm_path: &Path) {
        Tile::save_grid(grid, ppm_path).unwrap();

        if cfg!(test) {
            for row in grid {
                row.iter().for_each(|t| t.print());
                println!();
            }
        }
    }

    #[allow(clippy::needless_range_loop, clippy::ptr_arg)]
    fn validate(a: Point, b: Point, grid: &Vec<Vec<Tile>>) -> bool {
        let x0 = a.x.min(b.x);
        let x1 = a.x.max(b.x);
        let y0 = a.y.min(b.y);
        let y1 = a.y.max(b.y);

        if grid[y0][x0] == Tile::Floor || grid[y0][x1] == Tile::Floor || grid[y1][x0] == Tile::Floor || grid[y1][x1] == Tile::Floor {
            return false;
        }

        for y in y0..=y1 {
            let row = &grid[y];
            if row[x0] == Tile::Floor || row[x1] == Tile::Floor {
                return false;
            }
        }

        for x in x0..=x1 {
            if grid[y0][x] == Tile::Floor || grid[y1][x] == Tile::Floor {
                return false;
            }
        }

        /*for y in y0..=y1 {
            let row = &grid[y];
            for x in x0..=x1 {
                if row[x] == Tile::Floor {
                    return false;
                }
            }
        }*/

        true
    }

    fn max_area2(&self) -> usize {
        let grid = self.build_grid(true);
        /*[bench exclude]*/ {
            println!("Grid done");
            self.print_part2(&grid, Path::new("compact.ppm"));
            if cfg!(test) {
                println!("Full");
                self.print_part2(&self.build_grid(false), Path::new("full.ppm"));
                println!("Compactor: {:?}", self.compactor);
            }
        }

        let mut max_area = 0;

        for i in 0..self.red_tiles.len() {
            for j in i+1..self.red_tiles.len() {
                let a = self.red_tiles[i];
                let b = self.red_tiles[j];

                let ac = self.compacted_tiles[i];
                let bc = self.compacted_tiles[j];
                if !Self::validate(ac, bc, &grid) {
                    continue;
                }

                max_area = max_area.max(area(a, b));
            }
        }

        max_area
    }
}

fn part1(data: &str) -> usize {
    let map = parse_complete(&mut Map::parse, data);

    map.max_area()
}

fn part2(data: &str) -> usize {
    let map = parse_complete(&mut Map::parse, data);
    map.max_area2()
}

