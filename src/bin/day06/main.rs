use std::fs;

use char_enum_impl::{char_enum, data_enum};
use utils::{Color, Style, StyleUtil, StyledChar};

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

const PART2: bool = true;

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
        assert_eq!(part2(&example()), 6);
    }
}

#[char_enum]
#[derive(Clone, Copy)]
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

    fn idx(self) -> usize {
        match self {
            Self::UP => 0,
            Self::DOWN => 1,
            Self::RIGHT => 2,
            Self::LEFT => 3
        }
    }
}

fn part1(data: &str) -> usize {
    let mut grid: Vec<Vec<Tile>> = data.trim()
        .split("\n")
        .map(|l| l.chars().map(|c| Tile::decode(c)).collect())
        .collect();

    return mover(&mut grid).unwrap();
}

fn mover(grid: &mut Vec<Vec<Tile>>) -> Option<usize> {
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

    let start_pos = guard_pos;

    let mut visited_directions: Vec<Vec<[bool; 4]>> = (0..row_count).map(|_| (0..col_count).map(|_| [false; 4]).collect()).collect();
    visited_directions[start_pos.0 as usize][start_pos.1 as usize][Direction::UP.idx()] = true;

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

        if new_pos == guard_pos {
            let visited_directions = &mut visited_directions[new_pos.0 as usize][new_pos.1 as usize];
            if visited_directions[direction.idx()] {
                return None; // caught in a loop
            } else {
                visited_directions[direction.idx()] = true;
            }
        }

        /*let mut dbg_grid = grid_to_styled_grid(&grid);
        let green_fg = Style::fg(Some(Color::rgb(0, 255, 0)));
        dbg_grid.merge_style(guard_pos.0 as usize, guard_pos.1 as usize, &green_fg);
        utils::print_grid(&dbg_grid);
        println!("");*/
    }

    return Some(count);
}

fn print_grid(grid: &Vec<Vec<Tile>>) {
    let s: String = grid.iter()
        .map(|r| r.iter().map(|t| t.encode().to_string()).collect::<Vec<_>>().join(""))
        .collect::<Vec<_>>()
        .join("\n");
    println!("{}", s);
}

fn grid_to_styled_grid(grid: &Vec<Vec<Tile>>) -> Vec<Vec<StyledChar>> {
    grid.iter()
        .map(|r| r.iter()
            .map(|t| StyledChar::of(t.encode()))
            .collect())
        .collect()
}

fn part2(data: &str) -> usize {
    let grid: Vec<Vec<Tile>> = data.trim()
        .split("\n")
        .map(|l| l.chars().map(|c| Tile::decode(c)).collect())
        .collect();

    let row_count = grid.len();
    let col_count = grid[0].len();

    println!("Counts: {} {}", row_count, col_count);

    let mut exploratory_grid = grid.clone();
    mover(&mut exploratory_grid);
    let traversed: Vec<(usize, usize)> = {
        let mut traversed = vec![];
        for r in 0..row_count {
            for c in 0..col_count {
                if let Tile::OPEN_VISITED = exploratory_grid[r][c] {
                    traversed.push((r, c));
                }
            }
        }
        traversed
    };

    let mut dbg_grid = grid_to_styled_grid(&grid);
    let red_bg = Style::bg(Some(Color {
        r: 255,
        g: 0,
        b: 0
    }));
    for &(r, c) in &traversed {
        dbg_grid.merge_style(r, c, &red_bg);
    }
    utils::print_grid(&dbg_grid);

    let mut count = 0;

    for (r, c) in traversed {
        if let Tile::OPEN = grid[r][c] {
            println!("{} {}", r, c);
            let mut new_grid = grid.clone();
            new_grid[r][c] = Tile::BLOCKED;
            //print_grid(&new_grid);

            if let None = mover(&mut new_grid) {
                count += 1;
            }
        }
    }

    return count;
}

