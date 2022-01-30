use crate::read_lines;

fn read() -> Vec<i32> {
    read_lines("data/day1.txt")
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

/// https://adventofcode.com/2021/day/1
pub fn part1() {
    let lines = read();
    let tupled = lines.iter().zip(lines.iter().skip(1));
    let increased =
        tupled.filter(|(v1, v2)| v2 > v1).count();
    println!("{}", increased)
}

// https://adventofcode.com/2021/day/1#part2
pub fn part2() {
    let lines = read();
    let window_values =
        lines.iter().zip(lines.iter().skip(1)).zip(lines.iter().skip(2))
            .map(|((a, b), c)| a + b + c)
            .collect::<Vec<_>>();
    let tupled =
        window_values.iter().zip(window_values.iter().skip(1));
    let increased =
        tupled.filter(|(v1, v2)| v2 > v1).count();
    println!("{}", increased)
}