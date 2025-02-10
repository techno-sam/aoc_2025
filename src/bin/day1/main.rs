use std::fs;

fn main() {
    println!("AOC 2024 Day 1");

    let EXAMPLE = false;

    let contents = {
        if EXAMPLE {
            example()
        } else {
            fs::read_to_string("src/bin/day1/input.txt")
                .expect("Failed to read input")
        }
    };

    let ids: Vec<(usize, usize)> = contents.split("\n")
        .map(|s| s.split_once("   "))
        .filter_map(|o| o)
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .collect();
    let mut ids: (Vec<usize>, Vec<usize>) = ids.into_iter().unzip();

    ids.0.sort();
    ids.1.sort();

    let total: usize = ids.0.into_iter().zip(ids.1.into_iter())
        .map(|(a, b)| a.abs_diff(b))
        .sum();

    println!("Part 1: {}", total);
}

fn example() -> String {
    return "3   4
4   3
2   5
1   3
3   9
3   3
".to_owned();
}
