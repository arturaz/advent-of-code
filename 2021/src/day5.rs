use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Lines};
use std::iter::Map;
use crate::read_lines;

#[derive(Eq, PartialEq, Hash, Debug)]
struct Vec2 { x: u32, y: u32 }
impl Vec2 {
    fn new(x: u32, y: u32) -> Self {
        Vec2 { x, y }
    }
}

#[derive(Debug)]
struct Line { from: Vec2, to: Vec2 }
impl Line {
    fn coordinates(&self, include_diagonals: bool) -> Vec<Vec2> {
        let x_range: Vec<_> =
            if self.from.x < self.to.x { (self.from.x..(self.to.x + 1)).collect() }
            else { (self.to.x..(self.from.x + 1)).rev().collect() };
        let y_range: Vec<_> =
            if self.from.y < self.to.y { (self.from.y..(self.to.y + 1)).collect() }
            else { (self.to.y..(self.from.y + 1)).rev().collect() };
        if self.from.x == self.to.x {
            let x = self.from.x;
            y_range.iter().map(|y| Vec2::new(x, *y) ).collect()
        }
        else if self.from.y == self.to.y {
            let y = self.from.y;
            x_range.iter().map(|x| Vec2::new(*x, y) ).collect()
        }
        else if include_diagonals {
            x_range.iter().zip(y_range.iter())
                .map(|(x, y)| Vec2::new(*x, *y) ).collect()
        }
        else {
            Vec::new()
        }
    }
}

fn parse_point(s: &str) -> Vec2 {
    let mut elements =
        s.split(",").map(|s| s.parse::<u32>().unwrap());
    Vec2::new(elements.next().unwrap(), elements.next().unwrap())
}

fn read() -> Map<Map<Lines<BufReader<File>>, fn(std::io::Result<String>) -> String>, fn(String) -> Line> {
    read_lines("data/day5.txt").map(|line| {
        let mut iter = line.split_whitespace();
        let p1 = parse_point(iter.next().unwrap());
        iter.next();
        let p2 = parse_point(iter.next().unwrap());
        Line { from: p1, to: p2 }
    })
}

fn print(map: &HashMap<Vec2, u32>) {
    let max_x = map.keys().map(|v| v.x).max().unwrap();
    let max_y = map.keys().map(|v| v.y).max().unwrap();
    for y in 0..(max_y + 1) {
        for x in 0..(max_x + 1) {
            let entry = map.get(&Vec2::new(x, y)).unwrap_or(&0);
            print!("{}", if *entry == 0 { String::from(".") } else { entry.to_string() })
        }
        println!()
    }
}

fn run(include_diagonals: bool) {
    let mut map = HashMap::<Vec2, u32>::new();
    for line in read() {
        // println!("{:?}", line);
        for point in line.coordinates(include_diagonals) {
            // println!("{:?}", point);
            let entry = map.entry(point).or_insert(0);
            *entry += 1;
        }
    }
    // print(&map);

    let count = map.values().filter(|cnt| **cnt > 1).count();
    println!("count={}", count);
}

pub fn part1() {
    run(false);
}

pub fn part2() {
    run(true);
}