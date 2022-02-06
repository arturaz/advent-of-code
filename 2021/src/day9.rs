use alloc::rc::Rc;
use core::fmt::{Display, Formatter};
use std::collections::{HashMap, HashSet};
use std::iter::{FlatMap, Map};
use std::ops::{Deref, Range};
use itertools::Itertools;
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

pub struct GridMap<A> {
    pub data: Vec<Vec<A>>
}
impl<A> GridMap<A> {
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

impl<A : Display> Display for GridMap<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        for row in &self.data {
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
    HeightMap { data }
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