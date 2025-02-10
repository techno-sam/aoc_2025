use std::fs;

fn example() -> String {
    return "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
".trim().to_owned();
}

fn main() {
    println!("AOC 2024 Day 02");

    let contents = fs::read_to_string("src/bin/day02/input.txt").expect("Failed to read input");

    println!("Part 1: {}", part1(contents));
}

fn part1(data: String) -> usize {
    let reports: Vec<Report> = data.split("\n")
        .filter_map(|s| Report::parse(s.trim()))
        .collect();

    let safe_count = reports.iter().filter(|r| r.safe()).count();
    return safe_count;
}

#[test]
fn test_p1() {
    assert_eq!(part1(example()), 2);
}

struct Report(Vec<usize>);

impl Report {
    fn parse(s: &str) -> Option<Report> {
        let report = Report(
            s
            .split(" ")
            .filter_map(|s| s.parse::<usize>().ok())
            .collect()
        );

        if report.0.is_empty() {
            return None;
        } else {
            return Some(report);
        }
    }

    fn safe(&self) -> bool {
        let increasing = self.0[0] < self.0[1];

        for i in 0..(self.0.len()-1) {
            let a = self.0[i];
            let b = self.0[i+1];

            if increasing != (a < b) {
                return false;
            }

            let diff = a.abs_diff(b);
            if diff < 1 || diff > 3 {
                return false;
            }
        }

        return true;
    }
}

