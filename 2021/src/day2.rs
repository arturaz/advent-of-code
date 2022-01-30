use std::fs::File;
use std::io::{BufReader, Lines};
use std::iter::Map;
use crate::read_lines;

fn read() -> Map<Lines<BufReader<File>>, fn(std::io::Result<String>) -> String> {
    read_lines("data/day2.txt")
}

/// https://adventofcode.com/2021/day/2
pub fn part1() {
    let lines = read();
    let mut depth = 0;
    let mut horizontal_position = 0;
    for line in lines {
        let mut words = line.split(" ");
        let command = words.next().unwrap();
        let num = words.next().unwrap().parse::<i32>().unwrap();

        match command {
            "forward" => {
                horizontal_position += num
            },
            "down" => {
                depth += num;
            },
            "up" => {
                depth -= num;
            },
            _ => panic!()
        }
    }

    println!("{}", depth * horizontal_position);
}

/// https://adventofcode.com/2021/day/2#part2
pub fn part2() {
    let lines = read();
    let mut aim = 0;
    let mut depth = 0;
    let mut horizontal_position = 0;
    for line in lines {
        let mut words = line.split(" ");
        let command = words.next().unwrap();
        let num = words.next().unwrap().parse::<i32>().unwrap();

        match command {
            "down" => aim += num,
            "up" => aim -= num,
            "forward" => {
                horizontal_position += num;
                depth += aim * num;
            },
            _ => panic!()
        }
    }

    println!("{}", depth * horizontal_position);
}