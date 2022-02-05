use std::iter::{FlatMap, Map};
use std::ops::Range;
use itertools::Itertools;
use crate::read_lines;

struct HeightMap {
    data: Vec<Vec<u32>>
}
impl HeightMap {
    fn each_coord<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a {
        (0..self.data.len()).flat_map(|x|
            (0..self.data[x].len()).map(move |y| (x, y))
        )
    }

    fn get(&self, x: usize, y: usize) -> Option<u32> {
        self.data.get(x).and_then(|row| row.get(y).map(|v| *v))
    }

    fn is_low_point(&self, x: usize, y: usize) -> bool {
        let v = self.get(x, y).unwrap();
        let is_smaller_than = |maybe_v1: Option<u32>| match maybe_v1 {
            None => true,
            Some(v1) => v < v1
        };

        let smaller_than_upper = x == 0 || is_smaller_than(self.get(x - 1, y));
        let smaller_than_lower = is_smaller_than(self.get(x + 1, y));
        let smaller_than_left = y == 0 || is_smaller_than(self.get(x, y - 1));
        let smaller_than_right = is_smaller_than(self.get(x, y + 1));
        // println!(
        //     "({}, {}), upper: {}, lower: {}, left: {}, right: {}",
        //     x, y, smaller_than_upper, smaller_than_lower, smaller_than_left, smaller_than_right
        // );

        smaller_than_left && smaller_than_right && smaller_than_upper && smaller_than_lower
    }

    fn risk_level(&self, x: usize, y: usize) -> Option<u32> {
        if self.is_low_point(x, y) { self.get(x, y).map(|v| v + 1) } else { None }
    }
}

fn read() -> HeightMap {
    let data = read_lines("data/day9.txt").map(|line|
        line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec()
    ).collect_vec();
    HeightMap { data }
}

pub fn part1() {
    let map = read();
    // let low_points =
    //     map.each_coord().filter_map(|(x, y)|
    //         if map.is_low_point(x, y) { map.get(x, y).map(|v| ((x, y), v)) }
    //         else { None }
    //     ).collect_vec();
    // println!("low points: {:?}", low_points);
    let low_points: u32 =
        map.each_coord().filter_map(|(x, y)| map.risk_level(x, y)).sum();
    println!("result={}", low_points);
}
