fn prob1(lines: Vec<&str>) -> u64 {
    let mut result = 0;
    let split = input_split(&lines);
    let mut from = 0;
    for &to in split.iter() {
        let mp = mirror_place(lines.get(from..to).unwrap());
        println!("from: {}, to: {}, mp: {}", from, to, mp);
        result += mp;
        from = to + 1;
    }
    result
}

fn prob2(lines: Vec<&str>) -> u64 {
    let mut result = 0;
    let split = input_split(&lines);
    let mut from = 0;
    for &to in split.iter() {
        let mp = swap_place(lines.get(from..to).unwrap());
        println!("from: {}, to: {}, mp: {}", from, to, mp);
        result += mp;
        from = to + 1;
    }
    result
}

fn swap_place(lines: &[&str]) -> u64 {
    for i in 1..lines.len() {
        if horizontal_symmetricity(lines, i) == 1 {
            return 100 * i as u64;
        }
    }
    for j in 1..lines.first().unwrap().len() {
        if vertical_symmetricity(lines, j) == 1 {
            return j as u64;
        }
    }
    0
}

fn has_horizontal_symmetry(lines: &[&str], mirror: usize) -> bool {
    let max_symmetry_length = mirror.min(lines.len() - mirror);
    (0..max_symmetry_length).all(|i| lines[mirror + i] == lines[mirror - 1 - i])
}

fn horizontal_symmetricity(lines: &[&str], mirror: usize) -> u64 {
    let max_symmetry_length = mirror.min(lines.len() - mirror);
    (0..max_symmetry_length)
        .map(|i| {
            (0..lines.first().unwrap().len())
                .filter(|&j| {
                    lines[mirror + i].chars().nth(j) != lines[mirror - 1 - i].chars().nth(j)
                })
                .count()
        })
        .sum::<usize>() as u64
}

fn has_vertical_symmetry(lines: &[&str], mirror: usize) -> bool {
    let max_symmetry_length: usize = mirror.min(lines.first().unwrap().len() - mirror);
    lines.iter().all(|line| {
        (0..max_symmetry_length)
            .all(|j| line.chars().nth(mirror - 1 - j) == line.chars().nth(mirror + j))
    })
}

fn vertical_symmetricity(lines: &[&str], mirror: usize) -> u64 {
    let max_symmetry_length: usize = mirror.min(lines.first().unwrap().len() - mirror);
    lines
        .iter()
        .map(|line| {
            (0..max_symmetry_length)
                .filter(|j| line.chars().nth(mirror - 1 - j) != line.chars().nth(mirror + j))
                .count()
        })
        .sum::<usize>() as u64
}

fn mirror_place(lines: &[&str]) -> u64 {
    let horizontal_symmetry: u64 = (1..lines.len())
        .filter(|&i| has_horizontal_symmetry(lines, i))
        .sum::<usize>() as u64;
    let vertical_symmetry: u64 = (1..lines.first().unwrap().len())
        .filter(|&j| has_vertical_symmetry(lines, j))
        .sum::<usize>() as u64;
    horizontal_symmetry * 100 + vertical_symmetry
}

fn input_split(input: &Vec<&str>) -> Vec<usize> {
    input
        .iter()
        .enumerate()
        .filter_map(|(i, line)| if line.is_empty() { Some(i) } else { None })
        .chain(std::iter::once(input.len()))
        .collect()
}

pub fn main() {
    let input: Vec<&str> = include_str!("../day_13_input").trim().split('\n').collect();
    println!("prob 1: {}", prob1(input.clone()));
    println!("prob 2: {}", prob2(input));
}

#[cfg(test)]
mod tests {
    use crate::day_13::{
        has_horizontal_symmetry, horizontal_symmetricity, input_split, prob1, prob2,
        vertical_symmetricity,
    };

    fn example() -> Vec<&'static str> {
        vec![
            "#.##..##.",
            "..#.##.#.",
            "##......#",
            "##......#",
            "..#.##.#.",
            "..##..##.",
            "#.#.##.#.",
            "",
            "#...##..#",
            "#....#..#",
            "..##..###",
            "#####.##.",
            "#####.##.",
            "..##..###",
            "#....#..#",
        ]
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(example()), 405);
    }

    #[test]
    fn test_prob2() {
        assert_eq!(prob2(example()), 400);
    }

    #[test]
    fn test_split() {
        assert_eq!(input_split(&example()), vec![7, 15]);
    }

    #[test]
    fn test_has_horizontal_symmetry() {
        assert!(!has_horizontal_symmetry(&example().get(..7).unwrap(), 3));
        assert!(has_horizontal_symmetry(&example().get(8..).unwrap(), 4));
    }

    #[test]
    fn test_symmetricity() {
        assert_eq!(vertical_symmetricity(&example().get(..7).unwrap(), 5), 0);
        assert_eq!(horizontal_symmetricity(&example().get(..7).unwrap(), 3), 1);
    }
}
