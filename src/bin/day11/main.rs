use std::{collections::HashMap, fmt::Display, fs, ops::{Add, AddAssign, Index, IndexMut, Mul}};

use nom::{character::complete, multi::separated_list1, sequence::separated_pair, IResult, Parser};
use utils::{parse_complete, Point};

// hint van Steef: Topological Sort

#[allow(dead_code)]
fn example() -> String {
    "
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
".trim_matches('\n').to_owned()
}

#[allow(dead_code)]
fn example2() -> String {
    "
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
".trim_matches('\n').to_owned()
}

const PART2: bool = true;

fn main() {
    println!("AOC 2025 Day 11");

    let contents = fs::read_to_string("src/bin/day11/input.txt")
        .expect("Failed to read input");
    let contents = contents.trim_matches('\n');

    println!("Part 1: {}", part1(contents));

    if PART2 {
        println!("Part 2: {}", part2(contents));
    }
}

#[test]
fn matmul() {
    let mat = SquareMatrix {
        data: vec![
            1, 2,
            3, 4
        ].into_boxed_slice(),
        n: 2
    };

    let out = SquareMatrix {
        data: vec![
             7, 10,
            15, 22,
        ].into_boxed_slice(),
        n: 2
    };

    assert_eq!(mat, mat, "sanity");
    assert_eq!(&mat * &mat, out);
}

#[test]
fn test_p1() {
    assert_eq!(part1(&example()), 5);
}

#[test]
fn test_p2() {
    if PART2 {
        assert_eq!(part2(&example2()), 2);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SquareMatrix<N: num_traits::int::PrimInt> {
    data: Box<[N]>,
    n: usize,
}
impl<N> SquareMatrix<N> where N: num_traits::int::PrimInt {
    fn new(n: usize) -> Self {
        let data = std::iter::repeat_n(N::zero(), n*n).collect::<Vec<_>>().into_boxed_slice();
        Self { data, n }
    }

    fn clear_diagonal(&mut self) {
        for i in 0..self.n {
            self.data[i * self.n + i] = N::zero();
        }
    }

    fn get(&self, row: usize, col: usize) -> &N {
        &self.data[row * self.n + col]
    }

    fn get_mut(&mut self, row: usize, col: usize) -> &mut N {
        &mut self.data[row * self.n + col]
    }

    fn is_all_zero(&self) -> bool {
        self.data.iter().all(|v| v.is_zero())
    }
}
impl<N> Index<Point<usize>> for SquareMatrix<N> where N: num_traits::PrimInt {
    type Output = N;

    fn index(&self, index: Point<usize>) -> &Self::Output {
        &self.data[index.y * self.n + index.x]
    }
}
impl<N> IndexMut<Point<usize>> for SquareMatrix<N> where N: num_traits::PrimInt {
    fn index_mut(&mut self, index: Point<usize>) -> &mut Self::Output {
        &mut self.data[index.y * self.n + index.x]
    }
}
impl<N> Display for SquareMatrix<N> where N: num_traits::PrimInt + Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.n == 0 {
            return write!(f, "[]");
        } else if self.n == 1 {
            return write!(f, "[{}]", self.data[0]);
        }

        let mut offset = 0;
        for row in 0..self.n {
            write!(f, "|")?;
            for col in 0..self.n {
                let v = self.data[offset + col];

                if row == Rack::YOU && col == Rack::OUT {
                    write!(f, " {}", utils::colorize(&format!("{}", v), 180, 255, 180))?;
                } else if v.is_zero() {
                    write!(f, " {}", utils::colorize(&format!("{}", v), 100, 100, 100))?;
                } else {
                    write!(f, " {}", v)?;
                }
            }

            writeln!(f, " |")?;
            offset += self.n;
        }

        Ok(())
    }
}
impl<N> Mul for &SquareMatrix<N> where N: num_traits::PrimInt + std::ops::AddAssign {
    type Output = SquareMatrix<N>;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.n, rhs.n, "cannot multiply differently-sized matrices");
        let mut out = SquareMatrix::new(self.n);

        for i in 0..self.n {
            for j in 0..self.n {
                let mut sum = N::zero();

                for k in 0..self.n {
                    sum += *self.get(i, k) * *rhs.get(k, j);
                }

                *out.get_mut(i, j) = sum;
            }
        }

        out
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Label([char; 3]);
impl Label {
    fn parse(input: &str) -> IResult<&str, Self> {
        (nom::character::anychar, nom::character::anychar, nom::character::anychar)
            .map(|(a, b, c)| Self([a, b, c]))
            .parse(input)
    }
}
impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.0[0], self.0[1], self.0[2])
    }
}

struct Rack {
    #[allow(unused)]
    connections: Vec<(Label, Vec<Label>)>,
    matrix0: SquareMatrix<i128>,
    matrix: SquareMatrix<i128>,
    total_count: usize,
}
impl Rack {
    const YOU: usize = 0;
    const OUT: usize = 1;

    fn parse_connection(input: &str) -> IResult<&str, (Label, Vec<Label>)> {
        separated_pair(Label::parse, nom::bytes::tag(": "), separated_list1(complete::char(' '), Label::parse)).parse(input)
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        let (remainder, connections) = separated_list1(complete::line_ending, Self::parse_connection).parse(input)?;
        let mut matrix = SquareMatrix::new(connections.len() + 1);

        let mut index_map = HashMap::new();
        index_map.insert(Label::parse("you").unwrap().1, Self::YOU);
        index_map.insert(Label::parse("out").unwrap().1, Self::OUT);

        for (lbl, _) in &connections {
            let len = index_map.len();
            index_map.entry(*lbl).or_insert(len);
        }

        for (from, connected) in &connections {
            let from_idx = *index_map.get(from).unwrap();
            for to in connected {
                let to_idx = *index_map.get(to).unwrap();
                *matrix.get_mut(from_idx, to_idx) = 1;
            }
        }

        let matrix0 = matrix.clone();
        Ok((remainder, Self { connections, matrix0, matrix, total_count: 0 }))
    }

    fn step(&mut self) -> bool {
        self.matrix = &self.matrix * &self.matrix0;
        // we don't allow revisiting a node, so clear paths from a node to itself in any number of steps
        self.matrix.clear_diagonal();
        self.total_count += *self.matrix.get(Self::YOU, Self::OUT) as usize;
        self.matrix.is_all_zero()
    }
}

fn part1(data: &str) -> usize {
    let mut rack = parse_complete(&mut Rack::parse, data);

    /*[bench exclude]*/ {
        println!("Original[{}x{}]:\n{}\n\n", rack.matrix0.n, rack.matrix0.n, rack.matrix0);
    }

    for i in 1usize.. {
        let all_zero = rack.step();

        /*[bench exclude]*/ {
            if cfg!(test) || i % 10 == 0 {
                println!("Step {}:\n{}\n", i, rack.matrix);
            } else {
                println!("Step {}", i);
            }
        }

        if all_zero {
            if cfg!(test) {
                println!("All zero!");
            }
            break;
        }
    }

    rack.total_count
}

#[derive(Clone, Copy, Debug, Default)]
struct PathCounter {
    through_both: usize,
    through_fft: usize,
    through_dac: usize,
    through_neither: usize
}
impl Add for PathCounter {
    type Output = PathCounter;

    fn add(self, rhs: Self) -> Self::Output {
        PathCounter {
            through_both: self.through_both + rhs.through_both,
            through_fft: self.through_fft + rhs.through_fft,
            through_dac: self.through_dac + rhs.through_dac,
            through_neither: self.through_neither + rhs.through_neither,
        }
    }
}
impl AddAssign for PathCounter {
    fn add_assign(&mut self, rhs: Self) {
        self.through_both += rhs.through_both;
        self.through_fft += rhs.through_fft;
        self.through_dac += rhs.through_dac;
        self.through_neither += rhs.through_neither;
    }
}

#[derive(Clone, Debug)]
struct Node {
    connections: Vec<Label>,
    /// the number of paths from here, to the target
    counts: PathCounter,
}
impl Node {
    fn new(connections: Vec<Label>) -> Self {
        Self { connections, counts: PathCounter::default() }
    }

    fn out() -> Self {
        Self {
            connections: Vec::new(),
            counts: PathCounter {
                through_neither: 1,
                ..PathCounter::default()
            }
        }
    }
}

#[derive(Clone, Debug)]
struct TopoRack {
    nodes: HashMap<Label, Node>,
    heights: Vec<(Label, usize)>,
}
impl TopoRack {
    const SVR: Label = Label(['s', 'v', 'r']);
    const OUT: Label = Label(['o', 'u', 't']);
    const DAC: Label = Label(['d', 'a', 'c']);
    const FFT: Label = Label(['f', 'f', 't']);

    fn parse(input: &str) -> IResult<&str, Self> {
        let (remainder, connections) = separated_list1(complete::line_ending, Rack::parse_connection).parse(input)?;
        let mut nodes: HashMap<Label, Node> = connections.into_iter()
            .map(|(key, labels)| (key, Node::new(labels)))
            .collect();
        nodes.insert(Self::OUT, Node::out());

        let mut heights: HashMap<Label, usize> = HashMap::new();
        heights.insert(Self::OUT, 0);

        fn calculate_height(node: Label, nodes: &HashMap<Label, Node>, heights: &mut HashMap<Label, usize>) -> usize {
            if let Some(&height) = heights.get(&node) {
                height
            } else {
                let height = nodes.get(&node).unwrap_or_else(|| panic!("{} exists", node))
                    .connections.iter()
                    .map(|child| calculate_height(*child, nodes, heights) + 1)
                    .max().unwrap_or(0);
                heights.insert(node, height);
                height
            }
        }
        calculate_height(Self::SVR, &nodes, &mut heights);

        let mut heights: Vec<(Label, usize)> = heights.into_iter().collect();
        heights.sort_unstable_by_key(|(_, height)| *height);

        Ok((remainder, Self { nodes, heights }))
    }

    fn print_heights(&self) {
        for (lbl, height) in &self.heights {
            println!("{} {}", lbl, height);
        }
    }

    fn count(&mut self) -> usize {
        for (lbl, _) in &self.heights {
            let mut total = PathCounter::default();

            for child in &self.nodes.get(lbl).unwrap().connections {
                let child_counter = self.nodes.get(child).unwrap().counts;
                total += child_counter;
            }

            match *lbl {
                Self::DAC => {
                    total.through_both += total.through_fft;
                    total.through_fft = 0;

                    total.through_dac += total.through_neither;
                    total.through_neither = 0;
                }
                Self::FFT => {
                    total.through_both += total.through_dac;
                    total.through_dac = 0;

                    total.through_fft += total.through_neither;
                    total.through_neither = 0;
                }
                _ => {}
            }

            self.nodes.get_mut(lbl).unwrap().counts += total;
        }

        self.nodes.get(&Self::SVR).unwrap().counts.through_both
    }
}

fn part2(data: &str) -> usize {
    let mut rack = parse_complete(&mut TopoRack::parse, data);
    if cfg!(test) {
        rack.print_heights();
    }
    rack.count()
}

