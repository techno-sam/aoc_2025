use std::fs;

use char_enum_impl::{char_enum, data_enum};

#[allow(dead_code)]
fn example() -> String {
    return "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
".trim().to_owned();
}

const PART2: bool = false;

fn main() {
    println!("AOC 2024 Day 06");

    let contents = fs::read_to_string("src/bin/day06/input.txt")
        .expect("Failed to read input");
    let contents = contents.trim();

    println!("Part 1: {}", part1(contents));

    if PART2 {
        println!("Part 2: {}", part2(contents));
    }
}

#[test]
fn test_p1() {
    assert_eq!(part1(&example()), 41);
}

#[test]
fn test_p2() {
    if PART2 {
        assert_eq!(part2(&example()), 42);
    }
}

#[char_enum]
enum Tile {
    BLOCKED = '#',
    OPEN = '.',
    OPEN_VISITED = 'X',
    GUARD = '^',
}

#[data_enum((isize, isize))]
#[derive(Clone, Copy)]
enum Direction {
    UP = (-1, 0),
    DOWN = (1, 0),
    RIGHT = (0, 1),
    LEFT = (0, -1),
}
impl Direction {
    fn turn_right(self) -> Direction {
        match self {
            Self::UP => Self::RIGHT,
            Self::RIGHT => Self::DOWN,
            Self::DOWN => Self::LEFT,
            Self::LEFT => Self::UP
        }
    }
}

fn part1(data: &str) -> usize {
    let mut grid: Vec<Vec<Tile>> = data.trim()
        .split("\n")
        .map(|l| l.chars().map(|c| Tile::decode(c)).collect())
        .collect();

    let row_count = grid.len();
    let col_count = grid[0].len();

    let mut guard_pos: (isize, isize) = (0, 0);
    let mut direction = Direction::UP;
    let mut count = 0;

    'Outer: for r in 0..row_count {
        for c in 0..col_count {
            if let Tile::GUARD = grid[r][c] {
                guard_pos = (r as isize, c as isize);
                grid[r][c] = Tile::OPEN_VISITED;
                count += 1;
                break 'Outer;
            }
        }
    }

    loop {
        let new_pos = (guard_pos.0 + direction.value().0, guard_pos.1 + direction.value().1);
        if new_pos.0 < 0 || new_pos.1 < 0 || new_pos.0 >= (row_count as isize) || new_pos.1 >= (col_count as isize) {
            break;
        }

        match grid[new_pos.0 as usize][new_pos.1 as usize] {
            Tile::OPEN => {
                grid[new_pos.0 as usize][new_pos.1 as usize] = Tile::OPEN_VISITED;
                count += 1;
                guard_pos = new_pos;
            }
            Tile::OPEN_VISITED => {
                guard_pos = new_pos;
            }
            Tile::BLOCKED => {
                direction = direction.turn_right();
            }
            Tile::GUARD => {}
        }
    }

    return count;
}

fn part2(data: &str) -> usize {
    todo!();
}

