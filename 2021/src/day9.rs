use alloc::rc::Rc;
use core::fmt::{Display, Formatter};
use std::collections::{HashMap, HashSet};
use std::iter::{FlatMap, Map};
use std::ops::Range;
use itertools::Itertools;
use crate::day5::Vec2;
use crate::read_lines;

#[derive(Debug)]
struct Point {
    coords: Vec2,
    height: u32
}

impl Point {
    fn risk_level(&self) -> u32 { self.height + 1 }
}
impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{}@{}", self.height, self.coords))
    }
}

struct Basin {
    points: HashSet<Vec2>
}
impl Basin {
    fn size(&self) -> usize { self.points.len() }
}

struct HeightMap {
    data: Vec<Vec<u32>>
}
impl HeightMap {
    fn each_coord<'a>(&'a self) -> impl Iterator<Item = Vec2> + 'a {
        (0..self.data.len()).flat_map(|x|
            (0..self.data[x].len()).map(move |y| Vec2::new(x, y))
        )
    }

    fn each_point<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
        self.each_coord().map(|c| {
            self.get_point(&c).unwrap()
        })
    }

    fn get(&self, coord: &Vec2) -> Option<u32> {
        self.data.get(coord.x).and_then(|row|
            row.get(coord.y).map(|v| *v)
        )
    }

    fn get_point(&self, coord: &Vec2) -> Option<Point> {
        self.get(coord).map(|v| Point { coords: coord.clone(), height: v} )
    }

    fn get1(&self, x: usize, y: usize) -> Option<u32> {
        self.get(&Vec2::new(x, y))
    }

    fn get_up(&self, c: &Vec2) -> Option<u32> {
        c.x_sub1().and_then(|c| self.get(&c))
    }

    fn get_down(&self, c: &Vec2) -> Option<u32> {
        self.get(&c.x_plus1())
    }

    fn get_left(&self, c: &Vec2) -> Option<u32> {
        c.y_sub1().and_then(|c| self.get(&c))
    }

    fn get_right(&self, c: &Vec2) -> Option<u32> {
        self.get(&c.y_plus1())
    }

    fn is_low_point(&self, c: &Vec2) -> bool {
        let v = self.get(c).unwrap();
        let is_lower_than = |maybe_v1: Option<u32>| match maybe_v1 {
            None => true,
            Some(v1) => v < v1
        };

        is_lower_than(self.get_left(c)) && is_lower_than(self.get_right(c)) &&
            is_lower_than(self.get_up(c)) && is_lower_than(self.get_down(c))
    }

    fn low_points<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
        self.each_point().filter(|p| self.is_low_point(&p.coords))
    }

    fn basins(&self) -> Vec<Basin> {
        let mut basins = Vec::<Basin>::new();
        let mut checked = HashSet::<Vec2>::new();

        // Recursive closure: https://stackoverflow.com/a/16953239/935259
        struct FillBasin<'s> { f: &'s dyn Fn(&FillBasin, &Point, &mut Basin) -> () }
        let fill_basin = FillBasin { f: &|s, p, add_to| {
            add_to.points.insert(p.coords);

            let should_include = |maybe_c: Option<Vec2>, basin: &Basin| -> Option<Point> {
                // println!("  ### {:?}", maybe_c);
                maybe_c.and_then(|c| self.get_point(&c)).and_then(|p2| {
                    let is_top = p2.height == 9;
                    let is_higher = p2.height >= p.height;
                    let is_in_basin = basin.points.contains(&p2.coords);
                    let should_include = !is_top && is_higher && !is_in_basin;
                    // println!("  ### {} -> {}", p, p2);
                    // println!("  is_top: {}, is_higher={}, is_in_basin={}", is_top, is_higher, is_in_basin);
                    // println!("  should_include: {}", should_include);
                    // println!();
                    if should_include { Some(p2) } else { None }
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
                (fill_basin.f)(&fill_basin, &p, &mut basin);

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
    let low_points: u32 = map.low_points().map(|p| p.risk_level()).sum();
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