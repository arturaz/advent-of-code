use std::collections::HashMap;
use itertools::Itertools;
use crate::{read_lines, read_num_line};

type DaysTillBirth = u8;
type State = HashMap<DaysTillBirth, usize>;

fn read() -> State {
    let numbers = read_num_line("data/day6.txt");

    let mut map = State::new();
    for days_till_birth in numbers {
        *map.entry(days_till_birth).or_insert(0) += 1;
    }

    map
}

fn iterate(state: &State) -> State {
    let mut new_state = State::new();

    for (days_till_birth, count) in state {
        if *days_till_birth == 0 {
            *new_state.entry(6).or_insert(0) += *count;
            *new_state.entry(8).or_insert(0) += *count;
        }
        else {
            *new_state.entry(days_till_birth - 1).or_insert(0) += *count;
        }
    }

    new_state
}

fn render(state: &State) -> String {
    state.keys().sorted()
        .map(|key| format!("{}={}", key, state.get(key).unwrap()))
        .join(",")
}

fn state_size(state: &State) -> usize {
    state.values().sum()
}

fn run(iterations: usize) {
    let mut state = read();
    println!("Initial: {}", render(&state));
    for iteration in 0..iterations {
        state = iterate(&state);
        println!("Iteration={}, state size={}", iteration, state_size(&state));
        // println!("After {} days: {}", iteration + 1, render(&state));
    }

    println!("total={}", state_size(&state));
}

pub fn part1() {
    run(80);
}

pub fn part2() {
    run(256);
}