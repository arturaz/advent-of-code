use core::fmt::{Display, Formatter};
use std::mem;
use std::ops::Add;
use crate::day5::Vec2;
use crate::day9::GridMap;
use crate::read_lines;

#[derive(Copy, Clone)]
struct Dot(bool);
impl Add for Dot {
    type Output = Dot;
    fn add(self, rhs: Self) -> Self::Output { Dot(self.0 || rhs.0) }
}
impl Display for Dot {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(if self.0 { "#" } else { "." })
    }
}

#[derive(Debug)]
enum FoldAlongAxis { Row, Col }
#[derive(Debug)]
struct FoldAlong { axis: FoldAlongAxis, index: usize }

fn fold(map: &mut GridMap<Dot>, along: &FoldAlong) {
    match along.axis {
        FoldAlongAxis::Row => {
            // let rows = map.data.len();
            let (to, from) =
                map.data.split_at_mut(along.index + 1);
            for from_row_idx in 0..from.len() {
                let to_row_idx = along.index - 1 - from_row_idx;
                // println!(
                //     "rows={}, along.index={}, to_row_idx={}, to.len={}, from_row_idx={}, from.len={}",
                //     rows, along.index, to_row_idx, to.len(), from_row_idx, from.len()
                // );
                let from_row_len = from[from_row_idx].len();
                for col_idx in 0..from_row_len {
                    let target = &mut to[to_row_idx][col_idx];
                    let source = &from[from_row_idx][col_idx];
                    let result = *target + *source;
                    // println!("[{}][{}]: {} -> [{}][{}] -> {}", from_row_idx, col_idx, source, to_row_idx, col_idx, result);
                    *target = result;
                }
            }

            map.data.resize_with(along.index, || panic!("should not be invoked"));
        }
        FoldAlongAxis::Col => {
            for row in map.data.iter_mut() {
                let (to, from) = row.split_at_mut(along.index + 1);
                for from_idx in 0..from.len() {
                    let to_idx = along.index - 1 - from_idx;
                    let source = from[from_idx];
                    let target = &mut to[to_idx];
                    *target = *target + source;
                }

                row.resize_with(along.index, || panic!("should not be invoked"));
            }
        }
    }
}

fn read() -> (GridMap<Dot>, Vec<FoldAlong>) {
    let mut iter = read_lines("data/day13.txt");
    let mut reading_coords = true;
    let mut map = GridMap::<Dot>::new();
    let mut fold_along = Vec::<FoldAlong>::new();

    while let Some(line) = iter.next() {
        if line.is_empty() {
            reading_coords = false;
        }
        else if reading_coords {
            let (col_str, row_str) = line.split_once(",").unwrap();
            let col = col_str.parse::<usize>().unwrap();
            let row = row_str.parse::<usize>().unwrap();
            let c = Vec2::new(row, col);
            // println!("{}", c);
            map.ensure_indexes(&c, &Dot(false));
            *map.get_mut(&c).unwrap() = Dot(true);
        }
        else {
            let line = line.get("fold along ".len()..line.len()).unwrap();
            let (axis_str, index_str) = line.split_once("=").unwrap();
            let axis = match axis_str {
                "y" => FoldAlongAxis::Row,
                "x" => FoldAlongAxis::Col,
                _ => panic!("Unknown axis: {}", axis_str)
            };
            let index = index_str.parse::<usize>().unwrap();
            let instruction = FoldAlong { axis, index };
            fold_along.push(instruction);
        }
    }

    (map, fold_along)
}

pub fn part1() {
    let (mut map, fold_alongs) = read();
    println!("{}", map);
    println!("{:?}", fold_alongs);
    for fold_along in &fold_alongs {
        fold(&mut map, fold_along);
        println!("{}", map);
        let dots = map.each_point().filter(|p| p.value.0).count();
        println!("dots={}", dots);
    }
}