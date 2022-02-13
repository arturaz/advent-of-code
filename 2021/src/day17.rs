use core::fmt::{Display, Formatter, Write};
use crate::day5::Vec2;
use crate::day9::GridMap;
use crate::read_lines;

#[derive(Debug)]
struct Offset {
    x: usize,
    y: usize
}
impl Offset {
    fn map(&self, row: i64, col: i64) -> Vec2 {
        Vec2::new((row + self.x as i64) as usize, (col + self.y as i64) as usize)
    }
}

fn read_coords() -> (Offset, Vec2, Vec2) {
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

    let from = offset.map(row_from, col_from);
    let to = offset.map(row_to, col_to);

    (offset, from, to)
}

fn to_grid(offset: &Offset, from: &Vec2, to: &Vec2) -> GridMap<Tile> {
    let mut map = GridMap::new();
    let submarine = offset.map(0, 0);
    map.ensure_indexes(&submarine, &Tile::Empty);
    *map.get_mut(&submarine).unwrap() = Tile::Submarine;

    map.ensure_indexes(&to, &Tile::Empty);
    for row in from.x..=to.x {
        for col in from.y..=to.y {
            *map.get_mut(&Vec2::new(row, col)).unwrap() = Tile::Target;
        }
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

pub fn part1() {
    let (offset, from, to) = read_coords();
    println!("{:?}, from={}, to={}", offset, from, to);
    let map = to_grid(&offset, &from, &to);
    println!("{}", map);
}