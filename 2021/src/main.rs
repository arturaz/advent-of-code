#![allow(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::Map;

mod day1;
mod day2;
mod day3;

fn main() {
    day3::part2();
}

pub fn read_lines(path: &str) -> Map<Lines<BufReader<File>>, fn(std::io::Result<String>) -> String> {
    let file = File::open(path).unwrap();
    BufReader::new(file).lines().map(|line| line.unwrap())
}