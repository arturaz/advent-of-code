use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Lines};
use std::iter::Map;
use crate::read_lines;

#[derive(Debug)]
struct Board {
    numbers: Vec<Vec<u32>>
}
impl Board {
    fn check_row(&self, idx: usize, marked: &HashSet<u32>) -> bool {
        self.numbers[idx].iter().all(|n| marked.contains(n))
    }

    fn check_col(&self, idx: usize, marked: &HashSet<u32>) -> bool {
        self.numbers.iter().all(|row| marked.contains(&row[idx]))
    }

    fn check(&self, marked: &HashSet<u32>) -> bool {
        (0..5).any(|idx| self.check_row(idx, marked)) ||
            (0..5).any(|idx| self.check_col(idx, marked))
    }

    fn sum_of_unmarked(&self, marked: &HashSet<u32>) -> u32 {
        self.numbers.iter().map(|row| {
            let x: u32 = row.iter().filter(|n| !marked.contains(n)).sum();
            x
        }).sum()
    }
}

struct Input {
    numbers: Vec<u32>,
    boards: Vec<Board>
}

fn read_board(lines: &mut Map<Lines<BufReader<File>>, fn(std::io::Result<String>) -> String>) -> Board {
    let mut board = Vec::<Vec<u32>>::new();
    for _ in 0..5 {
        let line =
            lines.next().unwrap().split_whitespace()
                .map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();
        board.push(line);
    }
    Board { numbers: board }
}

fn read() -> Input {
    let mut lines = read_lines("data/day4.txt");
    let numbers =
        lines.next().unwrap().split(",")
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
    let mut boards = Vec::<Board>::new();
    while lines.next().is_some() {
        boards.push(read_board(&mut lines));
    }
    Input { numbers, boards }
}

fn determine_winning(input: &Input) -> Option<(&Board, HashSet<u32>, u32)> {
    let mut marked = HashSet::<u32>::new();
    for num in &input.numbers {
        marked.insert(*num);
        for board in &input.boards {
            if board.check(&marked) {
                return Some((&board, marked, *num))
            }
        }
    }

    None
}

pub fn part1() {
    let input = read();
    let (board, marked, num) = determine_winning(&input).unwrap();
    let score = board.sum_of_unmarked(&marked) * num;
    println!("score={}", score)
}