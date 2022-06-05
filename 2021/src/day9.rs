use alloc::rc::Rc;
use core::fmt::{Display, Formatter};
use std::collections::{HashMap, HashSet};
use std::iter::{FlatMap, Map};
use std::ops::{Deref, Range};
use itertools::Itertools;
use crate::day17::Vec2Signed;
use crate::day5::Vec2;
use crate::read_lines;

#[derive(Debug)]
pub struct Point<A> {
    pub coords: Vec2,
    pub value: A
}
impl<A : Copy> Point<&A> {
    pub fn copy(&self) -> Point<A> {
        Point { coords: self.coords, value: *self.value }
    }
}

impl Point<u32> {
    fn risk_level(&self) -> u32 { self.value + 1 }
}
impl<A : Display> Display for Point<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{}@{}", self.value, self.coords))
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Offset {
    pub x: usize,
    pub y: usize
}
impl Offset {
    pub fn contramap_row(&self, row: usize) -> i64 {
        (row as i64) - (self.x as i64)
    }

    pub fn contramap_col(&self, col: usize) -> i64 {
        (col as i64) - (self.y as i64)
    }

    pub fn map(&self, row: i64, col: i64) -> Vec2 {
        Vec2::new((row + self.x as i64) as usize, (col + self.y as i64) as usize)
    }
}

pub struct GridMap<A> {
    pub data: Vec<Vec<A>>,
    pub offset: Offset
}
impl<A> GridMap<A> {
    pub fn new() -> Self { GridMap { data: Vec::new(), offset: Offset::default() } }
    pub fn new_with_data(data: Vec<Vec<A>>) -> Self { GridMap { data, offset: Offset::default() } }
    pub fn new_with_offset(offset: Offset) -> Self { GridMap { data: Vec::new(), offset } }

    pub fn each_coord<'a>(&'a self) -> impl Iterator<Item = Vec2> + 'a {
        (0..self.data.len()).flat_map(|x|
            (0..self.data[x].len()).map(move |y| Vec2::new(x, y))
        )
    }

    pub fn each_point<'a>(&'a self) -> impl Iterator<Item = Point<&A>> + 'a {
        self.each_coord().map(|c| {
            self.get_point(&c).unwrap()
        })
    }

    pub fn get(&self, coord: &Vec2) -> Option<&A> {
        self.data.get(coord.x).and_then(|row|
            row.get(coord.y)
        )
    }

    pub fn get_mut(&mut self, coord: &Vec2) -> Option<&mut A> {
        self.data.get_mut(coord.x).and_then(|row|
            row.get_mut(coord.y)
        )
    }

    pub fn get_mut_offset(&mut self, coord: &Vec2Signed) -> Option<&mut A> {
        self.get_mut(&coord.to_vec2(&self.offset))
    }

    pub fn get_point(&self, coord: &Vec2) -> Option<Point<&A>> {
        self.get(coord).map(|v| Point { coords: coord.clone(), value: v} )
    }

    pub fn get_point_mut(&mut self, coord: &Vec2) -> Option<Point<&mut A>> {
        self.get_mut(coord).map(|v| Point { coords: *coord, value: v} )
    }

    pub fn get1(&self, x: usize, y: usize) -> Option<&A> {
        self.get(&Vec2::new(x, y))
    }

    pub fn get_up(&self, c: &Vec2) -> Option<&A> {
        c.x_sub1().and_then(|c| self.get(&c))
    }

    pub fn get_down(&self, c: &Vec2) -> Option<&A> {
        self.get(&c.x_plus1())
    }

    pub fn get_left(&self, c: &Vec2) -> Option<&A> {
        c.y_sub1().and_then(|c| self.get(&c))
    }

    pub fn get_right(&self, c: &Vec2) -> Option<&A> {
        self.get(&c.y_plus1())
    }

    pub fn get_left_up(&self, c: &Vec2) -> Option<&A> {
        c.x_sub1().and_then(|c| c.y_sub1()).and_then(|c| self.get(&c))
    }

    pub fn get_right_up(&self, c: &Vec2) -> Option<&A> {
        c.x_sub1().map(|c| c.y_plus1()).and_then(|c| self.get(&c))
    }

    pub fn get_left_down(&self, c: &Vec2) -> Option<&A> {
        c.y_sub1().map(|c| c.x_plus1()).and_then(|c| self.get(&c))
    }

    pub fn get_right_down(&self, c: &Vec2) -> Option<&A> {
        self.get(&c.y_plus1().x_plus1())
    }
}
impl<A : Clone> GridMap<A> {
    pub fn ensure_indexes(&mut self, c: &Vec2, default_value: &A) {
        let ensure_row_size = |row: &mut Vec<A>| {
            if c.y >= row.len() {
                row.resize_with(c.y + 1, || default_value.clone());
            }
        };

        if c.x >= self.data.len() {
            self.data.resize_with(c.x + 1, || {
                let mut row = Vec::<A>::new();
                ensure_row_size(&mut row);
                row
            });
        }

        for row in &mut self.data {
            ensure_row_size(row);
        }
    }

    pub fn ensure_indexes_offset(&mut self, c: &Vec2Signed, default_value: &A) {
        self.ensure_indexes(&c.to_vec2(&self.offset), default_value)
    }
}

impl<A : Display> Display for GridMap<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let rows = self.data.len();
        let should_invert_rows = self.offset.contramap_row(0) < 0;
        let row_prec = rows.log10() as usize + 1 + (if should_invert_rows { 1 } else { 0 });

        if rows == 0 { return Ok(()) }

        let cols = self.data[0].len();
        let col_prec = cols.log10() as usize + 1;

        let col_num_prefix = " ".repeat(row_prec + 1);
        for prec in (0..col_prec).rev() {
            f.write_str(&col_num_prefix)?;
            for col in 0..cols {
                let col = self.offset.contramap_col(col);
                let div = 10_usize.pow(prec as u32) as i64;
                let col_modded = col / div % 10;
                // f.write_fmt(format_args!(
                //     "prec={}, div={}, col={}, col_modded={}\n", prec, div, col, col_modded
                // ))?;
                f.write_fmt(format_args!("{}", col_modded))?;
            }
            f.write_str("\n")?;
        }

        let rows_iter = self.data.iter().enumerate();
        let rows_iter: Box<dyn Iterator<Item = _>> =
            if should_invert_rows { Box::new(rows_iter.rev()) } else { Box::new(rows_iter) };
        for (row_idx, row) in rows_iter {
            let row_idx = self.offset.contramap_row(row_idx);
            f.write_fmt(format_args!("{:prec$} ", row_idx, prec = row_prec))?;
            for a in row {
                f.write_fmt(format_args!("{}", a))?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

struct Basin {
    points: HashSet<Vec2>
}
impl Basin {
    fn size(&self) -> usize { self.points.len() }
}

type HeightMap = GridMap<u32>;
impl HeightMap {
    fn is_low_point(&self, c: &Vec2) -> bool {
        let v = self.get(c).unwrap();
        let is_lower_than = |maybe_v1: Option<&u32>| match maybe_v1 {
            None => true,
            Some(v1) => *v < *v1
        };

        is_lower_than(self.get_left(c)) && is_lower_than(self.get_right(c)) &&
            is_lower_than(self.get_up(c)) && is_lower_than(self.get_down(c))
    }

    fn low_points<'a>(&'a self) -> impl Iterator<Item = Point<&u32>> + 'a {
        self.each_point().filter(|p| self.is_low_point(&p.coords))
    }

    fn basins(&self) -> Vec<Basin> {
        let mut basins = Vec::<Basin>::new();
        let mut checked = HashSet::<Vec2>::new();

        // Recursive closure: https://stackoverflow.com/a/16953239/935259
        struct FillBasin<'s> { f: &'s dyn Fn(&FillBasin, &Point<u32>, &mut Basin) -> () }
        let fill_basin = FillBasin { f: &|s, p, add_to| {
            add_to.points.insert(p.coords);

            let should_include = |maybe_c: Option<Vec2>, basin: &Basin| -> Option<Point<u32>> {
                // println!("  ### {:?}", maybe_c);
                maybe_c.and_then(|c| self.get_point(&c)).and_then(|p2| {
                    let is_top = *p2.value == 9;
                    let is_higher = *p2.value >= p.value;
                    let is_in_basin = basin.points.contains(&p2.coords);
                    let should_include = !is_top && is_higher && !is_in_basin;
                    // println!("  ### {} -> {}", p, p2);
                    // println!("  is_top: {}, is_higher={}, is_in_basin={}", is_top, is_higher, is_in_basin);
                    // println!("  should_include: {}", should_include);
                    // println!();
                    if should_include { Some(Point { coords: p2.coords, value: *p2.value }) } else { None }
                })
            };

            if let Some(p2) = should_include(p.coords.x_sub1(), add_to) {
                (s.f)(s, &p2, add_to)
            }
            if let Some(p2) = should_include(Some(p.coords.x_plus1()), add_to) {
                (s.f)(s, &p2, add_to)
            }
            if let Some(p2) = should_include(p.coords.y_sub1(), add_to) {
                (s.f)(s, &p2, add_to)
            }
            if let Some(p2) = should_include(Some(p.coords.y_plus1()), add_to) {
                (s.f)(s, &p2, add_to)
            }
        } };

        for p in self.low_points() {
            if !checked.contains(&p.coords) {
                let mut basin = Basin { points: HashSet::new() };
                // println!("New basin, filling from {}", p.coords);
                (fill_basin.f)(&fill_basin, &p.copy(), &mut basin);

                for point in &basin.points {
                    checked.insert(*point);
                }

                basins.push(basin);
            }
        }

        basins
    }
}

fn read() -> HeightMap {
    let data = read_lines("data/day9.txt").map(|line|
        line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec()
    ).collect_vec();
    HeightMap { data, offset: Offset::default() }
}

pub fn part1() {
    let map = read();
    let low_points: u32 = map.low_points().map(|p| p.copy().risk_level()).sum();
    println!("result={}", low_points);
}

fn render_basin(map: &HeightMap, basin: &Basin) {
    for x in 0..map.data.len() {
        let row = &map.data[x];
        for y in 0..row.len() {
            let c = Vec2::new(x, y);
            if basin.points.contains(&c) {
                print!("{}", row[y]);
            }
            else {
                print!(" ");
            }
        }
        println!()
    }
}

pub fn part2() {
    let map = read();
    let basins = map.basins();
    // for (idx, basin) in basins.iter().enumerate() {
    //     println!("Basin #{}, size={}", idx, basin.size());
    //     render_basin(&map, basin);
    //     println!();
    // }

    let result =
        basins.iter().map(|b| b.size()).sorted().rev().take(3)
            .fold(1, |a, b| a * b);
    println!("result={}", result);
}