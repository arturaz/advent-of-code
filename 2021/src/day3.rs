use std::fs::File;
use std::io::{BufReader, Lines};
use std::iter::Map;
use crate::read_lines;

#[derive(Debug)]
struct Counts {
    zeroes: usize,
    ones: usize
}
impl Counts {
    fn process(&mut self, c: u8) {
        match c {
            1 => self.ones += 1,
            0 => self.zeroes += 1,
            _ => panic!()
        }
    }

    fn most_common(&self) -> u8 {
        if self.zeroes > self.ones { 0 } else { 1 }
    }

    fn least_common(&self) -> u8 {
        if self.zeroes <= self.ones { 0 } else { 1 }
    }
}

fn read() -> Map<Map<Lines<BufReader<File>>, fn(std::io::Result<String>) -> String>, fn(String) -> Vec<u8>> {
    read_lines("data/day3.txt").map(|line|
        line.chars().map(|c| match c {
            '1' => 1,
            '0' => 0,
            _ => panic!()
        }).collect::<Vec<_>>()
    )
}

fn counts_of(lines: &Vec<Vec<u8>>) -> Vec<Counts> {
    let mut iter = lines.iter();
    let first = iter.next().unwrap();
    let mut counts = first.iter().map(|c| {
        let mut counts = Counts { zeroes: 0, ones: 0 };
        counts.process(*c);
        counts
    }).collect::<Vec<_>>();
    for line in iter {
        for (idx, c) in line.iter().enumerate() {
            counts[idx].process(*c)
        }
    }
    counts
}

fn read_counts() -> Vec<Counts> {
    let lines = read().collect::<Vec<_>>();
    counts_of(&lines)
}

pub fn bits_to_u32(slice: &[u8]) -> u32 {
    slice.iter().rev().enumerate()
        .map(|(idx, bit)| (*bit as u32) << idx)
        .fold(0, |a, b| { a | b })
}
pub fn bits_bool_to_u32(slice: &[bool]) -> u32 {
    slice.iter().rev().enumerate()
        .map(|(idx, bit)| (if *bit { 1 } else { 0 }) << idx)
        .fold(0, |a, b| { a | b })
}
pub fn bits_to_usize(slice: &[u8]) -> usize {
    slice.iter().rev().enumerate()
        .map(|(idx, bit)| (*bit as usize) << idx)
        .fold(0, |a, b| { a | b })
}
pub fn bits_bool_to_usize(slice: &[bool]) -> usize {
    slice.iter().rev().enumerate()
        .map(|(idx, bit)| (if *bit { 1 } else { 0 }) << idx)
        .fold(0, |a, b| { a | b })
}

pub fn part1() {
    let counts = read_counts();
    let gamma_bits = counts.iter().map(|c| c.most_common()).collect::<Vec<_>>();
    let epsilon_bits = counts.iter().map(|c| c.least_common()).collect::<Vec<_>>();
    let gamma = bits_to_u32(&gamma_bits);
    let epsilon = bits_to_u32(&epsilon_bits);
    let result = gamma * epsilon;
    println!("gamma_bits: {:?}", gamma_bits);
    println!("gamma: {:?}", gamma);
    println!("epsilon_bits: {:?}", epsilon_bits);
    println!("epsilon: {:?}", epsilon);
    println!("result: {:?}", result);
}

fn filter(
    mut lines: Vec<Vec<u8>>, bit_criteria: impl Fn(&Counts) -> u8
) -> Vec<u8> {
    let range = 0..lines[0].len();
    for idx in range {
        // println!("idx={}, lines: {:?}", idx, lines);

        let counts = counts_of(&lines);
        // println!("counts: {:?}", counts);
        // println!("most_common: {:?}", counts.iter().map(|c| c.most_common()).collect::<Vec<_>>());
        // println!("least_common: {:?}", counts.iter().map(|c| c.least_common()).collect::<Vec<_>>());
        let bit = bit_criteria(&counts[idx]);
        lines.retain(|line| line[idx] == bit);
        if lines.len() <= 1 { return lines.remove(0) }
    }
    panic!();
}

pub fn part2() {
    let lines = read().collect::<Vec<_>>();

    println!("oxygen");
    let oxygen_generator_rating_bits =
        filter(lines.clone(), |c| c.most_common());
    let oxygen_generator_rating = bits_to_u32(&oxygen_generator_rating_bits);

    println!("co2");
    let co2_scrubber_rating_bits =
        filter(lines, |c| c.least_common());
    let co2_scrubber_rating = bits_to_u32(&co2_scrubber_rating_bits);

    let result = oxygen_generator_rating * co2_scrubber_rating;

    println!("oxygen_generator_rating_bits: {:?}", oxygen_generator_rating_bits);
    println!("oxygen_generator_rating: {:?}", oxygen_generator_rating);
    println!("co2_scrubber_rating_bits: {:?}", co2_scrubber_rating_bits);
    println!("co2_scrubber_rating: {:?}", co2_scrubber_rating);
    println!("result: {:?}", result);
}