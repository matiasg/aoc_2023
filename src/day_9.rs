use std::fs;

fn add_at_the_end(a: i32, v: &Vec<i32>) -> i32 {
    a + v.last().unwrap()
}

fn subtract_at_the_beginning(a: i32, v: &Vec<i32>) -> i32 {
    v.first().unwrap() - a
}

fn one_line(line: &str, folder: impl Fn(i32, &Vec<i32>) -> i32) -> i32 {
    let current: Vec<i32> = line.split(' ').map(|i| i.parse().unwrap()).collect();
    let lines = get_lines_until_0(current);
    lines.iter().rev().fold(0, |a, v| folder(a, v))
}

fn get_lines_until_0(current: Vec<i32>) -> Vec<Vec<i32>> {
    let mut lines: Vec<Vec<i32>> = vec![current];
    while lines.last().unwrap().iter().any(|&i| i != 0) {
        let ll = lines[lines.len() - 1].clone();
        let newline: Vec<i32> = ll
            .iter()
            .zip(ll.get(1..).unwrap().iter())
            .map(|(&a, &b)| b - a)
            .collect();
        lines.push(newline);
    }
    lines
}

fn prob1(input: Vec<&str>) -> i32 {
    input.iter().map(|l| one_line(l, add_at_the_end)).sum()
}

fn prob2(input: Vec<&str>) -> i32 {
    input
        .iter()
        .map(|l| one_line(l, subtract_at_the_beginning))
        .sum()
}

pub fn main() {
    let input = fs::read_to_string("day_9_input").expect("no input file");
    let input: Vec<&str> = input.trim().split("\n").collect();
    println!("prob1: {}", prob1(input.clone()));
    println!("prob2: {}", prob2(input.clone()));
}

#[cfg(test)]
mod tests {
    use crate::day_9::{add_at_the_end, one_line, prob1, prob2, subtract_at_the_beginning};

    fn example() -> Vec<&'static str> {
        vec!["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"]
    }

    #[test]
    fn test_one_line() {
        assert_eq!(one_line(example()[0], add_at_the_end), 18);
        assert_eq!(one_line(example()[1], add_at_the_end), 28);
        assert_eq!(one_line(example()[2], add_at_the_end), 68);
        assert_eq!(one_line(example()[0], subtract_at_the_beginning), -3);
        assert_eq!(one_line(example()[1], subtract_at_the_beginning), 0);
        assert_eq!(one_line(example()[2], subtract_at_the_beginning), 5);
    }

    #[test]
    fn test_probs() {
        assert_eq!(prob1(example()), 114);
        assert_eq!(prob2(example()), 2);
    }
}
