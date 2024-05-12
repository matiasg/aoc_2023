use itertools::iproduct;
use std::fs;

fn prob1(input: Vec<&str>) -> u32 {
    let mut distances: Vec<Vec<u32>> = vec![vec![0; input[0].len()]; input.len()];
    let (mut y, mut x) = get_start(&input);
    let (y0, x0) = (y, x);
    let mut distance = 0_u32;
    while distances[y0][x0] == 0 {
        let ((y1, x1), (y2, x2)) = get_neighbours(&input, y, x);
        distance += 1;
        if distances[y1][x1] == 0 {
            distances[y1][x1] = distance;
            (y, x) = (y1, x1);
        } else {
            distances[y2][x2] = distance;
            (y, x) = (y2, x2);
        }
    }
    distances[y][x] / 2
}

fn is_enclosed(input: &Vec<(usize, usize)>, y: usize, x: usize) -> bool {
    let mut integral = 0f32;
    for ((ay, ax), (by, bx)) in input.iter().zip(input[1..].iter()) {
        if (ay, ax) == (&y, &x) {
            return false;
        }
        let a_xy = ((*ay as f32 - y as f32), (*ax as f32 - x as f32));
        let a_xy_norm = (a_xy.0 * a_xy.0 + a_xy.1 * a_xy.1).sqrt();
        let b_xy = ((*by as f32 - y as f32), (*bx as f32 - x as f32));
        let b_xy_norm = (b_xy.0 * b_xy.0 + b_xy.1 * b_xy.1).sqrt();

        let anglediff = ((a_xy.0 * b_xy.1 - a_xy.1 * b_xy.0) / (a_xy_norm * b_xy_norm)).asin();
        integral += anglediff;
    }
    integral.abs() > 1f32
}

fn prob2(input: Vec<&str>) -> usize {
    let main_loop = get_main_loop(input.clone());
    let all_pos: Vec<(usize, usize)> =
        iproduct!(0..input.clone().len(), 0..input[0].len()).collect();
    let all_inside = all_pos.iter().filter(|p| is_enclosed(&main_loop, p.0, p.1));
    all_inside.count()
}

fn get_main_loop(input: Vec<&str>) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = vec![];
    let (mut y, mut x) = get_start(&input);
    let (y0, x0) = (y, x);
    result.push((y, x));
    (y, x) = get_neighbours(&input, y, x).0;
    while (y, x) != (y0, x0) {
        result.push((y, x));
        let ((y1, x1), (y2, x2)) = get_neighbours(&input, y, x);
        if result.len() == 1 || (y1, x1) != result[result.len() - 2] {
            (y, x) = (y1, x1);
        } else {
            (y, x) = (y2, x2);
        }
    }
    result.push((y, x));
    result
}

fn is_inside(input: &[&str], y: isize, x: isize) -> bool {
    0 <= y && y < input.len() as isize && 0 <= x && x < input[0].len() as isize
}

fn get_neighbours(input: &[&str], y: usize, x: usize) -> ((usize, usize), (usize, usize)) {
    match input[y].chars().nth(x).unwrap() {
        '-' => ((y, x - 1), (y, x + 1)),
        '|' => ((y - 1, x), (y + 1, x)),
        'F' => ((y, x + 1), (y + 1, x)),
        'L' => ((y - 1, x), (y, x + 1)),
        '7' => ((y, x - 1), (y + 1, x)),
        'J' => ((y - 1, x), (y, x - 1)),
        'S' => {
            let mut sn: Vec<(usize, usize)> = vec![];
            if is_inside(input, y as isize - 1, x as isize)
                && "F|7".contains(input[y - 1].chars().nth(x).unwrap())
            {
                sn.push((y - 1, x));
            }
            if is_inside(input, y as isize, x as isize - 1)
                && "L-F".contains(input[y].chars().nth(x - 1).unwrap())
            {
                sn.push((y, x - 1));
            }
            if is_inside(input, y as isize, x as isize + 1)
                && "J-7".contains(input[y].chars().nth(x + 1).unwrap())
            {
                sn.push((y, x + 1));
            }
            if is_inside(input, y as isize + 1, x as isize)
                && "J|L".contains(input[y + 1].chars().nth(x).unwrap())
            {
                sn.push((y + 1, x));
            }
            assert_eq!(sn.len(), 2);
            ((sn[0].0, sn[0].1), (sn[1].0, sn[1].1))
        }
        _ => ((0, 0), (0, 0)),
    }
}

fn get_start(input: &[&str]) -> (usize, usize) {
    for (y, line) in input.iter().enumerate() {
        if let Some(x) = line.find('S') {
            return (y, x);
        }
    }
    (0, 0)
}

pub fn main() {
    let input = fs::read_to_string("day_10_input").expect("no input file");
    let input: Vec<&str> = input.trim().split("\n").collect();
    println!("prob1: {}", prob1(input.clone()));
    println!("prob2: {}", prob2(input.clone()));
}

#[cfg(test)]
mod tests {
    use itertools::iproduct;

    use crate::day_10::{get_main_loop, get_start, is_enclosed, prob1, prob2};

    fn example() -> Vec<&'static str> {
        vec!["..F7.", ".FJ|.", "SJ.L7", "|F--J", "LJ..."]
    }

    fn example1() -> Vec<&'static str> {
        vec![
            "...........",
            ".S-------7.",
            ".|F-----7|.",
            ".||.....||.",
            ".||.....||.",
            ".|L-7.F-J|.",
            ".|..|.|..|.",
            ".L--J.L--J.",
            "...........",
        ]
    }

    fn example2() -> Vec<&'static str> {
        vec![
            "FF7FSF7F7F7F7F7F---7",
            "L|LJ||||||||||||F--J",
            "FL-7LJLJ||||||LJL-77",
            "F--JF--7||LJLJ7F7FJ-",
            "L---JF-JLJ.||-FJLJJ7",
            "|F|F-JF---7F7-L7L|7|",
            "|FFJF7L7F-JF7|JL---7",
            "7-L-JL7||F7|L7F-7F7|",
            "L.L7LFJ|||||FJL7||LJ",
            "L7JLJL-JLJLJL--JLJ.L",
        ]
    }

    #[test]
    fn test_get_start() {
        assert_eq!(get_start(&example()), (2, 0));
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(example()), 8);
    }

    #[test]
    fn test_is_enclosed() {
        let e1 = example1();
        let main_loop = get_main_loop(e1.clone());
        let all_pos: Vec<(usize, usize)> = iproduct!(0..e1.len(), 0..e1[0].len()).collect();
        let all_inside: Vec<&(usize, usize)> = all_pos
            .iter()
            .filter(|p| is_enclosed(&main_loop, p.0, p.1))
            .collect();
        assert_eq!(all_inside, vec![&(6, 2), &(6, 3), &(6, 7), &(6, 8)]);
    }

    #[test]
    fn test_get_main_loop() {
        assert_eq!(get_main_loop(example1()).len(), 47);
    }

    #[test]
    fn test_prob2() {
        assert_eq!(prob2(example2()), 10);
    }
}
