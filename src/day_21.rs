use std::collections::{HashMap, HashSet};

use crate::graphs::Graph;

fn neighbours(map: &Vec<&str>, pos: (isize, isize)) -> Vec<(isize, isize)> {
    let maxx = map[0].len() as isize;
    let maxy = map.len() as isize;
    let pre_result: Vec<(isize, isize)> = vec![
        (pos.0 - 1, pos.1),
        (pos.0 + 1, pos.1),
        (pos.0, pos.1 - 1),
        (pos.0, pos.1 + 1),
    ];
    pre_result
        .into_iter()
        .filter(|p| 0 <= p.0 && p.0 < maxy && 0 <= p.1 && p.1 < maxx)
        .filter(|&(y, x)| map[y as usize].chars().nth(x as usize).unwrap() != '#')
        .collect()
}
fn next_states(map: &Vec<&str>, positions: HashSet<(isize, isize)>) -> HashSet<(isize, isize)> {
    positions
        .iter()
        .map(|p| neighbours(map, *p))
        .flatten()
        .collect()
}
fn start_positions(map: &Vec<&str>) -> (isize, isize) {
    let y = map.iter().position(|s| s.contains('S')).unwrap();
    let x = map[y].chars().position(|c| c == 'S').unwrap();
    (y as isize, x as isize)
}

fn do_steps(input: &Vec<&str>, steps: usize) -> HashSet<(isize, isize)> {
    let mut reached: HashSet<(isize, isize)> = HashSet::from([start_positions(input)]);
    for _ in 0..steps {
        reached = next_states(input, reached);
    }
    reached
}
fn prob1(input: &Vec<&str>) -> usize {
    do_steps(input, 64).len()
}

fn prob2_steps(input: &Vec<&str>, steps: isize) -> isize {
    let (y, x) = start_positions(input);
    let height = input.len() as isize;
    let width = input[0].len() as isize;
    let graph = Graph::from_maze(input, ".S", '#');
    let start_and_corners: Vec<(isize, isize)> = vec![
        (y, x),
        (0, 0),
        (0, x),
        (0, width - 1),
        (y, width - 1),
        (height - 1, width - 1),
        (height - 1, x),
        (height - 1, 0),
        (y, 0),
    ];
    println!("computing distances for verts and mids");
    let distances: Vec<Vec<isize>> = start_and_corners
        .iter()
        .map(|&f| {
            graph
                .distances_between(f, &graph.nodes)
                .iter()
                .map(|d| d.unwrap_or(steps as usize + 1) as isize)
                .collect()
        })
        .collect();
    let max_distances: Vec<isize> = distances
        .iter()
        .map(|ds| {
            ds.iter()
                .filter(|&d| *d <= steps)
                .map(|&d| d)
                .max()
                .unwrap()
        })
        .collect();
    let one_square_odd = distances[0]
        .iter()
        .filter(|&&d| (d <= steps) && (d % 2 == 1))
        .count() as isize;
    let one_square_even = distances[0]
        .iter()
        .filter(|&&d| (d <= steps) && (d % 2 == 0))
        .count() as isize;
    let maxi = steps / height;

    let mut total: isize = if steps % 2 == 0 {
        one_square_even
    } else {
        one_square_odd
    };

    let how_to_compute: HashMap<usize, (usize, isize)> = HashMap::from([
        (0, (1, width)),
        (1, (3, height)),
        (2, (5, width)),
        (3, (7, height)),
    ]);
    let mut distances_middle: Vec<Vec<isize>> = Vec::new();
    let mut dmright: Vec<isize> = Vec::new();
    for (n, _) in graph.nodes.iter().enumerate() {
        dmright.push(
            // [3usize, 5]  use this for the tests
            [3, 4, 5]
                .iter()
                .map(|&d| {
                    let d2: usize = [0, 0, 0, 1, 8, 7][d];
                    let nd = graph.node_idx(&start_and_corners[d]);
                    distances[0][nd] + 1 + distances[d2][n]
                })
                .min()
                .unwrap(),
        );
    }
    distances_middle.push(dmright);
    let mut dmdown: Vec<isize> = Vec::new();
    for (n, _) in graph.nodes.iter().enumerate() {
        dmdown.push(
            // [5usize, 7]  use this for the tests
            [5, 6, 7]
                .iter()
                .map(|&d| {
                    let d2 = 8 - d;
                    let nd = graph.node_idx(&start_and_corners[d]);
                    distances[0][nd] + 1 + distances[d2][n]
                })
                .min()
                .unwrap(),
        );
    }
    distances_middle.push(dmdown);
    let mut dmleft: Vec<isize> = Vec::new();
    for (n, _) in graph.nodes.iter().enumerate() {
        dmleft.push(
            // [7, 1]  use this for the tests
            [7, 8, 1]
                .iter()
                .map(|&d| {
                    let d2 = (12 - d) % 8;
                    let nd = graph.node_idx(&start_and_corners[d]);
                    distances[0][nd] + 1 + distances[d2][n]
                })
                .min()
                .unwrap(),
        );
    }
    distances_middle.push(dmleft);
    let mut dmup: Vec<isize> = Vec::new();
    for (n, _) in graph.nodes.iter().enumerate() {
        dmup.push(
            // [1, 3]  use this for the tests
            [1, 2, 3]
                .iter()
                .map(|&d| {
                    let d2 = 8 - d;
                    let nd = graph.node_idx(&start_and_corners[d]);
                    distances[0][nd] + 1 + distances[d2][n]
                })
                .min()
                .unwrap(),
        );
    }
    distances_middle.push(dmup);
    // FIXME: should compute this for a few added blocks but it's the same on the input

    let mut this_sq;
    for dir in 0..4 {
        let (v_centers_idx, dir_hor_vert) = how_to_compute[&dir];
        let op_v_centers_idx = (v_centers_idx + 4) % 8;
        let v_idx = graph.node_idx(&start_and_corners[v_centers_idx]);
        let op_v_idx = graph.node_idx(&start_and_corners[op_v_centers_idx]);
        // this assumes that the path passes through opposed corner. True in input
        let d_start_to_v = distances[0][op_v_idx] + 2;
        let other_dir = height * width / dir_hor_vert;
        println!("starting with dir {dir}, v_idx {v_idx}, op_v_idx {op_v_idx}, d_start_to_v {d_start_to_v}");
        for i in 1..=maxi {
            // d_start_to_v + (i-1) * other_dir + (j-1) * dir_hor_vert + distances[v_centers_idx][k] == steps
            let minj = 1
                + (steps - d_start_to_v - (i - 1) * other_dir - max_distances[v_centers_idx])
                    / dir_hor_vert;
            let maxj = 1 + (steps - d_start_to_v - (i - 1) * other_dir) / dir_hor_vert + 1;
            for j in 1..=maxj {
                let d_to_sq = d_start_to_v + (i - 1) * other_dir + (j - 1) * dir_hor_vert;
                let this_sq;
                if j < minj {
                    this_sq = if (i * dir_hor_vert + j * other_dir - steps) % 2 == 0 {
                        one_square_even
                    } else {
                        one_square_odd
                    };
                } else {
                    this_sq = distances[v_centers_idx]
                        .iter()
                        .filter(|&d| (d_to_sq + *d <= steps) && (d_to_sq + *d - steps) % 2 == 0)
                        .count() as isize;
                }
                total += this_sq;
            }
        }
        // now i = 0
        // distances__middle[dir][k] + (j-1) * dir_hor_vert == steps
        let minj = 1 + ((steps - distances_middle[dir].iter().max().unwrap()) / dir_hor_vert);
        let maxj = 2 + steps / dir_hor_vert;
        for j in 1..=maxj {
            if j < minj {
                this_sq = if (dir_hor_vert * j - steps) % 2 == 0 {
                    one_square_even
                } else {
                    one_square_odd
                };
            } else {
                this_sq = distances_middle[dir]
                    .iter()
                    .filter(|&d| {
                        (j - 1) * dir_hor_vert + *d <= steps
                            && (dir_hor_vert * (j - 1) + *d - steps) % 2 == 0
                    })
                    .count() as isize;
            }
            total += this_sq;
        }
    }

    total
}

fn prob2(input: &Vec<&str>) -> isize {
    prob2_steps(input, 26501365)
}

pub fn main() {
    let input: Vec<&str> = include_str!("../day_21_input").trim().split('\n').collect();
    println!("prob1: {}", prob1(&input));
    println!("prob2: {}", prob2(&mut input.clone()));
}

#[cfg(test)]
mod tests {
    use crate::day_21::{do_steps, prob2_steps};

    fn example() -> Vec<&'static str> {
        vec![
            "...........",
            ".....###.#.",
            ".###.##..#.",
            "..#.#...#..",
            "....#.#....",
            ".##..S####.",
            ".##..#...#.",
            ".......##..",
            ".##.#.####.",
            ".##..##.##.",
            "...........",
        ]
    }

    #[test]
    fn test_prob1() {
        assert_eq!(do_steps(&example(), 6).len(), 16);
    }

    #[test]
    fn test_prob2() {
        // assert_eq!(prob2_steps(&example(), 6), 16);
        // assert_eq!(prob2_steps(&example(), 10), 50);
        assert_eq!(prob2_steps(&example(), 50), 1594);
        assert_eq!(prob2_steps(&example(), 100), 6536);
        assert_eq!(prob2_steps(&example(), 1000), 668697);
        assert_eq!(prob2_steps(&example(), 5000), 16733044);
    }
}
