use std::collections::HashMap;
use itertools::{Chunk, Itertools};
use crate::day10::ChunkKind::{Bracket, Curly, Fish, Parenthesis};
use crate::read_lines;

#[derive(Copy, Clone, Eq, PartialEq)]
enum ChunkKind { Curly, Parenthesis, Bracket, Fish }
impl ChunkKind {
    fn syntax_points(&self) -> usize {
        match self {
            Parenthesis => 3,
            Bracket => 57,
            Curly => 1197,
            Fish => 25137
        }
    }

    fn autocomplete_points(&self) -> usize {
        match self {
            Parenthesis => 1,
            Bracket => 2,
            Curly => 3,
            Fish => 4
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

    fn to_char(&self) -> char {
        match self {
            Symbol { kind: Curly, opening: true } => '{',
            Symbol { kind: Curly, opening: false } => '}',
            Symbol { kind: Parenthesis, opening: true } => '(',
            Symbol { kind: Parenthesis, opening: false } => ')',
            Symbol { kind: Bracket, opening: true } => '[',
            Symbol { kind: Bracket, opening: false } => ']',
            Symbol { kind: Fish, opening: true } => '<',
            Symbol { kind: Fish, opening: false } => '>'
        }
    }
}

struct InputLine {
    symbols: Vec<Symbol>
}
impl InputLine {
    fn find_first_incorrect(&self) -> Option<ChunkKind> {
        self.autocomplete().err()
    }

    fn autocomplete(&self) -> Result<usize, ChunkKind> {
        let mut open_stack = Vec::<ChunkKind>::new();
        let mut score = 0usize;

        for symbol in &self.symbols {
            if symbol.opening {
                open_stack.push(symbol.kind);
            }
            else {
                let matches =
                    open_stack.pop()
                        .is_some_with(|last_opened| *last_opened == symbol.kind);
                if !matches { return Err(symbol.kind) }
            }
        }

        for open in open_stack.iter().rev() {
            // print!("{}", Symbol { kind: *open, opening: false}.to_char());
            score = score * 5 + open.autocomplete_points();
        }
        // println!(" = {}", score);

        Ok(score)
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
    let result: usize = wrong.iter().map(|k| k.syntax_points()).sum();
    println!("result={}", result);
}

pub fn part2() {
    let lines = read();
    let autocompleted: Vec<usize> =
        lines.flat_map(|line| line.autocomplete().ok()).collect();
    let sorted = autocompleted.iter().sorted().collect_vec();
    // println!("sorted={:?}", sorted);
    let result = sorted[sorted.len() / 2];
    println!("result={}", result);
}