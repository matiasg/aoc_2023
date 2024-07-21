use std::collections::BTreeSet;
use std::fmt::Debug;

use crate::graphs::{DecoratedGraph, Graph};

fn dist(a: &(isize, isize), b: &(isize, isize)) -> isize {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn make_graph(input: &[&str], any_direction: bool) -> Graph<(isize, isize)> {
    let dots: Vec<(isize, isize)> = input
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(j, c)| {
                    if ".<v>^".contains(c) {
                        Some((i as isize, j as isize))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(isize, isize)>>()
        })
        .collect();
    let mut graph = Graph::new_with_nodes(dots);
    let nodes = graph.nodes.clone();
    for (i, pi) in nodes.iter().enumerate() {
        for (j, pj) in nodes.iter().enumerate() {
            if dist(pi, pj) == 1
                && can_go(
                    pi,
                    pj,
                    input[pi.0 as usize].chars().nth(pi.1 as usize).unwrap(),
                    any_direction,
                )
            {
                graph.add_edge_with_idxs(i, j);
            }
        }
    }
    graph
}

fn can_go(pi: &(isize, isize), pj: &(isize, isize), ci: char, any_direction: bool) -> bool {
    match ci {
        '.' => true,
        '>' => pj.1 == pi.1 + 1 || any_direction,
        'v' => pj.0 == pi.0 + 1 || any_direction,
        '<' => pj.1 == pi.1 - 1 || any_direction,
        '^' => pj.0 == pj.0 - 1 || any_direction,
        _ => false,
    }
}

fn prob1(input: &[&str]) -> usize {
    let start = (0isize, 1isize);
    let end = (input.len() as isize - 1, input[0].len() as isize - 2);
    let input = Vec::from(input);
    make_graph(&input, false)
        .bfs_acyclic_paths(start, end)
        .iter()
        .map(|p| p.len())
        .max()
        .unwrap()
        - 1 // start does not count
}

fn contract(graph: &Graph<(isize, isize)>) -> DecoratedGraph<Pt, usize> {
    let decision_vertices: Vec<(isize, isize)> = graph
        .nodes
        .iter()
        .filter(|&n| graph.edges_from(n).len() != 2)
        .copied()
        .collect();
    let mut result: DecoratedGraph<Pt, usize> =
        DecoratedGraph::new_with_nodes(decision_vertices.iter().map(|&(y, x)| Pt(y, x)));
    // DecoratedGraph::new_with_nodes(decision_vertices.clone());
    let mut interior_vertices: BTreeSet<(isize, isize)> =
        &BTreeSet::from_iter(graph.nodes.iter().copied())
            - &BTreeSet::from_iter(decision_vertices.iter().copied());
    while let Some(v) = interior_vertices.pop_first() {
        let mut path_ends: Vec<(isize, isize)> = vec![];
        let mut to_remove: BTreeSet<(isize, isize)> = BTreeSet::from([v]);
        for w in graph.edges_from(&v) {
            let mut x = w;
            while !decision_vertices.contains(&x) {
                to_remove.insert(x);
                x = *graph
                    .edges_from(&x)
                    .iter()
                    .find(|&y| !to_remove.contains(y))
                    .unwrap();
            }
            path_ends.push(x);
        }
        let v1 = path_ends[0];
        let v2 = path_ends[1];
        let d = to_remove.len() + 1;
        result.add_edge(Pt::from(v1), Pt::from(v2), d);
        result.add_edge(Pt::from(v2), Pt::from(v1), d);
        interior_vertices = &interior_vertices - &to_remove;
    }
    result
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
struct Pt(isize, isize);

impl Pt {
    fn from(yx: (isize, isize)) -> Pt {
        Pt(yx.0, yx.1)
    }
}

fn path_with_highest_sum(graph: DecoratedGraph<Pt, usize>, a: Pt, b: Pt) -> usize {
    let ia = graph.node_idx(&a);
    let mut result: Vec<Vec<usize>> = vec![];
    let mut considering: Vec<Vec<(usize, usize)>> = vec![vec![(ia, 0)]];
    while let Some(path_weights) = considering.pop() {
        let (last, _) = *path_weights.last().unwrap();
        let next_pts_weights = graph.edges_from_idxs(last);
        if next_pts_weights.is_empty() {
            result.push(path_weights.iter().map(|&(_, w)| w).collect());
        } else {
            for (next_pt, weight) in next_pts_weights {
                let next_idx = graph.node_idx(&next_pt);
                if path_weights
                    .iter()
                    .map(|&(n, _)| n)
                    .collect::<Vec<usize>>()
                    .contains(&next_idx)
                {
                    continue;
                }
                let path_with_next: Vec<(usize, usize)> = path_weights
                    .clone()
                    .into_iter()
                    .chain(std::iter::once((next_idx, weight)))
                    .collect();
                if next_pt == b {
                    result.push(path_with_next.into_iter().map(|(_, w)| w).collect());
                } else {
                    considering.push(path_with_next);
                }
            }
        }
    }
    result.iter().map(|ws| ws.iter().sum()).max().unwrap()
}

fn prob2(input: &[&str]) -> usize {
    let start = Pt(0isize, 1isize);
    let end = Pt(input.len() as isize - 1, input[0].len() as isize - 2);
    let input = Vec::from(input);
    let graph = make_graph(&input, true);
    let contracted = contract(&graph);
    println!(
        "graph len: {}, contracted len: {}",
        graph.len(),
        contracted.len()
    );
    path_with_highest_sum(contracted, start, end)
}

pub fn main() {
    let input: Vec<&str> = include_str!("../day_23_input").trim().lines().collect();
    println!("prob1: {}", prob1(&input));
    println!("prob2: {}", prob2(&input));
}

#[cfg(test)]
mod tests {
    use super::{contract, make_graph, prob1, prob2, Pt};

    fn example() -> Vec<&'static str> {
        vec![
            "#.#####################",
            "#.......#########...###",
            "#######.#########.#.###",
            "###.....#.>.>.###.#.###",
            "###v#####.#v#.###.#.###",
            "###.>...#.#.#.....#...#",
            "###v###.#.#.#########.#",
            "###...#.#.#.......#...#",
            "#####.#.#.#######.#.###",
            "#.....#.#.#.......#...#",
            "#.#####.#.#.#########v#",
            "#.#...#...#...###...>.#",
            "#.#.#v#######v###.###v#",
            "#...#.>.#...>.>.#.###.#",
            "#####v#.#.###v#.#.###.#",
            "#.....#...#...#.#.#...#",
            "#.#########.###.#.#.###",
            "#...###...#...#...#.###",
            "###.###.#.###v#####v###",
            "#...#...#.#.>.>.#.>.###",
            "#.###.###.#.###.#.#v###",
            "#.....###...###...#...#",
            "#####################.#",
        ]
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(&example()), 94);
    }

    #[test]
    fn test_prob2() {
        assert_eq!(prob2(&example()), 154);
    }

    #[test]
    fn test_graph() {
        let g = make_graph(&example(), false);
        assert_eq!(g.len(), 213);
        assert_eq!(g.edges_from(&(0, 1)), vec![(1, 1)]);
        assert_eq!(g.edges_from(&(3, 10)), vec![(3, 11)]);
        assert_eq!(g.edges_from(&(4, 3)), vec![(5, 3)]);
    }

    #[test]
    fn test_contract() {
        let g = make_graph(&example(), true);
        let gc = contract(&g);
        assert_eq!(gc.len(), 9);
        let start = (0, 1);
        let v2 = (5, 3);
        assert_eq!(gc.edges_from(&Pt::from(start)), vec![(Pt::from(v2), 15)]);
        assert_eq!(gc.edges_from(&Pt::from(v2)).len(), 3);
    }
}
