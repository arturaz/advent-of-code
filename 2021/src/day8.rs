use alloc;
use core::fmt::Write;
use core::slice::Iter;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::iter::Map;
use std::ops::Deref;
use itertools::Itertools;
use maplit::{hashmap, hashset};
use once_cell::sync::Lazy;
use crate::read_lines;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
enum Digit { _0 = 0, _1 = 1, _2 = 2, _3 = 3, _4 = 4, _5 = 5, _6 = 6, _7 = 7, _8 = 8, _9 = 9 }

#[derive(Eq, PartialEq)]
struct DigitWires {
    wires: HashSet<Segment>
}
impl DigitWires {
    fn new(wires: HashSet<Segment>) -> Self { DigitWires { wires } }

    fn iter(&self) -> alloc::vec::IntoIter<&Segment> { self.wires.iter().sorted() }
}
impl Hash for DigitWires {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for segment in self.iter() {
            segment.hash(state)
        }
    }
}
impl Debug for DigitWires {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::from("[");
        for s in self.iter() {
            str.push_str(format!("{:?}", s).as_str());
        }
        str.push(']');
        f.write_str(str.as_str())
    }
}

#[derive(Debug)]
struct DecoderMap {
    map: HashMap<DigitWires, Digit>
}

impl DecoderMap {
    fn decode(&self, encoded: &DigitWires) -> Option<&Digit> {
        self.map.get(encoded)
    }

    fn decode_iter(&self, iter: Iter<DigitWires>) -> DecodedDigits {
        let digits =
            iter.map(|encoded| self.decode(encoded).unwrap().clone()).collect_vec();
        DecodedDigits { digits }
    }
}

#[derive(Debug)]
struct DecodedDigits {
    digits: Vec<Digit>
}

impl DecodedDigits {
    fn to_usize(&self) -> usize {
        let mut current = 0usize;
        let mut base = 1usize;
        for digit in self.digits.iter().rev() {
            current += (*digit as usize) * base;
            base *= 10;
        }
        current
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone, Ord, PartialOrd)]
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
        let wires: Option<HashSet<Segment>> = s.chars().map(|c| Segment::parse(c)).collect();
        wires.map(|w| DigitWires { wires: w })
    }

    fn parse_line(s: &str) -> Option<Vec<DigitWires>> {
        s.split_whitespace().map(|chunk| Segment::parse_chunk(chunk)).collect()
    }
}

// static NORMAL_MAPPING: Lazy<HashMap<Digit, HashSet<Segment>>> = Lazy::new(|| hashmap! {
//     Digit::_0 => hashset! { Segment::A, Segment::B, Segment::C, Segment::E, Segment::F, Segment::G },
//     Digit::_1 => hashset! { Segment::C, Segment::F },
//     Digit::_2 => hashset! { Segment::A, Segment::C, Segment::D, Segment::E, Segment::G },
//     Digit::_3 => hashset! { Segment::A, Segment::C, Segment::D, Segment::F, Segment::G },
//     Digit::_4 => hashset! { Segment::B, Segment::C, Segment::D, Segment::F },
//     Digit::_5 => hashset! { Segment::A, Segment::B, Segment::D, Segment::F, Segment::G },
//     Digit::_6 => hashset! { Segment::A, Segment::B, Segment::D, Segment::E, Segment::F, Segment::G },
//     Digit::_7 => hashset! { Segment::A, Segment::C, Segment::F },
//     Digit::_8 => hashset! { Segment::A, Segment::B, Segment::C, Segment::D, Segment::E, Segment::F, Segment::G },
//     Digit::_9 => hashset! { Segment::A, Segment::B, Segment::C, Segment::D, Segment::F, Segment::G },
// });

#[derive(Debug)]
struct Line {
    input_patterns: Vec<DigitWires>,
    outputs: Vec<DigitWires>
}
impl Line {
    fn decode(&self) -> DecoderMap {
        let grouped =
            self.input_patterns.iter().into_group_map_by(|w| w.wires.len());

        let one = grouped.get(&2).unwrap().get(0).unwrap().wires.clone();
        let four = grouped.get(&4).unwrap().get(0).unwrap().wires.clone();
        let seven = grouped.get(&3).unwrap().get(0).unwrap().wires.clone();
        let eight = grouped.get(&7).unwrap().get(0).unwrap().wires.clone();
        // println!("1: {:?}", one);
        // println!("4: {:?}", four);
        // println!("7: {:?}", seven);
        // println!("8: {:?}", eight);

        let len_5_digits = grouped.get(&5).unwrap();
        let two_three_five1 = &len_5_digits.get(0).unwrap().wires;
        let two_three_five2 = &len_5_digits.get(1).unwrap().wires;
        let two_three_five3 = &len_5_digits.get(2).unwrap().wires;

        let six_or_nine_or_zero_digits = grouped.get(&6).unwrap();
        let six_or_nine_or_zero1 = &six_or_nine_or_zero_digits.get(0).unwrap().wires;
        let six_or_nine_or_zero2 = &six_or_nine_or_zero_digits.get(1).unwrap().wires;
        let six_or_nine_or_zero3 = &six_or_nine_or_zero_digits.get(2).unwrap().wires;
        // println!("6 | 9 | 0: 1st={:?}", six_or_nine_or_zero1);
        // println!("6 | 9 | 0: 2nd={:?}", six_or_nine_or_zero2);
        // println!("6 | 9 | 0: 3rd={:?}", six_or_nine_or_zero3);

        let is_six = |digit: &HashSet<Segment>|
            eight.difference(digit).any(|s| one.contains(s));
        let (six, nine_or_zero1, nine_or_zero2) =
            if is_six(six_or_nine_or_zero1) {
                (six_or_nine_or_zero1.clone(), six_or_nine_or_zero2, six_or_nine_or_zero3)
            }
            else if is_six(six_or_nine_or_zero2) {
                (six_or_nine_or_zero2.clone(), six_or_nine_or_zero1, six_or_nine_or_zero3)
            }
            else {
                (six_or_nine_or_zero3.clone(), six_or_nine_or_zero1, six_or_nine_or_zero2)
            };
        // println!("6: {:?}", six);

        let is_zero = |digit: &HashSet<Segment>|
            eight.difference(digit).any(|s| four.contains(s));
        let (nine, zero) =
            if is_zero(nine_or_zero1) { (nine_or_zero2.clone(), nine_or_zero1.clone()) }
            else { (nine_or_zero1.clone(), nine_or_zero2.clone()) };
        // println!("9: {:?}", nine);
        // println!("0: {:?}", zero);

        let six_or_nine_intersection: HashSet<Segment> =
            six.intersection(&nine).map(|s| s.clone()).collect();
        // println!("6 & 9: intersection={:?}", six_or_nine_intersection);
        let segment_e =
            eight.difference(&six_or_nine_intersection)
                .map(|s| s.clone())
                .collect::<HashSet<Segment>>()
                .difference(&one)
                .next().unwrap().clone();
        // println!("e: {:?}", segment_e);

        // let segment_a = seven.difference(&one).next().unwrap().clone();
        // println!("a: {:?}", segment_a);

        let segment_c = nine.difference(&six).next().unwrap().clone();
        // println!("c: {:?}", segment_c);

        // let segment_f =
        //     one.difference(&hashset! {segment_c}).next().unwrap().clone();
        // println!("f: {:?}", segment_f);

        let (two, three_five1, three_five2) =
            if two_three_five1.contains(&segment_e) {
                (two_three_five1.clone(), two_three_five2, two_three_five3)
            }
            else if two_three_five2.contains(&segment_e) {
                (two_three_five2.clone(), two_three_five1, two_three_five3)
            }
            else {
                (two_three_five3.clone(), two_three_five1, two_three_five2)
            };

        let (three, five) =
            if three_five1.contains(&segment_c) { (three_five1.clone(), three_five2.clone()) }
            else { (three_five2.clone(), three_five1.clone()) };

        // println!("0: {:?}", zero);
        // println!("1: {:?}", one);
        // println!("2: {:?}", two);
        // println!("3: {:?}", three);
        // println!("4: {:?}", four);
        // println!("5: {:?}", five);
        // println!("6: {:?}", six);
        // println!("7: {:?}", seven);
        // println!("8: {:?}", eight);
        // println!("9: {:?}", nine);

        let map = hashmap! {
            DigitWires::new(zero) => Digit::_0,
            DigitWires::new(one) => Digit::_1,
            DigitWires::new(two) => Digit::_2,
            DigitWires::new(three) => Digit::_3,
            DigitWires::new(four) => Digit::_4,
            DigitWires::new(five) => Digit::_5,
            DigitWires::new(six) => Digit::_6,
            DigitWires::new(seven) => Digit::_7,
            DigitWires::new(eight) => Digit::_8,
            DigitWires::new(nine) => Digit::_9,
        };
        DecoderMap { map }
    }
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

pub fn part2() {
    let lines = read();
    let result: usize = lines.iter().map(|line| {
        // println!("line: {:?}", line);

        let decode_map = line.decode();
        // println!("map: {:?}", decode_map);

        let decoded = decode_map.decode_iter(line.outputs.iter());
        // println!("decoded: {:?}={}", decoded, decoded.to_usize());
        decoded.to_usize()
    }).sum();
    println!("result={}", result);
}