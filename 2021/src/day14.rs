use std::collections::HashMap;
use std::iter::repeat;
use itertools::Itertools;
use crate::read_lines;

type Pair = (char, char);
type Rules = HashMap<Pair, char>;
type Polymer = Vec<char>;

fn read() -> (Polymer, Rules) {
    let mut iter = read_lines("data/day14.txt");
    let polymer = iter.next().unwrap().chars().collect_vec();
    iter.next().unwrap();
    let rules =
        Rules::from_iter(iter.map(|line| {
            let mut line = line.chars();
            let pair = (line.nth(0).unwrap(), line.nth(0).unwrap());
            let to = line.nth(4).unwrap();
            (pair, to)
        }));
    (polymer, rules)
}

fn pair_at(p: &Polymer, idx: usize) -> (char, char) {
    (*p.get(idx).unwrap(), *p.get(idx + 1).unwrap())
}

fn grow(p: &mut Polymer, rules: &Rules) {
    let mut idx = 0usize;
    while idx < p.len() - 1 {
        let pair = pair_at(p, idx);
        match rules.get(&pair) {
            None => {
                idx += 1;
            }
            Some(to_insert) => {
                p.insert(idx + 1, *to_insert);
                idx += 2;
            }
        }
    }
}

pub fn part1() {
    let (mut polymer, rules) = read();

    println!("initial: {}", polymer.iter().join(""));
    for step in 0..10 {
        grow(&mut polymer, &rules);
        println!("step {}: {}", step, polymer.iter().join(""));
    }

    let counts = polymer.iter().counts_by(|c| c);
    println!("counts: {:?}", counts);

    let min_count = counts.iter().min_by_key(|(_, count)| *count).unwrap();
    let max_count = counts.iter().max_by_key(|(_, count)| *count).unwrap();
    let result = max_count.1 - min_count.1;
    println!("min={:?}, max={:?}, result={}", min_count, max_count, result);
}