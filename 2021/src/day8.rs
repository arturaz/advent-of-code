use std::collections::{HashMap, HashSet};
use maplit::{hashmap, hashset};
use once_cell::sync::Lazy;
use crate::read_lines;

#[derive(Eq, PartialEq, Hash)]
enum Digit { _0, _1, _2, _3, _4, _5, _6, _7, _8, _9 }

struct DigitWires {
    wires: Vec<Segment>
}

#[derive(Eq, PartialEq, Hash)]
enum Segment { A, B, C, D, E, F, G }
impl Segment {
    fn parse(c: char) -> Option<Segment> {
        match c {
            'a' => Some(Segment::A),
            'b' => Some(Segment::B),
            'c' => Some(Segment::C),
            'd' => Some(Segment::D),
            'e' => Some(Segment::E),
            'f' => Some(Segment::F),
            'g' => Some(Segment::G),
            _ => None
        }
    }

    fn parse_chunk(s: &str) -> Option<DigitWires> {
        let wires: Option<Vec<Segment>> = s.chars().map(|c| Segment::parse(c)).collect();
        wires.map(|w| DigitWires { wires: w })
    }

    fn parse_line(s: &str) -> Option<Vec<DigitWires>> {
        s.split_whitespace().map(|chunk| Segment::parse_chunk(chunk)).collect()
    }
}

static NORMAL_MAPPING: Lazy<HashMap<Digit, HashSet<Segment>>> = Lazy::new(|| hashmap! {
    Digit::_0 => hashset! { Segment::A, Segment::B, Segment::C, Segment::E, Segment::F, Segment::G },
    Digit::_1 => hashset! { Segment::C, Segment::F },
    Digit::_2 => hashset! { Segment::A, Segment::C, Segment::D, Segment::E, Segment::G },
    Digit::_3 => hashset! { Segment::A, Segment::C, Segment::D, Segment::F, Segment::G },
    Digit::_4 => hashset! { Segment::B, Segment::C, Segment::D, Segment::F },
    Digit::_5 => hashset! { Segment::A, Segment::B, Segment::D, Segment::F, Segment::G },
    Digit::_6 => hashset! { Segment::A, Segment::B, Segment::D, Segment::E, Segment::F, Segment::G },
    Digit::_7 => hashset! { Segment::A, Segment::C, Segment::F },
    Digit::_8 => hashset! { Segment::A, Segment::B, Segment::C, Segment::D, Segment::E, Segment::F, Segment::G },
    Digit::_9 => hashset! { Segment::A, Segment::B, Segment::C, Segment::D, Segment::F, Segment::G },
});

struct Line {
    input_patterns: Vec<DigitWires>,
    outputs: Vec<DigitWires>
}

fn read() -> Vec<Line> {
    read_lines("data/day8.txt").map(|line| {
        let mut iter = line.split(" | ");
        let input_patterns = Segment::parse_line(iter.next().unwrap()).unwrap();
        let outputs = Segment::parse_line(iter.next().unwrap()).unwrap();
        Line { input_patterns, outputs }
    }).collect()
}

pub fn part1() {
    let counted: usize = read().iter().map(|line|
        line.outputs.iter().filter(|w| match w.wires.len() {
            2 | 4 | 3 | 7 => true,
            _ => false
        }).count()
    ).sum();
    println!("result={}", counted);
}