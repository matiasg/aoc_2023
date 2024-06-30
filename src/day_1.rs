use regex::Regex;

fn first_and_last(line: &str) -> u64 {
    let (first, last) = get_regexes();
    let f = first.captures(line).unwrap().get(1).unwrap().as_str();
    let l = last.captures(line).unwrap().get(1).unwrap().as_str();
    f.parse::<u64>().unwrap() * 10 + l.parse::<u64>().unwrap()
}

fn get_regexes() -> (Regex, Regex) {
    let first = Regex::new("^[^0-9]*([0-9]).*$").unwrap();
    let last = Regex::new("^.*([0-9])[^0-9]*$").unwrap();
    (first, last)
}

fn get_number_letter_regexes() -> (Vec<Regex>, Vec<Regex>) {
    let numbers = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let letters = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let nregs: Vec<Regex> = numbers
        .iter()
        .map(|&number| Regex::new(format!("^{number}").as_str()).unwrap())
        .collect();
    let lregs: Vec<Regex> = letters
        .iter()
        .map(|&letters| Regex::new(format!("^{letters}").as_str()).unwrap())
        .collect();
    (nregs, lregs)
}
fn first_and_last_letters(line: &str) -> u64 {
    let (nregs, lregs) = get_number_letter_regexes();
    let mut first: Option<u64> = None;
    let mut last: Option<u64> = None;
    for idx in 0..line.len() {
        for (nstr, re) in nregs.iter().enumerate() {
            if re.is_match(&line[idx..]) {
                if first.is_none() {
                    first = Some(nstr as u64);
                }
                last = Some(nstr as u64);
                break;
            }
        }
        for (nstr, re) in lregs.iter().enumerate() {
            if re.is_match(&line[idx..]) {
                if first.is_none() {
                    first = Some(nstr as u64);
                }
                last = Some(nstr as u64);
                break;
            }
        }
    }
    assert!(first.is_some() && last.is_some());
    first.unwrap() * 10 + last.unwrap()
}

fn prob1(input: &Vec<&str>) -> u64 {
    input.iter().map(|&s| first_and_last(s)).sum()
}

fn prob2(input: &Vec<&str>) -> u64 {
    input.iter().map(|&s| first_and_last_letters(s)).sum()
}

pub fn main() {
    let input = include_str!("../day_1_input").trim().split('\n').collect();
    println!("prob 1: {}", prob1(&input));
    println!("prob 2: {}", prob2(&input));
}

#[cfg(test)]
mod tests {
    use super::{prob1, prob2};

    fn example1() -> Vec<&'static str> {
        vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
    }

    fn example2() -> Vec<&'static str> {
        vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ]
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(&example1()), 142);
    }

    #[test]
    fn test_prob2() {
        assert_eq!(prob2(&example2()), 281);
    }
}
