fn numbers_in_line(line: &str) -> Vec<usize> {
    line.split(' ')
        .nth(1)
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn places_where_it_would_fit(line: &str, expected: &[usize], wanted: usize) -> Vec<usize> {
    let wl = expected[wanted];
    let first = expected.get(..wanted).unwrap().iter().sum::<usize>() + wanted;
    let after =
        expected.get(wanted + 1..).unwrap().iter().sum::<usize>() + expected.len() - wanted - 1;
    let last = line.len() - after - wl;
    (first..=last)
        .filter(|&i| !line.get(i..(i + wl)).unwrap().contains('.'))
        .collect::<Vec<usize>>()
}

fn one_line_by_middle(line: &str, expected: &[usize]) -> u64 {
    if expected.is_empty() {
        return if line.contains('#') { 0 } else { 1 };
    }
    let middle_pos = expected.len() / 2;
    let middle_len = expected[middle_pos];
    let available = places_where_it_would_fit(line, expected, middle_pos);
    let result = available
        .iter()
        .map(|&p| {
            let left: u64 = if (p > 0) && (&line[p - 1..p] == "#") {
                0u64
            } else {
                one_line_by_middle(&line[..p.saturating_sub(1)], &expected[..middle_pos])
            };
            let right: u64 = if (p + middle_len < line.len())
                && (&line[p + middle_len..p + middle_len + 1] == "#")
            {
                0
            } else {
                one_line_by_middle(
                    &line[(p + middle_len + 1).min(line.len())..],
                    &expected[middle_pos + 1..],
                )
            };
            left * right
        })
        .sum();
    result
}

fn one_line_combinations(line: &str, expected: Vec<usize>) -> u64 {
    one_line_recursive(line, &expected, 0)
}

fn one_line_recursive(line: &str, expected: &[usize], position: usize) -> u64 {
    if position >= line.len() {
        if expected.is_empty() {
            return 1;
        }
        return 0;
    }
    match line.chars().nth(position) {
        Some('#') => {
            if expected.is_empty() {
                return 0;
            }
            if line
                .get(position..position + expected[0])
                .unwrap_or(".")
                .contains('.')
            {
                return 0;
            }
            if line.chars().nth(position + expected[0]) == Some('#') {
                return 0;
            }
            let only = one_line_recursive(line, &expected[1..], position + expected[0] + 1);
            return only;
        }
        Some('.') => {
            return one_line_recursive(line, expected, position + 1);
        }
        _ => {
            let first = one_line_recursive(line, expected, position + 1);
            let second = one_line_recursive(
                (line[..position].to_string() + "#" + &line[position + 1..]).as_str(),
                expected,
                position,
            );
            return first + second;
        }
    }
}

fn prob1(lines: Vec<&str>) -> u64 {
    let mut s: u64 = 0;
    for line in lines {
        let pad = line.split(' ').next().unwrap();
        s += one_line_combinations(pad, numbers_in_line(line));
    }
    s
}

fn prob2(lines: Vec<&str>) -> u64 {
    let mut s: u64 = 0;
    let multiplier = 5;
    for line in lines.iter() {
        let pad = line.split(' ').next().unwrap();
        let pad = std::iter::repeat(pad)
            .take(multiplier - 1)
            .fold(pad.to_string(), |s, x| s + "?" + x);
        let numbers = numbers_in_line(line);
        let numbers = (0..multiplier).fold(vec![], |mut a, _| {
            a.extend(numbers.clone());
            a
        });
        s += one_line_by_middle(pad.as_str(), &numbers);
    }
    s
}

pub fn main() {
    let input = include_str!("../day_12_input");
    let input: Vec<&str> = input.lines().collect();
    println!("{}", prob1(input.clone()));
    println!("{}", prob2(input.clone()));
}

#[cfg(test)]
mod tests {
    use crate::day_12::{
        numbers_in_line, one_line_by_middle, one_line_recursive, places_where_it_would_fit, prob1,
        prob2,
    };

    fn example() -> Vec<&'static str> {
        vec![
            "???.### 1,1,3",
            ".??..??...?##. 1,1,3",
            "?#?#?#?#?#?#?#? 1,3,1,6",
            "????.#...#... 4,1,1",
            "????.######..#####. 1,6,5",
            "?###???????? 3,2,1",
        ]
    }

    fn multip(line: &'static str, ns: &'static [usize], times: usize) -> (String, Vec<usize>) {
        let pad = std::iter::repeat(line)
            .take(times - 1)
            .fold(line.to_string(), |s, x| s + "?" + x);
        let numbers = (0..times).fold(vec![], |mut a, _| {
            a.extend(ns);
            a
        });
        (pad, numbers)
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(example()), 21);
    }

    #[test]
    fn test_prob2() {
        assert_eq!(prob2(example()), 525152);
    }

    #[test]
    fn test_one_line_recursive() {
        assert_eq!(one_line_recursive("???.###", &[1, 1, 3], 0), 1);
        assert_eq!(one_line_recursive(".??..??...?##", &[1, 1, 3], 0), 4);
        assert_eq!(one_line_recursive("?###????????", &[3, 2, 1], 0), 10);
        let (pad, numbers) = multip(".??..??...?##", &[1, 1, 3], 5);
        assert_eq!(one_line_recursive(&pad, &numbers, 0), 16384);
        let (pad, numbers) = multip(".??..??...?##", &[1, 1, 3], 2);
        assert_eq!(one_line_recursive(&pad, &numbers, 0), 32);
        let (pad, numbers) = multip("?#?#?#?#?#?#?#?", &[1, 3, 1, 6], 3);
        assert_eq!(one_line_recursive(&pad, &numbers, 0), 1);
        let (pad, numbers) = multip(".?.??#???..", &[1, 1, 1], 5);
        assert_eq!(one_line_recursive(&pad, &numbers, 0), 5);
    }

    #[test]
    fn test_numbers_in_line() {
        assert_eq!(numbers_in_line(example()[0]), vec![1, 1, 3]);
        assert_eq!(numbers_in_line(example()[1]), vec![1, 1, 3]);
        assert_eq!(numbers_in_line(example()[2]), vec![1, 3, 1, 6]);
    }

    #[test]
    fn test_places_where_it_would_fit() {
        assert_eq!(places_where_it_would_fit("???.###", &[1, 1, 3], 0), vec![0]);
        assert_eq!(
            places_where_it_would_fit(".??..??...?##", &[1, 1, 3], 1),
            vec![2, 5, 6]
        );
    }

    #[test]
    fn test_one_line_by_middle() {
        assert_eq!(one_line_by_middle("???.###", &[1, 1, 3]), 1);
        let (pad, numbers) = multip(".??..??...?##", &[1, 1, 3], 2);
        assert_eq!(one_line_by_middle(&pad, &numbers), 32);
        let (pad, numbers) = multip(".??..??...?##", &[1, 1, 3], 5);
        assert_eq!(one_line_by_middle(&pad, &numbers), 16384);
        let (pad, numbers) = multip("?#?#?#?#?#?#?#?", &[1, 3, 1, 6], 5);
        assert_eq!(one_line_by_middle(&pad, &numbers), 1);
        let (pad, numbers) = multip(".?.??#???..", &[1, 1, 1], 5);
        assert_eq!(one_line_by_middle(&pad, &numbers), 720005);
    }
}
