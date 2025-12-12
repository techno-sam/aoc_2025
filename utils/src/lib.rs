use std::{collections::{HashMap, HashSet}, hash::Hash, ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign}, slice::SliceIndex};

use avl::AvlTreeSet;

#[inline]
#[must_use]
pub fn parse_complete<'a, P, O>(parser: &mut P, input: &'a str) -> O where P: nom::Parser<&'a str, Output = O, Error: std::fmt::Debug> {
    let (remainder, parsed) = parser.parse(input).expect("parsing failed");
    assert_eq!(remainder.len(), 0, "Non-empty remainder: '{}'", remainder);
    parsed
}

pub fn colorize(input: &str, r: u8, g: u8, b: u8) -> String {
    return "\x1b[38;2;".to_owned()+&r.to_string()+";"+&g.to_string()+";"+&b.to_string()+"m"+input+"\x1b[0m";
}

pub fn highlight(input: &str, actually: bool, r: u8, g: u8, b: u8) -> String {
    if !actually {
        return input.to_owned();
    }
    return "\x1b[48;2;".to_owned()+&r.to_string()+";"+&g.to_string()+";"+&b.to_string()+"m"+input+"\x1b[0m";
}

pub const RESET: &str = "\x1b[0m";

pub fn fg_string(c: Color) -> String {
    return "\x1b[38;2;".to_owned() + &c.r.to_string() + ";" + &c.g.to_string() + ";" + &c.b.to_string() + "m";
}

pub fn bg_string(c: Color) -> String {
    return "\x1b[48;2;".to_owned() + &c.r.to_string() + ";" + &c.g.to_string() + ";" + &c.b.to_string() + "m";
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}
impl Color {
    pub const WHITE: Color = Color::rgb(255, 255, 255);
    pub const BLACK: Color = Color::rgb(0, 0, 0);

    pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn hsl(h: f32, s: f32, l: f32) -> Color {
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let h_prime = h / 60.0;
        let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());

        let (r1, g1, b1) = match h_prime as u32 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            5 => (c, 0.0, x),
            _ => (0.0, 0.0, 0.0),
        };

        let m = l - c / 2.0;

        Color {
            r: ((r1 + m) * 255.0).round() as u8,
            g: ((g1 + m) * 255.0).round() as u8,
            b: ((b1 + m) * 255.0).round() as u8,
        }
    }

    pub fn random_from_seed(seed: usize) -> Color {
        // multiply by a large prime to simulate randomness
        let seed = seed.wrapping_mul(10722542609); 

        let hue = (seed % 360) as f32;
        let saturation = 0.9;
        let lightness = 0.6;

        Color::hsl(hue, saturation, lightness)
    }
}
impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::rgb(r, g, b)
    }
}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
}
impl Style {
    pub fn fg(c: Option<Color>) -> Style {
        Style {
            fg: c,
            bg: None
        }
    }

    pub fn bg(c: Option<Color>) -> Style {
        Style {
            fg: None,
            bg: c
        }
    }

    pub fn merge(&mut self, other: &Style) {
        if let None = self.fg {
            self.fg = other.fg;
        }

        if let None = self.bg {
            self.bg = other.bg;
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct StyledChar {
    pub chr: char,
    pub style: Style
}
impl StyledChar {
    pub fn of(chr: char) -> StyledChar {
        StyledChar {
            chr,
            style: Style::default()
        }
    }
}

pub trait StyleUtil {
    fn merge_style(&mut self, row: usize, column: usize, style: &Style);
}

impl StyleUtil for Vec<Vec<StyledChar>> {
    fn merge_style(&mut self, row: usize, column: usize, style: &Style) {
        self[row][column].style.merge(style);
    }
}

pub fn print_grid(input: &Vec<Vec<StyledChar>>) {
    for r in 0..input.len() {
        let row = &input[r];
        let mut last_style = Style::default();

        for tile in row {
            if tile.style != last_style {
                last_style = tile.style;
                print!("{}", RESET);

                if let Some(c) = tile.style.fg {
                    print!("{}", fg_string(c));
                }

                if let Some(c) = tile.style.bg {
                    print!("{}", bg_string(c));
                }
            }
            print!("{}", tile.chr);
        }

        if let (None, None) = (last_style.fg, last_style.bg) {} else {
            print!("{}", RESET);
        }

        print!("\n");
    }
}

pub fn parse_grid<T>(input: &str) -> Vec<Vec<T>> where T: From<char> {
    let g: Vec<Vec<T>> = input.trim()
        .split("\n")
        .map(|r| r.chars()
            .map(|c| c.into())
            .collect())
        .collect();

    assert_eq!(g.len() * g[0].len(), g.iter().map(|v| v.len()).sum(), "malformed grid");

    g
}

pub fn make_grid<V>(rows: usize, cols: usize, v: V) -> Vec<Vec<V>> where V: Copy {
    (0..rows).map(|_| (0..cols).map(|_| v).collect()).collect()
}

pub trait GridMap<T> {
    fn grid_map<F, B>(&self, f: F) -> Vec<Vec<B>> where F: FnMut(&T) -> B;
    fn row_map<F, B, R>(&self, f: F) -> Vec<R> where F: FnMut(&T) -> B, R: FromIterator<B>;
}

impl<T> GridMap<T> for Vec<Vec<T>> {
    fn grid_map<F, B>(&self, mut f: F) -> Vec<Vec<B>> where F: FnMut(&T) -> B {
        self.iter().map(|r| r.iter().map(&mut f).collect()).collect()
    }

    fn row_map<F, B, R>(&self, mut f: F) -> Vec<R> where F: FnMut(&T) -> B, R: FromIterator<B> {
        self.iter().map(|r| r.iter().map(&mut f).collect()).collect()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T
}
impl<T> Point<T> {
    pub fn map<U>(self, mapper: fn(T) -> U) -> Point<U> {
        Point {
            x: mapper(self.x),
            y: mapper(self.y)
        }
    }
}
impl<T> Add for Point<T> where T: Add<Output = T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: Add::add(self.x, rhs.x),
            y: Add::add(self.y, rhs.y)
        }
    }
}
impl<T> AddAssign for Point<T> where T: AddAssign {
    fn add_assign(&mut self, rhs: Self) {
        AddAssign::add_assign(&mut self.x, rhs.x);
        AddAssign::add_assign(&mut self.y, rhs.y);
    }
}
impl<T> Sub for Point<T> where T: Sub<Output = T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: Sub::sub(self.x, rhs.x),
            y: Sub::sub(self.y, rhs.y)
        }
    }
}
impl<T> SubAssign for Point<T> where T: SubAssign {
    fn sub_assign(&mut self, rhs: Self) {
        SubAssign::sub_assign(&mut self.x, rhs.x);
        SubAssign::sub_assign(&mut self.y, rhs.y);
    }
}
impl<T> Mul<T> for Point<T> where T: Mul<Output = T> + Copy {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: Mul::mul(self.x, rhs),
            y: Mul::mul(self.y, rhs)
        }
    }
}
impl<T> MulAssign<T> for Point<T> where T: MulAssign + Copy {
    fn mul_assign(&mut self, rhs: T) {
        MulAssign::mul_assign(&mut self.x, rhs);
        MulAssign::mul_assign(&mut self.y, rhs);
    }
}
impl<T> Div<T> for Point<T> where T: Div<Output = T> + Copy {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: Div::div(self.x, rhs),
            y: Div::div(self.y, rhs)
        }
    }
}
impl<T> DivAssign<T> for Point<T> where T: DivAssign + Copy {
    fn div_assign(&mut self, rhs: T) {
        DivAssign::div_assign(&mut self.x, rhs);
        DivAssign::div_assign(&mut self.y, rhs);
    }
}

impl<T> From<(T, T)> for Point<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1
        }
    }
}

impl<T> From<Point<T>> for (T, T) {
    fn from(value: Point<T>) -> Self {
        (value.x, value.y)
    }
}

impl<P, V> Index<Point<P>> for Vec<Vec<V>> where P: SliceIndex<[Vec<V>], Output = Vec<V>>, P: SliceIndex<[V], Output = V> {
    type Output = V;

    fn index(&self, index: Point<P>) -> &Self::Output {
        &self[index.y][index.x]
    }
}

impl<P, V> IndexMut<Point<P>> for Vec<Vec<V>> where
    P: SliceIndex<[Vec<V>], Output = Vec<V>>,
    P: SliceIndex<[V], Output = V>
{
    fn index_mut(&mut self, index: Point<P>) -> &mut Self::Output {
        &mut self[index.y][index.x]
    }
}

#[derive(Debug)]
pub struct Compactor<T> where T: std::cmp::Ord {
    xs: AvlTreeSet<T>,
    ys: AvlTreeSet<T>
}
impl<T> Default for Compactor<T> where T: std::cmp::Ord + num_traits::Num + AddAssign<T> + Copy  {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> Compactor<T> where T: std::cmp::Ord + num_traits::Num + AddAssign<T> + Copy {
    pub fn new() -> Self {
        Self { xs: AvlTreeSet::new(), ys: AvlTreeSet::new() }
    }

    pub fn add_key_point(&mut self, pt: Point<T>) {
        self.xs.insert(pt.x);
        self.ys.insert(pt.y);
    }

    pub fn width(&self) -> usize {
        self.xs.len()
    }

    pub fn height(&self) -> usize {
        self.ys.len()
    }

    pub fn compact(&self, pt: Point<T>) -> Point<T> {
        let mut x = T::zero();
        for &key_x in &self.xs {
            if key_x > pt.x {
                break;
            }

            x += T::one();
        }

        let mut y = T::zero();
        for &key_y in &self.ys {
            if key_y > pt.y {
                break;
            }

            y += T::one();
        }

        Point { x, y }
    }
}

pub trait DijkstraNode<T> where Self: PartialEq + Eq + Hash + Clone {
    /// Returns a vector of (node, distance) pairs
    fn get_connected(&self, context: &T) -> Vec<(Self, usize)> where Self: Sized;
}

pub struct DijkstraData<Node, T> where Node: DijkstraNode<T> {
    unvisited: HashSet<Node>,
    visited: HashSet<Node>,
    pub best_distance: HashMap<Node, usize>,
    pub prev_in_chain: HashMap<Node, Node>,
    context: T
}
impl <Node, T>DijkstraData<Node, T> where Node: DijkstraNode<T> {
    /// note: does NOT add initial to frontier (unvisited nodes)
    fn new(initial: Node, context: T) -> DijkstraData<Node, T> {
        let unvisited = HashSet::new();
        let visited = {
            let mut visited = HashSet::new();
            visited.insert(initial.clone());
            visited
        };
        let best_distance = {
            let mut best_distance = HashMap::new();
            best_distance.insert(initial, 0);
            best_distance
        };
        DijkstraData { unvisited, visited, best_distance, prev_in_chain: HashMap::new(), context }
    }

    fn get_best_unvisited(&self) -> Option<&Node> {
        if self.unvisited.is_empty() {
            return None;
        }
        let mut best: Option<&Node> = None;
        let mut best_distance = usize::MAX;
        for node in &self.unvisited {
            let dist = *self.best_distance.get(node).expect("Missing best distance for unvisited point");
            //.unwrap_or(&usize::max_value());
            if dist <= best_distance {
                best_distance = dist;
                best = Some(node);
            }
        }

        best
    }


    pub fn dijkstra(initial: Node, context: T, should_halt: impl Fn(&Node) -> bool) -> DijkstraData<Node, T> {
        let mut data = DijkstraData::new(initial.clone(), context);
        let context = &data.context;
        for (other, distance) in initial.get_connected(context) {
            data.best_distance.insert(other.clone(), distance);
            data.prev_in_chain.insert(other.clone(), initial.clone());
            data.unvisited.insert(other);
        }

        while let Some(cur) = data.get_best_unvisited() {
            let dist_so_far = *data.best_distance.get(cur).unwrap();
            let cur = cur.clone();

            for (other, dist) in cur.get_connected(context) {
                if data.visited.contains(&other) {
                    continue;
                }

                data.unvisited.insert(other.clone());
                let new_dist = dist_so_far + dist;
                let (better, best_dist) = match data.best_distance.get(&other) {
                    None => (true, new_dist),
                    Some(&existing) => {
                        if new_dist < existing {
                            (true, new_dist)
                        } else {
                            (false, existing)
                        }
                    }
                };
                if better {
                    data.prev_in_chain.insert(other.clone(), cur.clone());
                }
                data.best_distance.insert(other, best_dist);
            }
            data.unvisited.remove(&cur);
            data.visited.insert(cur.clone());
            if should_halt(&cur) {
                return data;
            }
        }

        data
    }
}

#[cfg(test)]
#[allow(dead_code, unused_imports)]
mod tests {
    use super::*;
    use char_enum_impl::data_enum;

    // graph from https://www.youtube.com/watch?v=bZkzH5x0SKU
    #[derive(PartialEq, Eq, Hash, Clone, Copy)]
    #[data_enum(Vec<(T, usize)>)]
    enum T {
        A = vec![(T::B, 2), (T::D, 8)],
        B = vec![(T::A, 2), (T::D, 5), (T::E, 6)],
        C = vec![(T::E, 9), (T::F, 3)],
        D = vec![(T::A, 8), (T::B, 5), (T::E, 3), (T::F, 2)],
        E = vec![(T::B, 6), (T::C, 9), (T::D, 2), (T::F, 1)],
        F = vec![(T::C, 3), (T::D, 2), (T::E, 1)]
    }
    impl DijkstraNode<()> for T {
        fn get_connected(&self, _: &()) -> Vec<(Self, usize)> where Self: Sized {
            return self.value();
        }
    }

    #[test]
    fn dijkstra_search() {
        fn hlt(node: &T) -> bool { *node == T::E }
        let a = DijkstraData::dijkstra(T::A, (), hlt);
        assert_eq!(Some(&8_usize), a.best_distance.get(&T::E), "Early halt");

        fn never(_: &T) -> bool { false }
        let a = DijkstraData::dijkstra(T::A, (), never);
        assert_eq!(Some(&12_usize), a.best_distance.get(&T::C), "Halt-less");
    }
}
