use std::collections::HashMap;
use std::iter::repeat;
use itertools::Itertools;
use crate::read_lines;

type Pair = (char, char);
type Rules = HashMap<Pair, char>;

#[derive(Debug)]
struct Polymer {
    pairs: HashMap<Pair, usize>,
    counts: HashMap<char, usize>
}
impl Polymer {
    fn from_str(s: &str) -> Self {
        let counts = s.chars().counts_by(|c| c);
        let pairs = s.chars().tuple_windows::<(_, _)>().counts_by(|pair| pair);
        Self { pairs, counts }
    }

    fn grow(&mut self, rules: &Rules) {
        let mut new_pairs = HashMap::<Pair, usize>::new();
        for (pair, count) in &self.pairs {
            match rules.get(&pair) {
                None => {
                    *new_pairs.entry(*pair).or_default() += count;
                }
                Some(to_insert) => {
                    let new_pair_1 = (pair.0, *to_insert);
                    let new_pair_2 = (*to_insert, pair.1);

                    *new_pairs.entry(new_pair_1).or_default() += count;
                    *new_pairs.entry(new_pair_2).or_default() += count;
                    *self.counts.entry(*to_insert).or_default() += count;
                }
            }
        }

        self.pairs = new_pairs;
    }
}

fn read() -> (Polymer, Rules) {
    let mut iter = read_lines("data/day14.txt");
    let polymer = Polymer::from_str(&iter.next().unwrap());
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

pub fn run(iters: usize) {
    let (mut polymer, rules) = read();

    println!("initial: {:?}", polymer);
    for step in 1..=iters {
        polymer.grow(&rules);
        println!("step {}: {:?}", step, polymer);
    }

    println!("counts: {:?}", polymer.counts);

    let min_count = polymer.counts.iter().min_by_key(|(_, count)| *count).unwrap();
    let max_count = polymer.counts.iter().max_by_key(|(_, count)| *count).unwrap();
    let result = max_count.1 - min_count.1;
    println!("min={:?}, max={:?}, result={}", min_count, max_count, result);
}

pub fn part1() {
    run(10);
}

pub fn part2() {
    run(40);
}