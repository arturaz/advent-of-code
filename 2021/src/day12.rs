use alloc::rc::Rc;
use core::fmt::{Debug, Display, Formatter};
use im_rc::{HashMap, HashSet, Vector};
use crate::read_lines;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Node {
    name: String,
    is_large_cave: bool
}
impl Node {
    fn from(s: &str) -> Self {
        Node {
            name: String::from(s),
            is_large_cave: s == s.to_uppercase()
        }
    }
}

#[derive(Clone)]
struct Path {
    ordered: Vector<Rc<Node>>,
    visited: HashSet<Rc<Node>>
}
impl Path {
    fn new() -> Self {
        Path { ordered: Vector::new(), visited: HashSet::new() }
    }

    fn add(&mut self, n: &Rc<Node>) {
        self.ordered.push_back(n.clone());
        self.visited.insert(n.clone());
    }

    fn has_visited(&self, n: &Node) -> bool {
        self.visited.contains(n)
    }

    fn start(&self) -> Option<&Rc<Node>> { self.ordered.head() }
}
impl Debug for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str("Path { ")?;
        let mut first = true;
        for node in &self.ordered {
            if first { first = false; } else { f.write_str(",")?; }
            f.write_fmt(format_args!("{}", node.name))?;
        }
        f.write_str(" }")
    }
}

#[derive(Clone)]
enum AllowVisitingSmallCaveTwice {
    Disallow,
    Allow { visited: Option<Rc<Node>> }
}
impl Display for AllowVisitingSmallCaveTwice {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            AllowVisitingSmallCaveTwice::Disallow => f.write_str("-"),
            AllowVisitingSmallCaveTwice::Allow { visited: None } => f.write_str("+ [ ]"),
            AllowVisitingSmallCaveTwice::Allow { visited: Some(node) } =>
                f.write_fmt(format_args!("+ [{}]", node.name))
        }
    }
}
impl AllowVisitingSmallCaveTwice {
    pub(crate) fn with(&self, node: Rc<Node>) -> AllowVisitingSmallCaveTwice {
        AllowVisitingSmallCaveTwice::Allow { visited: Some(node) }
    }
}

struct Graph {
    by_name: HashMap<String, Rc<Node>>,
    data: HashMap<Rc<Node>, HashSet<Rc<Node>>>
}
impl Graph {
    pub(crate) fn get(&self, name: &str) -> Option<&Rc<Node>> {
        self.by_name.get(name)
    }

    fn new() -> Self { Graph { by_name: HashMap::new(), data: HashMap::new() } }

    fn insert(&mut self, from: &str, to: &str) {
        let from = Rc::new(Node::from(from));
        let to = Rc::new(Node::from(to));

        self.by_name.insert(String::from(&from.name), from.clone());
        self.by_name.insert(String::from(&to.name), to.clone());

        self.data.entry(from).or_default().insert(to.clone());
    }

    fn all_paths(
        &self,
        // Path until the current_node, excluding the node.
        current_path: &Path,
        current_node: &Rc<Node>,
        target_node: &Rc<Node>,
        allow_visiting: AllowVisitingSmallCaveTwice
    ) -> Vector<Path> {
        let mut paths = Vector::<Path>::new();

        let mut next_path = current_path.clone();
        next_path.add(current_node);

        // println!("all_paths: {}, {:?}", allow_visiting, next_path);

        let mut arrived = || {
            // println!("No where to go, path={:?}", &next_path);
            if current_node == target_node {
                paths.push_back(next_path.clone());
            }
        };

        if current_node == target_node {
            arrived();
        }
        else {
            let maybe_possible_set =
                self.data.get(current_node).filter(|s| !s.is_empty());
            match maybe_possible_set {
                None => arrived(),
                Some(possible) => {
                    for next in possible.iter() {
                        if current_path.start().contains(&next) {
                            // println!("Not going into the start node '{}'", next.name);
                            continue;
                        }

                        let mut recurse = |allow_visiting: AllowVisitingSmallCaveTwice| {
                            // println!("all_paths: next: {:?}", next.name);
                            let other_paths = self.all_paths(
                                &next_path, next, target_node,
                                allow_visiting
                            );
                            paths.append(other_paths);
                        };

                        if next.is_large_cave {
                            recurse(allow_visiting.clone());
                        }
                        else {
                            let has_visited_next = current_path.has_visited(next);
                            match allow_visiting {
                                AllowVisitingSmallCaveTwice::Disallow
                                | AllowVisitingSmallCaveTwice::Allow { visited: Some(_) } => {
                                    if !has_visited_next { recurse(allow_visiting.clone()); }
                                }
                                AllowVisitingSmallCaveTwice::Allow { visited: None } => {
                                    let new_allow =
                                        if has_visited_next { allow_visiting.with(next.clone()) }
                                        else { allow_visiting.clone() };
                                    recurse(new_allow);
                                }
                            }
                        }
                    }
                }
            }
        }

        paths
    }
}

fn read() -> Graph {
    let mut graph = Graph::new();
    for line in read_lines("data/day12.txt") {
        let (from, to) = line.split_once("-").unwrap();
        graph.insert(from, to);
        graph.insert(to, from);
    }
    graph
}

pub fn part1() {
    let graph = read();
    let start = graph.get("start").unwrap();
    let end = graph.get("end").unwrap();
    let paths = graph.all_paths(
        &Path::new(), start, end,
        AllowVisitingSmallCaveTwice::Disallow
    );

    println!("Result ({}):", paths.len());
    for path in paths {
        println!("{:?}", path);
    }
}

pub fn part2() {
    let graph = read();
    let start = graph.get("start").unwrap();
    let end = graph.get("end").unwrap();
    let paths = graph.all_paths(
        &Path::new(), start, end,
        AllowVisitingSmallCaveTwice::Allow { visited: None }
    );

    println!("Result ({}):", paths.len());
    for path in paths {
        println!("{:?}", path);
    }
}