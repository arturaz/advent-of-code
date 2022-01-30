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

struct WinningBoard {
    board: Board,
    numbers: HashSet<u32>,
    won_at_num: u32
}

fn filter_winning(mut input: Input) -> Vec<WinningBoard> {
    let mut marked = HashSet::<u32>::new();
    let mut winning_boards = Vec::<WinningBoard>::new();

    for num in &input.numbers {
        marked.insert(*num);
        let winning_this_number =
            input.boards.drain_filter(|board| board.check(&marked));

        for board in winning_this_number {
            winning_boards.push(WinningBoard {
                board, numbers: marked.clone(), won_at_num: *num
            });
        }
    }

    winning_boards
}

pub fn part1() {
    let input = read();
    let (board, marked, num) = determine_winning(&input).unwrap();
    let score = board.sum_of_unmarked(&marked) * num;
    println!("score={}", score)
}

pub fn part2() {
    let input = read();
    let winning_boards = filter_winning(input);
    let winning_board = winning_boards.last().unwrap();
    let score = winning_board.board.sum_of_unmarked(&winning_board.numbers) * winning_board.won_at_num;
    println!("score={}", score)
}