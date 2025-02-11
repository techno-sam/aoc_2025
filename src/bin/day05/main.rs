use std::fs;

#[allow(dead_code)]
fn example() -> String {
    return "
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
".trim().to_owned();
}

const PART2: bool = true;

fn main() {
    println!("AOC 2024 Day 05");

    let contents = fs::read_to_string("src/bin/day05/input.txt")
        .expect("Failed to read input");
    let contents = contents.trim();

    println!("Part 1: {}", part1(contents));

    if PART2 {
        println!("Part 2: {}", part2(contents));
    }
}

#[test]
fn test_p1() {
    assert_eq!(part1(&example()), 143);
}

#[test]
fn test_p2() {
    if PART2 {
        assert_eq!(part2(&example()), 123);
    }
}

struct Ordering {
    a: usize,
    b: usize
}

impl Ordering {
    fn parse(line: &str) -> Ordering {
        let line = line.trim();
        let pieces = line.split_once('|').unwrap();
        Ordering {
            a: pieces.0.parse().unwrap(),
            b: pieces.1.parse().unwrap(),
        }
    }
}

#[derive(Clone)]
struct Update(Vec<usize>);

impl Update {
    fn is_ordered(&self, order: &Ordering) -> bool {
        let mut found_a: bool = false;
        let mut found_b_anywhere: bool = false;

        for v in &self.0 {
            if *v == order.a {
                found_a = true;
            } else if *v == order.b {
                if found_a {
                    return true;
                } else {
                    found_b_anywhere = true;
                }
            }
        }

        return !(found_a && found_b_anywhere); // if we didn't even find both numbers, it's ok that they're not in the right order
    }
    
    fn middle_num(&self) -> usize {
        let idx = self.0.len() / 2;
        return self.0[idx];
    }

    fn fix_order(&mut self, order: &Ordering) {
        if self.is_ordered(order) {
            return;
        }

        let mut idxa = None;
        let mut idxb = None;

        for i in 0..self.0.len() {
            if let None = idxa {
                if self.0[i] == order.a {
                    idxa = Some(i);
                }
            }

            if let None = idxb {
                if self.0[i] == order.b {
                    idxb = Some(i);
                }
            }
        }

        let mindex = idxa.unwrap().min(idxb.unwrap());
        let maxdex = idxa.unwrap().max(idxb.unwrap());

        self.0[mindex] = order.a;
        self.0[maxdex] = order.b;
    }
}

fn part1(data: &str) -> usize {
    let (orders, updates) = data.split_once("\n\n").unwrap();

    let orders: Vec<Ordering> = orders.trim()
        .split("\n")
        .map(|s| Ordering::parse(s))
        .collect();

    let updates: Vec<Update> = updates.trim()
        .split("\n")
        .map(|l| l.split(",")
            .map(|s| s.parse().unwrap())
            .collect()
        )
        .map(|v| Update(v))
        .collect();

    return updates.iter()
        .filter(|u| orders.iter().all(|o| u.is_ordered(o)))
        .map(|u| u.middle_num())
        .sum();
}

fn part2(data: &str) -> usize {
    let (orders, updates) = data.split_once("\n\n").unwrap();

    let orders: Vec<Ordering> = orders.trim()
        .split("\n")
        .map(|s| Ordering::parse(s))
        .collect();

    let mut updates: Vec<Update> = updates.trim()
        .split("\n")
        .map(|l| l.split(",")
            .map(|s| s.parse().unwrap())
            .collect()
        )
        .map(|v| Update(v))
        .collect();

    return updates.iter_mut()
        .filter(|u| orders.iter().any(|o| !u.is_ordered(o)))
        .map(|u| {
            loop {
                orders.iter().for_each(|o| u.fix_order(o));
                if orders.iter().all(|o| u.is_ordered(o)) {
                    break;
                }
            }
            return u;
        })
        .map(|u| u.middle_num())
        .sum();
}

