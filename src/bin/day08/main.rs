use std::{collections::HashSet, fmt::Display, fs};

use nom::{character::complete, multi::separated_list1, sequence::separated_pair, IResult, Parser};
use utils::parse_complete;

#[allow(dead_code)]
fn example() -> String {
    "
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
".trim_matches('\n').to_owned()
}

const PART2: bool = true;

fn main() {
    println!("AOC 2025 Day 08");

    let contents = fs::read_to_string("src/bin/day08/input.txt")
        .expect("Failed to read input");
    let contents = contents.trim_matches('\n');

    println!("Part 1: {}", part1(contents));

    if PART2 {
        println!("Part 2: {}", part2(contents));
    }
}

#[test]
fn test_p1() {
    assert_eq!(part1_(&example(), 10), 40);
}

#[test]
fn test_p2() {
    if PART2 {
        assert_eq!(part2(&example()), 25272);
    }
}

#[derive(Clone, Copy, Debug)]
struct JunctionBox {
    x: usize,
    y: usize,
    z: usize
}
impl JunctionBox {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_pair(complete::usize, complete::char(','), separated_pair(complete::usize, complete::char(','), complete::usize))
            .map(|(x, (y, z))| Self { x, y, z})
            .parse(input)
    }

    fn dist_sq(&self, other: &Self) -> usize {
        let dx = self.x as isize - other.x as isize;
        let dy = self.y as isize - other.y as isize;
        let dz = self.z as isize - other.z as isize;

        (dx*dx + dy*dy + dz*dz) as usize
    }
}
impl Display for JunctionBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

#[derive(Clone, Copy, Debug)]
struct ConnectionCandidate {
    dist_sq: usize,
    /// the index of the first box
    id_a: usize,
    /// the index of the second box
    id_b: usize,
}

#[derive(Clone, Debug)]
struct Net {
    members: HashSet<usize>,
}
impl Net {
    fn new() -> Self {
        Self { members: HashSet::new() }
    }

    fn len(&self) -> usize {
        self.members.len()
    }

    fn add(&mut self, id: usize) {
        self.members.insert(id);
    }

    fn absorb(&mut self, other: &mut Net) {
        self.members.extend(other.members.iter());
        other.members.clear();
    }
}

#[derive(Clone, Debug)]
struct Cave {
    boxes: Vec<(JunctionBox, Option<usize>)>,
    candidates: Vec<ConnectionCandidate>,
    nets: Vec<Net>,

}
impl Cave {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(complete::line_ending, JunctionBox::parse).map(|boxes| Self {
            boxes: boxes.into_iter().map(|jb| (jb, None)).collect(),
            candidates: vec![],
            nets: vec![]
        }).parse(input)
    }

    fn populate_distances(&mut self) {
        for i in 0..self.boxes.len() {
            for j in (i+1)..self.boxes.len() {
                let dist_sq = self.boxes[i].0.dist_sq(&self.boxes[j].0);
                self.candidates.push(ConnectionCandidate { dist_sq, id_a: i, id_b: j });
            }
        }
        self.candidates.sort_by_key(|cc| cc.dist_sq);
    }

    fn print_net(&self, id: usize) {
        println!("Net[{}]", id);

        let net = &self.nets[id];
        let mut members: Vec<usize> = net.members.iter().copied().collect();
        members.sort();
        for box_id in members {
            println!("  {} [{:?}]", self.boxes[box_id].0, self.boxes[box_id].1);
        }
    }

    fn run_step(&mut self, step: usize) -> (usize, usize) {
        let candidate = &self.candidates[step];
        let a = candidate.id_a;
        let b = candidate.id_b;

        let product = self.boxes[a].0.x * self.boxes[b].0.x;

        let net_a = self.boxes[a].1;
        let net_b = self.boxes[b].1;

        if cfg!(test) {
            println!("Connecting {}[{:?}] and {}[{:?}]", self.boxes[a].0, net_a, self.boxes[b].0, net_b);
        }

        let combined_id = match (net_a, net_b) {
            (None, None) => {
                self.nets.push(Net::new());
                let net_id = self.nets.len() - 1;
                let net = &mut self.nets[net_id];
                net.add(a);
                net.add(b);
                self.boxes[a].1 = Some(net_id);
                self.boxes[b].1 = Some(net_id);
                net_id
            }
            (Some(net_a), None) => {
                self.boxes[b].1 = Some(net_a);
                self.nets[net_a].add(b);
                net_a
            }
            (None, Some(net_b)) => {
                self.boxes[a].1 = Some(net_b);
                self.nets[net_b].add(a);
                net_b
            }
            (Some(net_a), Some(net_b)) => {
                if net_a == net_b {
                    return (product, self.nets[net_a].len());
                }

                let (net_a, net_b) = {
                    if self.nets[net_a].len() >= self.nets[net_b].len() {
                        (net_a, net_b)
                    } else {
                        (net_b, net_a)
                    }
                };

                for &id in &self.nets[net_b].members {
                    self.boxes[id].1 = Some(net_a);
                }

                assert_ne!(net_a, net_b);
                if net_a < net_b {
                    let (part_a, part_b) = self.nets.split_at_mut(net_b);
                    part_a[net_a].absorb(&mut part_b[0]);
                } else {
                    let (part_b, part_a) = self.nets.split_at_mut(net_a);
                    part_a[0].absorb(&mut part_b[net_b]);
                }

                net_a
            }
        };

        if cfg!(test) {
            self.print_net(combined_id);
            println!();
        }

        (product, self.nets[combined_id].len())
    }

    fn connect_n(&mut self, n: usize) -> usize {
        for i in 0..n {
            self.run_step(i);
        }

        if cfg!(test) {
            for net in &self.nets {
                println!("Net size {}: {:?}", net.len(), net.members)
            }
        }

        let mut sizes: Vec<usize> = self.nets.iter().map(|n| n.len()).collect();
        sizes.sort();
        sizes.into_iter().rev().take(3).product()
    }

    fn connect_all(&mut self) -> usize {
        let mut last_product = 0;

        for i in 0..self.candidates.len() {
            let net_size;
            (last_product, net_size) = self.run_step(i);

            if net_size == self.boxes.len() {
                break;
            }
        }

        last_product
    }
}

fn part1_(data: &str, n: usize) -> usize {
    let mut cave = parse_complete(&mut Cave::parse, data);
    cave.populate_distances();
    cave.connect_n(n)
}

fn part1(data: &str) -> usize {
    part1_(data, 1000)
}

fn part2(data: &str) -> usize {
    let mut cave = parse_complete(&mut Cave::parse, data);
    cave.populate_distances();
    cave.connect_all()
}

