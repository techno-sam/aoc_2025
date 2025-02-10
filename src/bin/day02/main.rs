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

    let contents = &fs::read_to_string("src/bin/day02/input.txt").expect("Failed to read input");

    println!("Part 1: {}", part1(contents));

    println!("Part 2: {}", part2(contents));
}

fn part1(data: &String) -> usize {
    let reports: Vec<Report> = data.split("\n")
        .filter_map(|s| Report::parse(s.trim()))
        .collect();

    let safe_count = reports.iter().filter(|r| r.safe()).count();
    return safe_count;
}

fn part2(data: &String) -> usize {
    let reports: Vec<Report> = data.split("\n")
        .filter_map(|s| Report::parse(s.trim()))
        .collect();

    let safe_count = reports.iter().filter(|r| r.safe2()).count();
    return safe_count;
}

#[test]
fn test_p1() {
    assert_eq!(part1(&example()), 2);
}

#[test]
fn test_p2() {
    assert_eq!(part2(&example()), 4);
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

    fn safe2(&self) -> bool {
        let incr1 = self.0[0] < self.0[1];
        let incr2 = self.0[0] < self.0[2];
        let incr3 = self.0[0] < self.0[3];

        let incr_count = incr1 as u8 + incr2 as u8 + incr3 as u8;
        let increasing = incr_count >= 2;

        let mut already_skipped = false;
        let mut last_skipped = false;

        for i in 0..(self.0.len()-1) {
            if i == self.0.len() - 2 { // no more space to skip
                already_skipped = true;
                if last_skipped {
                    break;
                }
            }

            let a = self.0[i];
            let b = {
                if last_skipped {
                    last_skipped = false;
                    self.0[i+2]
                } else {
                    self.0[i+1]
                }
            };

            if increasing != (a < b) {
                if already_skipped {
                    return false;
                } else {
                    already_skipped = true;
                    last_skipped = true;

                    let c = self.0[i+2];
                    if increasing != (a < c)  {
                        return false;
                    }
                }
            }

            let diff = a.abs_diff(b);
            if diff < 1 || diff > 3 {
                if already_skipped && !last_skipped {
                    return false;
                } else {
                    already_skipped = true;
                    last_skipped = true;

                    let c = self.0[i+2];
                    let diff = a.abs_diff(c);
                    if diff < 1 || diff > 3 {
                        return false;
                    }
                }
            }
        }

        return true;
    }
}

