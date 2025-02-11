use std::fs;

use char_enum_impl::char_enum;

#[allow(dead_code)]
fn example0() -> String {
    return "
..X...
.SAMX.
.A..A.
XMAS.S
.X....
".trim().to_owned();
}

#[allow(dead_code)]
fn example() -> String {
    return "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
".trim().to_owned();
}

const PART2: bool = true;

fn main() {
    println!("AOC 2024 Day 04");

    let contents = fs::read_to_string("src/bin/day04/input.txt")
        .expect("Failed to read input");
    let contents = contents.trim();

    println!("Part 1: {}", part1(contents));

    if PART2 {
        println!("Part 2: {}", part2(contents));
    }
}

#[test]
fn test_p1_0() {
    assert_eq!(part1(&example0()), 4);
}

#[test]
fn test_p1() {
    assert_eq!(part1(&example()), 18);
}

#[test]
fn test_p2() {
    if PART2 {
        assert_eq!(part2(&example()), 9);
    }
}

#[char_enum]
#[derive(PartialEq)]
enum Chars {
    X = 'X',
    M = 'M',
    A = 'A',
    S = 'S',
    FILLER = '.',
}

const XMAS: [Chars; 4] = [Chars::X, Chars::M, Chars::A, Chars::S];
const SAMX: [Chars; 4] = [Chars::S, Chars::A, Chars::M, Chars::X];

fn part1(data: &str) -> usize {
    let grid: Vec<Vec<Chars>> = data.split("\n")
        .map(|chars| chars.chars()
            .map(|c| Chars::decode(c))
            .collect()
        )
        .collect();

    let row_count = grid.len();
    let column_count = grid[0].len();

    const OFFSETS: [(isize, isize); 4] = [
        (1, 0),
        (0, 1),
        (1, 1),
        (-1, 1)
    ];

    let mut count = 0;

    for r in 0..row_count {
        for c in 0..column_count {
            let char = &grid[r][c];

            let target: [Chars; 4] = {
                if *char == Chars::X {
                    XMAS
                } else if *char == Chars::S {
                    SAMX
                } else {
                    continue;
                }
            };

            'Outer: for offset in OFFSETS {
                if r as isize + offset.0 * 3 >= row_count as isize || r as isize + offset.0 * 3 < 0 || c as isize + offset.1 * 3 >= column_count as isize{
                    continue;
                }

                // check all the targets for a given offset
                for i in 1..=3 {
                    let r2 = (r as isize + i*offset.0) as usize;
                    let c2 = (c as isize + i*offset.1) as usize;

                    let char2 = &grid[r2][c2];
                    if *char2 != target[i as usize] {
                        continue 'Outer;
                    }
                }
                // will only get here if all positions check out correctly
                count += 1;
            }
        }
    }

    return count;
}

fn part2(data: &str) -> usize {
    let grid: Vec<Vec<Chars>> = data.split("\n")
        .map(|chars| chars.chars()
            .map(|c| Chars::decode(c))
            .collect()
        )
        .collect();

    let row_count = grid.len();
    let column_count = grid[0].len();

    let mut count = 0;

    for r in 1..(row_count - 1) {
        for c in 1..(column_count - 1) {
            if grid[r][c] == Chars::A {
                let a1 = &grid[r-1][c-1];
                let a2 = &grid[r+1][c+1];

                let b1 = &grid[r-1][c+1];
                let b2 = &grid[r+1][c-1];

                if (*a1 == Chars::M && *a2 == Chars::S) || (*a1 == Chars::S && *a2 == Chars::M) {
                    if (*b1 == Chars::M && *b2 == Chars::S) || (*b1 == Chars::S && *b2 == Chars::M) {
                        count += 1;
                    }
                }
            }
        }
    }

    return count;
}

