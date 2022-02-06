use std::collections::HashSet;
use itertools::Itertools;
use crate::day5::Vec2;
use crate::day9::{GridMap, Point};
use crate::read_lines;

fn step_one(data: &mut GridMap<u32>, flashed: &mut HashSet<Vec2>, c: &Vec2) {
    if let Some(p) = data.get_point_mut(&c) {
        *p.value += 1;

        if *p.value > 9 && !flashed.contains(c) {
            flashed.insert(p.coords);

            for neighbor_coord in p.coords.adjacent(true) {
                step_one(data, flashed, &neighbor_coord);
            }
        }
    }
}

fn step(data: &mut GridMap<u32>) -> usize {
    let coords = data.each_coord().collect_vec();
    let mut flashed = HashSet::<Vec2>::new();

    for c in coords {
        step_one(data, &mut flashed, &c);
    }

    for c in &flashed {
        *data.get_point_mut(c).unwrap().value = 0;
    }

    flashed.len()
}

fn read() -> GridMap<u32> {
    let data = read_lines("data/day11.txt").map(|line|
        line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec()
    ).collect_vec();
    GridMap { data }
}

pub fn part1() {
    let mut map = read();
    // println!("Step #0:");
    // println!("{}", map);

    let mut flashes = 0;
    for idx in 0..100 {
        flashes += step(&mut map);
        println!("Step #{} (flashes={}):\n{}", idx + 1, flashes, "-");
    }
    println!("Total flashes={}", flashes);
}

pub fn part2() {
    let mut map = read();

    let mut step_idx = 0usize;
    loop {
        step(&mut map);
        if map.each_point().all(|p| *p.value == 0) {
            break
        }
        else {
            step_idx += 1;
        }
    }
    println!("at={}", step_idx + 1);
}