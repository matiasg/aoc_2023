use std::collections::{HashMap, HashSet};
use std::fmt;

use crate::graphs::Graph;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Brick {
    start: (i32, i32, i32),
    end: (i32, i32, i32),
}
impl fmt::Debug for Brick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}--{:?}]", self.start, self.end)
    }
}
impl Brick {
    fn from_input_line(line: &str) -> Self {
        let se: Vec<&str> = line.split('~').collect();
        let start: Vec<i32> = se[0].split(',').map(|n| n.parse().unwrap()).collect();
        let end: Vec<i32> = se[1].split(',').map(|n| n.parse().unwrap()).collect();
        assert!(start.iter().zip(end.iter()).filter(|(s, e)| s == e).count() >= 2);
        return if start < end {
            Self {
                start: (start[0], start[1], start[2]),
                end: (end[0], end[1], end[2]),
            }
        } else {
            Self {
                end: (start[0], start[1], start[2]),
                start: (end[0], end[1], end[2]),
            }
        };
    }

    /// returns (bottom,left), (top,right)
    fn bounding_xy_box(&self) -> ((i32, i32), (i32, i32)) {
        let bottom = self.start.1.min(self.end.1);
        let left = self.start.0.min(self.end.0);
        let top = self.start.1.max(self.end.1);
        let right = self.start.0.max(self.end.0);
        ((bottom, left), (top, right))
    }

    fn depends_on(&self, other: &Self) -> bool {
        let bbox_self = self.bounding_xy_box();
        let bbox_other = other.bounding_xy_box();
        if bbox_self.0 .0 > bbox_other.1 .0 || bbox_self.1 .0 < bbox_other.0 .0 {
            return false;
        }
        if bbox_self.0 .1 > bbox_other.1 .1 || bbox_self.1 .1 < bbox_other.0 .1 {
            return false;
        }
        other.end.2 < self.start.2
    }
}

/// Note: assumes input `bricks` are sorted by z
fn bricks_graph(bricks: &Vec<Brick>) -> Graph<&Brick> {
    let mut g = Graph::new_with_nodes(bricks);
    for (i, &bi) in bricks.iter().enumerate() {
        for (i_plus_1_to_j, &bj) in bricks.get(i + 1..).unwrap().iter().enumerate() {
            if bj.depends_on(&bi) {
                g.add_edge_with_idxs(i, i + 1 + i_plus_1_to_j);
            }
        }
    }
    g
}

fn transitively_close<N>(graph: &mut Graph<N>) -> Vec<Vec<usize>>
where
    N: Copy + Eq,
{
    let mut max_dists: Vec<Vec<usize>> = vec![vec![0; graph.len()]; graph.len()];
    for from in 0..graph.len() {
        for &to in graph.edges.get(&from).unwrap().iter() {
            max_dists[from][to] = 1
        }
    }
    for mid in 0..graph.len() {
        for low in 0..mid {
            let edges_mid = graph.edges.get(&mid).unwrap().clone();
            for &top in edges_mid.iter() {
                let path_through_mid = max_dists[low][mid] + max_dists[mid][top];
                max_dists[low][top] = max_dists[low][top].max(path_through_mid);
                graph.add_edge_with_idxs(low, top);
            }
        }
    }

    max_dists
}

fn prob2(input: &Vec<&str>) -> usize {
    let (bricks, supported_by) = get_bricks_and_supports(input);
    let on_floor: Vec<usize> = bricks
        .iter()
        .enumerate()
        .filter(|(_, b)| b.start.2 == 1)
        .map(|(i, _)| i)
        .collect();
    let mut all_paths: Vec<Vec<usize>> = vec![vec![0; bricks.len()]; bricks.len()];
    for (&i, sups) in supported_by.iter() {
        for &j in sups.iter() {
            all_paths[j][i] = 1;
        }
    }
    // does removing bi move bj? Take a k != i, and let
    // bi -- [pij] -> bj  (pij paths from bi to bj)
    // bk -- [pkj] -> bj
    // bk -- [pki] -> bi
    // if pkj > pki * pij then bk has paths to bj that do not pass through bi
    // Conversely, if removing bi does not move bj there must be some other support
    // for bj. That support has some brick on the floor below, bk, such that there
    // are paths bk -> bj that do not go through bi. That is, pkj > pki * pij.
    // So we compute first all paths between nodes
    for (j, bj) in bricks.iter().enumerate() {
        for (i, bi) in bricks.iter().enumerate() {
            if bi.end.2 >= bj.start.2 {
                continue;
            }
            for (k, bk) in bricks.iter().enumerate() {
                if bj.end.2 >= bk.start.2 {
                    continue;
                }
                all_paths[i][k] += all_paths[i][j] * all_paths[j][k];
            }
        }
    }
    // now count all pairs i, j such that no k != i, k in the floor
    // such that pkj > pki * pij.
    let mut result: usize = 0;
    for i in 0..bricks.len() {
        for j in 0..bricks.len() {
            if all_paths[i][j] == 0 {
                continue;
            }
            if on_floor
                .iter()
                .all(|&k| k == i || all_paths[k][j] <= all_paths[k][i] * all_paths[i][j])
            {
                result += 1;
            }
        }
    }
    result
}

fn prob1(input: &Vec<&str>) -> usize {
    let (bricks, supported_by) = get_bricks_and_supports(input);
    let supports: HashSet<usize> = supported_by
        .values()
        .filter_map(|sups| if sups.len() == 1 { Some(sups[0]) } else { None })
        .collect();

    bricks.len() - supports.len()
}

// 2nd returned is i -> [ai1,...,ain] where each aij supports i
fn get_bricks_and_supports(input: &Vec<&str>) -> (Vec<Brick>, HashMap<usize, Vec<usize>>) {
    let mut bricks: Vec<Brick> = input
        .iter()
        .map(|&line| Brick::from_input_line(line))
        .collect::<Vec<Brick>>();
    bricks.sort_by_cached_key(|b| b.start.2);
    let bricks_down = fallen_bricks(&bricks);

    let mut supported_by: HashMap<usize, Vec<usize>> = HashMap::new();
    for (i, below) in bricks_down.iter().enumerate() {
        for (i_plus_1_to_j, above) in bricks_down.iter().skip(i + 1).enumerate() {
            if above.start.2 == below.end.2 + 1 && above.depends_on(below) {
                supported_by
                    .entry(i + 1 + i_plus_1_to_j)
                    .or_insert(vec![])
                    .push(i);
            }
        }
    }
    (bricks_down, supported_by)
}

fn fallen_bricks(bricks: &[Brick]) -> Vec<Brick> {
    let mut result: Vec<Brick> = vec![];
    for b in bricks.iter() {
        let falls_to_brick = result
            .iter()
            .rev()
            .filter(|&fb| b.depends_on(&fb))
            .max_by_key(|&fb| fb.end.2);
        let falls_to = falls_to_brick.map_or(1, |fb| fb.end.2 + 1);
        let fallen_brick = Brick {
            start: (b.start.0, b.start.1, falls_to),
            end: (b.end.0, b.end.1, b.end.2 - b.start.2 + falls_to),
        };
        result.push(fallen_brick);
    }
    result
}

pub fn main() {
    let input: Vec<&str> = include_str!("../day_22_input").trim().split('\n').collect();
    println!("prob 1: {}", prob1(&input));
    println!("prob 2: {}", prob2(&input));
}

#[cfg(test)]
mod tests {

    use crate::graphs::Graph;

    use super::{bricks_graph, fallen_bricks, prob1, prob2, transitively_close, Brick};

    fn example() -> Vec<&'static str> {
        vec![
            "1,0,1~1,2,1",
            "0,0,2~2,0,2",
            "0,2,3~2,2,3",
            "0,0,4~0,2,4",
            "2,0,5~2,2,5",
            "0,1,6~2,1,6",
            "1,1,8~1,1,9",
        ]
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(&example()), 5);
    }

    #[test]
    fn test_brick() {
        let bricks = sorted_example();
        assert_eq!(bricks.len(), 7);
        assert!(bricks.iter().all(|b| b.start.2 <= b.end.2));
        assert!(!bricks[0].depends_on(&bricks[1]));
        assert!(!bricks[0].depends_on(&bricks[0]));
        assert!(bricks[1].depends_on(&bricks[0]));
        assert!(bricks[2].depends_on(&bricks[0]));
        assert!(bricks.iter().all(|b| !b.depends_on(&bricks[6])))
    }

    fn sorted_example() -> Vec<Brick> {
        let mut bricks: Vec<Brick> = example()
            .iter()
            .map(|&line| Brick::from_input_line(line))
            .collect();
        bricks.sort_by_cached_key(|b| b.start.2);
        bricks
    }

    #[test]
    fn test_graph() {
        let unsorted_bricks: Vec<Brick> = example()
            .iter()
            .map(|&line| Brick::from_input_line(line))
            .collect();
        let sorted_bicks = sorted_example();
        let g = bricks_graph(&sorted_bicks);
        assert_eq!(g.len(), 7);

        let sorting_idx: Vec<usize> = unsorted_bricks.iter().map(|b| g.node_idx(&b)).collect();
        let edges_from_a = g.edges_from(&&unsorted_bricks[0]);
        let expected_edges = vec![1usize, 2];
        assert!(expected_edges
            .iter()
            .all(|&be| edges_from_a.contains(&g.nodes[sorting_idx[be]])));
    }

    #[test]
    fn test_transitive_closure() {
        let mut g = Graph::new_with_nodes([0, 1, 2, 3, 4]);
        for i in 0..4 {
            g.add_edge(i, i + 1);
        }
        g.add_edge(1, 3);
        g.add_edge(0, 4);
        let md = transitively_close(&mut g);
        for i in 0..4 {
            for j in (i + 1)..=4 {
                assert!(g.edges_from(&i).contains(&j));
                assert!(!g.edges_from(&j).contains(&i));
                assert_eq!(md[i][j], j - i);
                assert_eq!(md[j][i], 0);
            }
        }
    }

    #[test]
    fn test_fallen() {
        let sorted_bricks = sorted_example();
        let fallen = fallen_bricks(&sorted_bricks);
        assert_eq!(fallen.len(), 7);
        assert_eq!(fallen[0], sorted_bricks[0]);
        assert_eq!(fallen[1], sorted_bricks[1]);
        assert_eq!(fallen[2].start.2, 2);
        assert_eq!(fallen[6].start.2, 5);
        assert_eq!(fallen[6].end.2, 6);
    }

    fn example2() -> Vec<&'static str> {
        vec![
            "0,0,2~0,1,2",
            "0,2,2~1,2,2",
            "2,2,2~2,1,2",
            "2,0,2~1,0,2",
            "0,0,3~1,0,3",
            "2,0,3~2,1,3",
            "2,2,3~1,2,3",
            "0,2,3~0,1,3",
            "0,0,4~0,1,4",
            "0,2,4~1,2,4",
            "2,2,4~2,1,4",
            "2,0,4~1,0,4",
            "0,0,5~1,0,5",
            "2,0,5~2,1,5",
            "2,2,5~1,2,5",
            "0,2,5~0,1,5",
        ]
    }
    #[test]
    fn test_prob2() {
        let sum_of_moves = prob2(&example());
        assert_eq!(sum_of_moves, 7);
        assert_eq!(prob2(&example2()), 0);
    }
}
