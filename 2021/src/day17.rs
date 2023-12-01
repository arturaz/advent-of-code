use core::fmt::{Display, Formatter, Write};
use std::cmp::Ordering;
use crate::day5::Vec2;
use crate::day9::{GridMap, Offset};
use crate::read_lines;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub struct Vec2Signed {
    /// aka row
    pub x: i64,
    /// aka col
    pub y: i64
}
impl Vec2Signed {
    pub fn inverse_x(&self) -> Self {
        Self { x: -self.x, y: self.y }
    }

    pub fn inverse_y(&self) -> Self {
        Self { x: self.x, y: -self.y }
    }
}
impl PartialOrd<Vec2Signed> for Vec2Signed {
    fn partial_cmp(&self, other: &Vec2Signed) -> Option<Ordering> {
        if self.x == other.x && self.y == other.y { Some(Ordering::Equal) }
        else if self.x > other.x && self.y > other.y { Some(Ordering::Greater) }
        else if self.y < other.x && self.y < other.y { Some(Ordering::Less) }
        else { None }
    }
}

impl Vec2Signed {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn to_vec2(&self, offset: &Offset) -> Vec2 {
        offset.map(self.x, self.y)
    }
}
impl Display for Vec2Signed {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.x, self.y))
    }
}

struct TargetArea {
    from: Vec2Signed,
    to: Vec2Signed
}

impl TargetArea {
    pub(crate) fn inverse_x(&self) -> Self {
        Self { from: self.from.inverse_x(), to: self.to.inverse_x() }
    }

    pub(crate) fn within(&self, c: &Vec2Signed) -> bool {
        c >= &self.from && c <= &self.to
    }

    pub(crate) fn missed(&self, c: &Vec2Signed) -> bool {
        c.y > self.to.y
    }

    fn each_coord(&self) -> impl Iterator<Item = Vec2Signed> + '_ {
        (self.from.x..=self.to.x).flat_map(|row| {
            (self.from.y..=self.to.y).map(move |col| {
                Vec2Signed::new(row, col)
            })
        })
    }
}
impl Display for TargetArea {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("[from={}, to={}]", self.from, self.to))
    }
}

fn read_coords() -> (Offset, TargetArea) {
    // target area: x=20..30, y=-10..-5
    let line = read_lines("data/day17_test.txt").next().unwrap();
    let xy = line.split_once(":").unwrap().1.trim();
    let (x, y) = xy.split_once(", ").unwrap();
    let (x_from, x_to) = x.split_once("=").unwrap().1.split_once("..").unwrap();
    let (y_from, y_to) = y.split_once("=").unwrap().1.split_once("..").unwrap();

    let col_from = x_from.parse::<i64>().unwrap();
    let col_to = x_to.parse::<i64>().unwrap();
    let row_from = y_from.parse::<i64>().unwrap();
    let row_to = y_to.parse::<i64>().unwrap();

    let offset = Offset {
        x: if row_from < 0 { -row_from } else { 0 } as usize,
        y: if col_from < 0 { -col_from } else { 0 } as usize
    };

    let from = Vec2Signed::new(row_from, col_from);
    let to = Vec2Signed::new(row_to, col_to);
    let target_area = TargetArea { from, to };

    (offset, target_area)
}

const SUBMARINE: Vec2Signed = Vec2Signed::new(0, 0);

fn to_grid(offset: Offset, target_area: &TargetArea) -> GridMap<Tile> {
    let mut map = GridMap::new_with_offset(offset);
    map.ensure_indexes_offset(&SUBMARINE, &Tile::Empty);
    *map.get_mut_offset(&SUBMARINE).unwrap() = Tile::Submarine;

    map.ensure_indexes_offset(&target_area.to, &Tile::Empty);
    let inversed_target_area = target_area.inverse_x();
    map.ensure_indexes_offset(&inversed_target_area.from, &Tile::Empty);
    map.ensure_indexes_offset(&inversed_target_area.to, &Tile::Empty);

    for coord in target_area.each_coord() {
        *map.get_mut_offset(&coord).unwrap() = Tile::Target;
    }
    map
}

#[derive(Copy, Clone)]
enum Tile { Submarine, Shot, Empty, Target }
impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_char(match self {
            Tile::Submarine => 'S',
            Tile::Shot => '#',
            Tile::Empty => '.',
            Tile::Target => 'T'
        })
    }
}

fn shoot(map: &mut GridMap<Tile>, target_area: &TargetArea, shoot_vector: &Vec2Signed) {
    let mut pos = *shoot_vector;
    let mut vec = *shoot_vector;
    while !target_area.within(&pos) && !target_area.missed(&pos) {
        println!("pos={}", pos);
        vec.y_towards_0();
    }
}

pub fn part1() {
    let (offset, target_area) = read_coords();
    println!("{:?}, target_area={}", offset, target_area);
    let map = to_grid(offset, &target_area);
    println!("{}", map);
}