use regex::Regex;
use std::fs;
use std::iter;

fn parse(input: &Vec<&str>) -> Vec<(i64, i64)> {
    let line_re = Regex::new("^(Time|Distance):\\s*([0-9 ]+)$").unwrap();
    let spaces = Regex::new(" +").unwrap();
    let times: Vec<i64> = spaces
        .split(line_re.captures(input[0]).unwrap().get(2).unwrap().as_str())
        .map(|s| s.parse().unwrap())
        .collect();
    let dists: Vec<i64> = spaces
        .split(line_re.captures(input[1]).unwrap().get(2).unwrap().as_str())
        .map(|s| s.parse().unwrap())
        .collect();
    iter::zip(times, dists).collect()
}

fn how_many_more(td: &(i64, i64)) -> i64 {
    // x * (t - x) = d => x^2 - tx + d = 0
    let disc: f64 = ((td.0.pow(2) - 4 * td.1) as f64).sqrt();
    let mut x0 = (((td.0 as f64) - disc) / 2_f64).ceil() as i64;
    let mut x1 = (((td.0 as f64) + disc) / 2_f64).floor() as i64;
    if x0 * (td.0 - x0) == td.1 {
        x0 += 1;
    }
    if x1 * (td.0 - x1) == td.1 {
        x1 -= 1;
    }
    x1 - x0 + 1
}

fn prob1(input: &Vec<&str>) -> i64 {
    parse(input).iter().map(how_many_more).product()
}

fn prob2(input: &Vec<&str>) -> i64 {
    let spaces = Regex::new(" ").unwrap();
    let nospaces: Vec<String> = input
        .iter()
        .map(|l| spaces.replace_all(*l, "").to_string())
        .collect();
    let nospaces = vec![nospaces[0].as_str(), nospaces[1].as_str()];
    parse(&nospaces).iter().map(how_many_more).product()
}

pub fn main() {
    let input: String = fs::read_to_string("day_6_input").unwrap();
    let input: Vec<&str> = input.trim().split("\n").collect();
    println!("prob 1: {}", prob1(&input));
    println!("prob 2: {}", prob2(&input));
}

#[cfg(test)]
mod tests {
    use crate::day_6::{parse, prob1, prob2};

    fn small_input() -> Vec<&'static str> {
        vec!["Time:      7  15   30", "Distance:  9  40  200"]
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(&small_input()), 288);
    }

    #[test]
    fn test_prob2() {
        assert_eq!(prob2(&small_input()), 71503);
    }

    #[test]
    fn test_parse() {
        let expected: Vec<(i64, i64)> = vec![(7, 9), (15, 40), (30, 200)];
        assert_eq!(parse(&small_input()), expected);
    }
}
