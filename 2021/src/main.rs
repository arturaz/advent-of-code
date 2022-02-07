#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(drain_filter)]
#![feature(int_abs_diff)]
#![feature(is_some_with)]

extern crate alloc;
extern crate core;

use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::Map;
use std::ops::{Add, Div, Mul, Sub};
use std::process::Output;
use std::str::FromStr;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;

fn main() {
    day12::part1();
}

pub fn read_lines(path: &str) -> Map<Lines<BufReader<File>>, fn(std::io::Result<String>) -> String> {
    let file = File::open(path).unwrap();
    BufReader::new(file).lines().map(|line| line.unwrap())
}

pub fn read_num_line<A : FromStr>(path: &str) -> Vec<A> where <A as FromStr>::Err : Debug {
    let line = read_lines(path).next().unwrap();
    line.split(",").map(|s| s.parse::<A>().unwrap()).collect()
}

pub fn arithmetic_sum<
    A : Copy + From<u8> + PartialOrd +
    Add<Output = A> + Sub<Output = A> + Div<Output = A> + Mul<Output = A>
>(from: A, to_exclusive: A) -> Option<A> {
    if to_exclusive <= from { None }
    else {
        let to = to_exclusive - 1.into();
        let count = to_exclusive - from;
        let two: A = 2.into();
        Some(count * (from + to) / two)
    }
}