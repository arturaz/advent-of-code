use core::fmt::Display;
use std::collections::HashMap;
use std::ops::Add;
use itertools::Itertools;
use ndarray::{Array2, ArrayBase, OwnedRepr};
use petgraph::{Directed, Graph};
use petgraph::algo::dijkstra;
use petgraph::graph::NodeIndex;
use crate::day5::Vec2;
use crate::read_lines;

type MyGraph = Graph<Vec2, u32, Directed>;

fn render_arr<A : Display>(arr: &Array2<A>) -> String {
    let mut s = String::new();
    let dim: (usize, usize) = arr.dim();
    for row in 0..dim.0 {
        for col in 0..dim.1 {
            let value = arr.get((row, col)).unwrap();
            s.push_str(format!("{}", value).as_str());
        }
        s.push('\n');
    }
    s
}

fn read_raw() -> Array2<u32> {
    let lines = read_lines("data/day15.txt").collect_vec();
    let rows = lines.len();
    let cols = lines[0].len();
    let mut array = Array2::zeros((rows, cols));
    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            *array.get_mut((row, col)).unwrap() = c;
        }
    }
    array
}

fn add_wrap_around(v: u32, add: u32) -> u32 {
    let mut new = v + add;
    if new > 9 { new = new % 9; }
    new
}

fn repeat(raw: &Array2<u32>, times: usize) -> Array2<u32> {
    let dim: (usize, usize) = raw.dim();
    let mut array = Array2::zeros((dim.0 * times, dim.1 * times));

    let points = (0..dim.0).flat_map(|row|
        (0..dim.1).map(move |col| Vec2::new(row, col))
    ).collect_vec();

    for row_repetition in 0..times {
        for col_repetition in 0..times {
            for p in &points {
                let orig_value = *raw.get(p.as_tuple()).unwrap();
                let value = add_wrap_around(orig_value, (row_repetition + col_repetition) as u32);
                let target_coord = p.add_xy(dim.0 * row_repetition, dim.1 * col_repetition);
                // println!(
                //     "rep {},{}, {:?}: {} -> {}",
                //     row_repetition, col_repetition, target_coord, orig_value, value
                // );
                *array.get_mut(target_coord.as_tuple()).unwrap() = value;
            }
        }
    }
    array
}

fn into_graph(raw: &Array2<u32>) -> (MyGraph, HashMap<Vec2, NodeIndex>, Vec2) {
    let rows = raw.dim().0;
    let cols = raw.dim().1;
    let mut graph = MyGraph::new();
    // Make this eager.
    let map = (0..rows).flat_map(|row|
        (0..cols).map(move |col| Vec2::new(row, col))
    ).map(|c| (c, graph.add_node(c.clone()))).collect::<HashMap<_, _>>();

    for (c, node_idx) in &map {
        for c1 in c.adjacent(false) {
            if let Some(node_idx1) = map.get(&c1) {
                let edge_weight = *raw.get((c1.x, c1.y)).unwrap();
                graph.update_edge(node_idx.clone(), node_idx1.clone(), edge_weight);
            }
        }
    }
    let max_coord = Vec2::new(rows - 1, cols - 1);

    (graph, map, max_coord)
}

pub fn part1() {
    let (graph, map, max_coord) = into_graph(&read_raw());
    // println!("graph={:?}", graph);
    // println!("map={:?}", map);
    println!("max_coord={}", max_coord);
    let start = map.get(&Vec2::new(0, 0)).unwrap();
    let end = map.get(&max_coord).unwrap();
    println!("start={:?}, end={:?}", start, end);
    let results =
        dijkstra(&graph, *start, Some(*end), |e| *e.weight());
    let result = results[end];
    println!("result={:?}", result);
}

pub fn part2() {
    let raw = read_raw();
    let raw = repeat(&raw, 5);
    println!("raw=\n{}", render_arr(&raw));
    let (graph, map, max_coord) = into_graph(&raw);
    // println!("graph={:?}", graph);
    // println!("map={:?}", map);
    println!("max_coord={}", max_coord);
    let start = map.get(&Vec2::new(0, 0)).unwrap();
    let end = map.get(&max_coord).unwrap();
    println!("start={:?}, end={:?}", start, end);
    let results =
        dijkstra(&graph, *start, Some(*end), |e| *e.weight());
    let result = results[end];
    println!("result={:?}", result);
}