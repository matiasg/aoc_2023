use std::fs;

fn prob1(input: Vec<&str>) -> usize {
    distances_plus_empty_times(input, 2)
}

fn prob2(input: Vec<&str>) -> usize {
    distances_plus_empty_times(input, 1_000_000)
}

fn distances_plus_empty_times(input: Vec<&str>, empty_multiplier: usize) -> usize {
    let accum_empty_columns = integral(get_empty_columns(&input));
    let accum_empty_rows = integral(get_empty_rows(&input));
    let galaxies = get_galaxies(&input);
    let mut total_dists: usize = 0;
    for (n, &(i1, j1)) in galaxies.iter().enumerate() {
        for &(i2, j2) in galaxies[n + 1..].iter() {
            let normal_dist = dist_l1((i1, j1), (i2, j2));
            let emptyness_distance = dist_l1(
                (accum_empty_rows[i1], accum_empty_columns[j1]),
                (accum_empty_rows[i2], accum_empty_columns[j2]),
            );
            total_dists += normal_dist + emptyness_distance * (empty_multiplier - 1);
        }
    }
    total_dists
}

fn get_empty_rows(input: &Vec<&str>) -> Vec<usize> {
    input
        .iter()
        .map(|&l| l.chars().filter(|c| *c == '#').count())
        .map(|s: usize| if s == 0 { 1 } else { 0 })
        .collect()
}

fn get_empty_columns(input: &Vec<&str>) -> Vec<usize> {
    (0..input[0].len())
        .map(|j| {
            input
                .iter()
                .map(|l| {
                    if l.chars().nth(j).unwrap() == '#' {
                        1
                    } else {
                        0
                    }
                })
                .sum()
        })
        .map(|s: usize| if s == 0 { 1 } else { 0 })
        .collect()
}

fn integral(series: Vec<usize>) -> Vec<usize> {
    series.iter().fold(vec![], |mut acc, &x| {
        acc.push(acc.last().unwrap_or(&0) + x);
        acc
    })
}

fn get_galaxies(input: &Vec<&str>) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = vec![];
    for (i, l) in input.iter().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == '#' {
                result.push((i, j));
            }
        }
    }
    result
}

fn dist_l1((i1, j1): (usize, usize), (i2, j2): (usize, usize)) -> usize {
    (i1 as i32 - i2 as i32).abs() as usize + (j1 as i32 - j2 as i32).abs() as usize
}

pub fn main() {
    let input = fs::read_to_string("day_11_input").expect("Could not read input.txt");
    let input: Vec<&str> = input.trim().split("\n").collect();
    println!("problem 1: {}", prob1(input.clone()));
    println!("problem 2: {}", prob2(input.clone()));
}

#[cfg(test)]
mod tests {
    use crate::day_11::{distances_plus_empty_times, integral, prob1};

    fn example() -> Vec<&'static str> {
        vec![
            "...#......",
            ".......#..",
            "#.........",
            "..........",
            "......#...",
            ".#........",
            ".........#",
            "..........",
            ".......#..",
            "#...#.....",
        ]
    }

    #[test]
    fn test_integral() {
        assert_eq!(integral(vec![1, 2, 3]), vec![1, 3, 6]);
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(example()), 374);
    }

    #[test]
    fn test_general_prob() {
        assert_eq!(distances_plus_empty_times(example(), 100), 8410);
        assert_eq!(distances_plus_empty_times(example(), 10), 1030);
    }
}
