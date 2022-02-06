use std::collections::HashMap;
use itertools::{Chunk, Itertools};
use crate::day10::ChunkKind::{Bracket, Curly, Fish, Parenthesis};
use crate::read_lines;

#[derive(Copy, Clone, Eq, PartialEq)]
enum ChunkKind { Curly, Parenthesis, Bracket, Fish }
impl ChunkKind {
    fn points(&self) -> usize {
        match self {
            Parenthesis => 3,
            Bracket => 57,
            Curly => 1197,
            Fish => 25137
        }
    }
}

struct Symbol {
    kind: ChunkKind,
    opening: bool
}
impl Symbol {
    fn parse(c: char) -> Option<Symbol> {
        match c {
            '{' => Some(Symbol { kind: Curly, opening: true }),
            '}' => Some(Symbol { kind: Curly, opening: false }),
            '(' => Some(Symbol { kind: Parenthesis, opening: true }),
            ')' => Some(Symbol { kind: Parenthesis, opening: false }),
            '[' => Some(Symbol { kind: Bracket, opening: true }),
            ']' => Some(Symbol { kind: Bracket, opening: false }),
            '<' => Some(Symbol { kind: Fish, opening: true }),
            '>' => Some(Symbol { kind: Fish, opening: false }),
            _ => None
        }
    }
}

struct InputLine {
    symbols: Vec<Symbol>
}
impl InputLine {
    fn find_first_incorrect(&self) -> Option<ChunkKind> {
        let mut open_stack = Vec::<ChunkKind>::new();

        for symbol in &self.symbols {
            if symbol.opening {
                open_stack.push(symbol.kind);
            }
            else {
                let matches =
                    open_stack.pop()
                        .is_some_with(|last_opened| *last_opened == symbol.kind);
                if !matches { return Some(symbol.kind) }
            }
        }

        None
    }
}

fn read() -> impl Iterator<Item = InputLine> {
    read_lines("data/day10.txt").map(|line| {
        let symbols =
            line.chars().map(|c| Symbol::parse(c).unwrap()).collect_vec();
        InputLine { symbols }
    })
}

pub fn part1() {
    let lines = read();
    let wrong: Vec<ChunkKind> = lines.flat_map(|line| line.find_first_incorrect()).collect();
    let result: usize = wrong.iter().map(|k| k.points()).sum();
    println!("result={}", result);
}