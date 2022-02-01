use std::collections::HashMap;
use std::ops::{Add, Sub};
use itertools::Itertools;
use crate::{arithmetic_sum, read_num_line};

type Position = usize;
type Count = usize;
type Fuel = usize;
type State = HashMap<Position, Count>;

fn read() -> State {
    let positions = read_num_line::<Position>("data/day7.txt");
    let mut state = State::new();
    for position in positions {
        *state.entry(position).or_default() += 1;
    }
    state
}

fn fuel_for(state: &State, target_position: &Position, fuel_fn: impl Fn(usize) -> Fuel) -> Fuel {
    let mut fuel: Fuel = 0;
    for (position, count) in state {
        let diff = target_position.abs_diff(*position);
        fuel += fuel_fn(diff) * count;
    }
    fuel
}

fn run(fuel_fn: impl Fn(usize) -> Fuel) {
    let state = read();
    let min = *state.keys().min().unwrap();
    let max = *state.keys().max().unwrap();

    let mut fuels_at_positions = HashMap::<Position, Fuel>::new();
    for position in min..=max {
        fuels_at_positions.insert(position, fuel_for(&state, &position, &fuel_fn));
    }

    println!("fuels_at_positions={:?}", fuels_at_positions);

    let (min_pos, min_fuel) = fuels_at_positions.iter()
        .min_by_key(|(_, fuel)| **fuel)
        .unwrap();
    println!("min_pos={}, min_fuel={}", min_pos, min_fuel);
}

pub fn part1() {
    run(|diff| diff)
}

pub fn part2() {
    run(|diff| {
        if diff == 0 { 0 }
        else {
            match arithmetic_sum(1, diff + 1) {
                Some(v) => v,
                None => panic!("Can't calc arithmetic_sum from 1..={}", diff)
            }
        }
    })
}