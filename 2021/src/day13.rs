use core::fmt::{Display, Formatter};
use crate::day5::Vec2;
use crate::day9::GridMap;
use crate::read_lines;

#[derive(Copy, Clone)]
struct Dot(bool);
impl Display for Dot {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(if self.0 { "#" } else { "." })
    }
}

#[derive(Debug)]
enum FoldAlongAxis { Row, Col }
#[derive(Debug)]
struct FoldAlong { axis: FoldAlongAxis, index: usize }

fn read() -> (GridMap<Dot>, Vec<FoldAlong>) {
    let mut iter = read_lines("data/day13_test.txt");
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
    let (map, fold_along) = read();
    println!("{}", map);
    println!("{:?}", fold_along);
}