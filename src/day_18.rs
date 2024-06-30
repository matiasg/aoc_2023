use regex::Regex;
use std::i64;

fn dir_to_vec(dir: &str) -> (i64, i64) {
    match dir {
        "R" => (0, 1),
        "L" => (0, -1),
        "U" => (-1, 0),
        "D" => (1, 0),
        _ => (0, 0),
    }
}

fn dir_to_dir_prob2(dir: char) -> &'static str {
    match dir {
        '0' => "R",
        '1' => "D",
        '2' => "L",
        '3' => "U",
        _ => "0",
    }
}

fn convert_to_vectors(input: &[&str]) -> Vec<(i64, i64)> {
    // coordinates start at (0,0) and are (i,j), i positive downwards, j positive rightwards
    let re = Regex::new(r"([RDLU]) (\d+) ").unwrap();
    let lines: Vec<(&str, i64)> = input
        .iter()
        .map(|s| re.captures(s).unwrap())
        .map(|c| {
            (
                c.get(1).unwrap().as_str(),
                c.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            )
        })
        .collect();
    let mut result: Vec<(i64, i64)> = vec![(0, 0)];
    let mut current = (0i64, 0i64);
    for line in lines {
        let v = dir_to_vec(line.0);
        current = (current.0 + v.0 * line.1, current.1 + v.1 * line.1);
        result.push(current);
    }
    result
}

fn prob1(input: &[&str]) -> u64 {
    let vectors = convert_to_vectors(input);
    get_area(vectors)
}

fn prob2(input: &[&str]) -> u64 {
    let vectors = convert_to_vectors_prob2(input);
    get_area(vectors)
}

fn convert_to_vectors_prob2(input: &[&str]) -> Vec<(i64, i64)> {
    let re = Regex::new(r"[RDLU] \d+ \(#([0-9a-f]{5})([0-9a-f])\)").unwrap();
    let lines: Vec<(&str, i64)> = input
        .iter()
        .map(|s| re.captures(s).unwrap())
        .map(|c| {
            (
                dir_to_dir_prob2(c.get(2).unwrap().as_str().chars().nth(0).unwrap()),
                i64::from_str_radix(c.get(1).unwrap().as_str(), 16).unwrap(),
            )
        })
        .collect();
    let mut result: Vec<(i64, i64)> = vec![(0, 0)];
    let mut current = (0i64, 0i64);
    for line in lines {
        let v = dir_to_vec(line.0);
        current = (current.0 + v.0 * line.1, current.1 + v.1 * line.1);
        result.push(current);
    }
    result
}

fn get_area(vectors: Vec<(i64, i64)>) -> u64 {
    let mut result = 0i64;
    for (v, w) in vectors.iter().zip(vectors[1..].iter()) {
        let diff = (w.0 - v.0, w.1 - v.1);
        result += if diff.0 == 0 {
            -diff.1 * v.0 + diff.1.abs()
        } else {
            diff.0 * v.1 + diff.0.abs()
        };
    }
    result as u64 / 2 + 1
}

pub fn main() {
    let input: Vec<&str> = include_str!("../day_18_input").trim().lines().collect();
    println!("prob 1: {}", prob1(&input));
    println!("prob 2: {}", prob2(&input));
}

#[cfg(test)]
mod tests {
    use crate::day_18::{convert_to_vectors, prob1, prob2};

    fn example() -> Vec<&'static str> {
        vec![
            "R 6 (#70c710)",
            "D 5 (#0dc571)",
            "L 2 (#5713f0)",
            "D 2 (#d2c081)",
            "R 2 (#59c680)",
            "D 2 (#411b91)",
            "L 5 (#8ceee2)",
            "U 2 (#caa173)",
            "L 1 (#1b58a2)",
            "U 2 (#caa171)",
            "R 2 (#7807d2)",
            "U 3 (#a77fa3)",
            "L 2 (#015232)",
            "U 2 (#7a21e3)",
        ]
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(&example()), 62);
    }

    #[test]
    fn test_prob2() {
        assert_eq!(prob2(&example()), 952408144115);
    }

    #[test]
    fn test_converts() {
        let vectors = convert_to_vectors(&example());
        assert_eq!(vectors[1], (0, 6));
        assert_eq!(vectors[2], (5, 6));
        assert_eq!(vectors.len(), 15);
        assert_eq!(vectors[14], (0, 0));
    }
}
