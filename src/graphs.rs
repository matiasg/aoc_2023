use itertools::iproduct;
use std::cmp::Reverse;
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

#[derive(Debug)]
pub struct Graph<N> {
    pub nodes: Vec<N>,
    pub edges: HashMap<usize, Vec<usize>>,
}

impl<N> Graph<N>
where
    N: Eq + PartialEq + Copy,
{
    pub fn new_with_nodes(nodes: impl IntoIterator<Item = N>) -> Self {
        let nodes: Vec<N> = nodes.into_iter().collect();
        let edges: HashMap<usize, Vec<usize>> = (0..nodes.len()).map(|i| (i, vec![])).collect();
        Self { nodes, edges }
    }
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
    pub fn add_node(&mut self, node: N) {
        self.nodes.push(node);
    }
    pub fn node_idx(&self, node: &N) -> usize {
        self.nodes.iter().position(|n| n == node).unwrap()
    }
    pub fn add_edge(&mut self, a: N, b: N) {
        let pa = self.node_idx(&a);
        let pb = self.node_idx(&b);
        self.add_edge_with_idxs(pa, pb);
    }
    pub fn add_edge_with_idxs(&mut self, pa: usize, pb: usize) {
        let ea = self.edges.get_mut(&pa).unwrap();
        match ea.iter().enumerate().find(|(_, pc)| **pc == pb) {
            Some((i, _)) => ea[i] = pb,
            None => ea.push(pb),
        };
    }
    pub fn edges_from_idxs(&self, node: usize) -> Vec<usize> {
        self.edges.get(&node).unwrap_or(&vec![]).clone()
    }
    pub fn edges_from(&self, node: &N) -> Vec<N> {
        let pn = self.node_idx(node);
        self.edges_from_idxs(pn)
            .iter()
            .map(|n| self.nodes[*n])
            .collect()
    }
    pub fn edges_to(&self, node: &N) -> Vec<N> {
        let pn = self.node_idx(node);
        self.edges
            .iter()
            .filter(|(_, vs)| vs.contains(&pn))
            .map(|(n, _)| self.nodes[*n])
            .collect()
    }
    pub fn all_paths(&self) -> HashMap<(usize, usize), Vec<Vec<usize>>> {
        let mut result: HashMap<(usize, usize), Vec<Vec<usize>>> = HashMap::new();
        self.edges.iter().for_each(|(n, vs)| {
            vs.iter().for_each(|v| {
                result.insert((*n, *v), vec![vec![*n]]);
            })
        });
        for middle in 0..self.nodes.len() {
            for from in 0..self.nodes.len() {
                for to in 0..self.nodes.len() {
                    let ft_added_paths: Vec<Vec<usize>> = iproduct!(
                        result.get(&(from, middle)).unwrap_or(&vec![]).iter(),
                        result.get(&(middle, to)).unwrap_or(&vec![]).iter()
                    )
                    .map(|(fm, mt)| fm.iter().chain(mt.iter()).cloned().collect())
                    .collect::<Vec<Vec<usize>>>();
                    result
                        .entry((from, to))
                        .or_insert(vec![])
                        .extend(ft_added_paths)
                }
            }
        }

        result
    }
    pub fn paths_between(&self, start: N, end: N) -> Vec<Vec<N>> {
        let start = self.node_idx(&start);
        let end = self.node_idx(&end);
        let all_paths = self.all_paths();
        let result_paths_idxs = all_paths.get(&(start, end)).unwrap();
        result_paths_idxs
            .iter()
            .map(|p| p.iter().map(|i| self.nodes[*i].clone()).collect())
            .collect()
    }
    /// number of paths between any 2 nodes by idx.
    /// Assumes that graph is acyclic. Otherwise, the result is wrong.
    pub fn all_paths_size(&self) -> Vec<Vec<usize>> {
        let mut result: Vec<Vec<usize>> = vec![vec![0; self.len()]; self.len()];
        self.edges.iter().for_each(|(n, vs)| {
            vs.iter().for_each(|v| {
                result[*n][*v] = 1;
            })
        });
        for middle in 0..self.nodes.len() {
            for from in 0..self.nodes.len() {
                for to in 0..self.nodes.len() {
                    result[from][to] += result[from][middle] * result[middle][to];
                }
            }
        }
        result
    }
    /// Dijkstra algorithm
    pub fn distance_between(&self, a: N, b: N) -> Option<usize> {
        let aidx = self.node_idx(&a);
        let bidx = self.node_idx(&b);
        let mut visited: Vec<bool> = vec![false; self.len()];
        let mut to_visit: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();
        visited[aidx] = true;
        to_visit.push((Reverse(0), aidx));
        while let Some((Reverse(dist), node)) = to_visit.pop() {
            let next_dist = dist + 1;
            for next in self.edges_from_idxs(node) {
                if visited[next] {
                    continue;
                }
                if next == bidx {
                    return Some(next_dist);
                }
                visited[next] = true;
                to_visit.push((Reverse(next_dist), next));
            }
        }
        None
    }
    /// Dijkstra algorithm for (maybe) more than one target
    pub fn distances_between(&self, a: N, b: &Vec<N>) -> Vec<Option<usize>> {
        let aidx = self.node_idx(&a);
        let idx_to_b: HashMap<usize, usize> = b
            .iter()
            .enumerate()
            .map(|(i, n)| (self.node_idx(n), i))
            .collect();
        let mut result: Vec<Option<usize>> = vec![None; b.len()];
        let mut visited: Vec<bool> = vec![false; self.len()];
        let mut to_visit: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();
        visited[aidx] = true;
        to_visit.push((Reverse(0), aidx));
        if let Some(&pos) = idx_to_b.get(&aidx) {
            result[pos] = Some(0);
        }
        while let Some((Reverse(dist), node)) = to_visit.pop() {
            let next_dist = dist + 1;
            for next in self.edges_from_idxs(node) {
                if visited[next] {
                    continue;
                }
                if let Some(&pos) = idx_to_b.get(&next) {
                    if result[pos].is_none() {
                        result[pos] = Some(next_dist);
                    }
                }
                visited[next] = true;
                to_visit.push((Reverse(next_dist), next));
            }
        }
        result
    }
    pub fn there_is_a_path_between(&self, a: N, b: N) -> bool {
        self.distance_between(a, b).is_some()
    }
    /// Floyd-Warshall algorithm.
    /// It's O(n^3) where n = #nodes
    pub fn all_distances(&self) -> Vec<Vec<usize>> {
        let mut result = vec![vec![usize::MAX; self.len()]; self.len()];
        for (from, tos) in self.edges.iter() {
            for to in tos.iter() {
                result[*from][*to] = 1;
            }
        }
        for middle in 0..self.len() {
            for from in 0..self.len() {
                if result[from][middle] == usize::MAX {
                    continue;
                }
                for to in 0..self.len() {
                    if result[middle][to] == usize::MAX {
                        continue;
                    }
                    result[from][to] =
                        result[from][to].min(result[from][middle] + result[middle][to]);
                }
            }
        }
        result
    }
    /// BFS from a to b
    /// The paths are taken care to no repeat nodes. The ids of the nodes are returned
    /// NOTE: also, paths that end in a leaf (or in a vertex from where there are no edges to unvisited nodes) are returned.
    pub fn bfs_acyclic_paths(&self, a: N, b: N) -> Vec<Vec<usize>> {
        let ia = self.node_idx(&a);
        let ib = self.node_idx(&b);
        let mut result = vec![];
        let mut considering: Vec<Vec<usize>> = vec![vec![ia]];
        while let Some(path) = considering.pop() {
            let last = *path.last().unwrap();
            let next_idxs = self.edges_from_idxs(last);
            if next_idxs.is_empty() {
                result.push(path);
            } else {
                for next in next_idxs {
                    if path.contains(&next) {
                        continue;
                    }
                    if next == ib {
                        result.push(
                            path.clone()
                                .into_iter()
                                .chain(std::iter::once(next))
                                .collect(),
                        );
                    } else {
                        considering.push(
                            path.clone()
                                .into_iter()
                                .chain(std::iter::once(next))
                                .collect(),
                        );
                    }
                }
            }
        }
        result
    }
}
impl<N> Graph<N>
where
    N: Debug + Eq + PartialEq + Copy + Hash,
{
    pub fn nodes_between(&self, start: N, end: N) -> Vec<N> {
        let s: HashSet<N> = self
            .paths_between(start, end)
            .into_iter()
            .flatten()
            .collect();
        s.clone().into_iter().collect()
    }
}
impl Graph<(isize, isize)> {
    pub fn from_maze(input: &Vec<&str>, floor: &str, wall: char) -> Self {
        let width = input[0].len() as isize;
        assert!(
            input.iter().all(|s| s.len() == width as usize),
            "all lines of `input` should have the same length"
        );
        let height = input.len() as isize;
        let nodes: Vec<(isize, isize)> = iproduct!(0..height, 0..width)
            .filter(|(y, x)| {
                floor
                    .chars()
                    .any(|c| c == input[*y as usize].chars().nth(*x as usize).unwrap())
            })
            .collect();
        let idx_nodes: HashMap<(isize, isize), usize> =
            nodes.iter().enumerate().map(|(i, &xy)| (xy, i)).collect();
        let mut edges: HashMap<usize, Vec<usize>> = HashMap::new();
        for (i, &(y, x)) in nodes.iter().enumerate() {
            let mut i_edges: Vec<usize> = vec![];
            if x > 0 && input[y as usize].chars().nth(x as usize - 1).unwrap() != wall {
                i_edges.push(idx_nodes[&(y, x - 1)]);
            }
            if x < width - 1 && input[y as usize].chars().nth(x as usize + 1).unwrap() != wall {
                i_edges.push(idx_nodes[&(y, x + 1)]);
            }
            if y > 0 && input[y as usize - 1].chars().nth(x as usize).unwrap() != wall {
                i_edges.push(idx_nodes[&(y - 1, x)]);
            }
            if y < height - 1 && input[y as usize + 1].chars().nth(x as usize).unwrap() != wall {
                i_edges.push(idx_nodes[&(y + 1, x)]);
            }
            if !i_edges.is_empty() {
                edges.insert(i, i_edges);
            }
        }

        Graph { nodes, edges }
    }
}

pub struct DecoratedGraph<N, E> {
    pub graph: Graph<N>,
    pub labels: HashMap<(usize, usize), E>,
}

impl<N, E> DecoratedGraph<N, E>
where
    N: Debug + Eq + PartialEq + Copy,
    E: Eq + Copy,
{
    pub fn new_with_nodes(nodes: impl IntoIterator<Item = N>) -> Self {
        Self {
            graph: Graph::new_with_nodes(nodes),
            labels: HashMap::new(),
        }
    }
    pub fn add_node(&mut self, node: N) {
        self.graph.add_node(node);
    }
    pub fn node_idx(&self, node: &N) -> usize {
        self.graph.node_idx(node)
    }
    pub fn add_edge(&mut self, a: N, b: N, label: E) {
        let pa = self.node_idx(&a);
        let pb = self.node_idx(&b);
        self.graph.add_edge(a, b);
        self.labels.insert((pa, pb), label);
    }
    pub fn edges_from(&self, node: &N) -> Vec<(N, E)> {
        let pn = self.node_idx(node);
        self.graph
            .edges
            .get(&pn)
            .unwrap()
            .iter()
            .map(|n| (self.graph.nodes[*n], self.labels[&(pn, *n)]))
            .collect()
    }
    pub fn len(&self) -> usize {
        self.graph.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn example1() -> Graph<&'static str> {
        let mut g = Graph::new_with_nodes(vec!["a", "b", "c", "d"]);
        g.add_edge("a", "b");
        g.add_edge("b", "d");
        g.add_edge("a", "c");
        g.add_edge("c", "d");
        g
    }
    fn example2() -> Graph<&'static str> {
        let mut g = Graph::new_with_nodes(vec!["a", "b", "c", "d", "e"]);
        g.add_edge("a", "d");
        g.add_edge("a", "b");
        g.add_edge("b", "c");
        g.add_edge("c", "d");
        g.add_edge("d", "e");
        g
    }
    fn example_cyclic() -> Graph<&'static str> {
        let mut g = Graph::new_with_nodes(vec!["a", "b", "c", "d"]);
        g.add_edge("a", "b");
        g.add_edge("b", "c");
        g.add_edge("c", "d");
        g.add_edge("d", "a");
        g
    }
    #[test]
    fn test_graph_with_nodes() {
        let mut g: Graph<&str> = Graph::new_with_nodes(vec!["a", "b", "c"]);
        assert_eq!(g.len(), 3);
        assert!(g.edges_from(&"a").is_empty());
        g.add_edge("a", "b");
        assert_eq!(g.edges_from(&"a"), vec!["b"]);
        assert!(g.edges_to(&"a").is_empty());
        assert_eq!(g.edges_to(&"b"), vec!["a"]);
        g.add_edge("c", "b");
        let mut et = g.edges_to(&"b");
        et.sort();
        assert_eq!(et, vec!["a", "c"]);
        g.add_edge("b", "b");
        let mut et = g.edges_to(&"b");
        et.sort();
        assert_eq!(et, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_paths() {
        let g = example1();
        let paths = g.paths_between("a", "d");
        assert_eq!(paths.len(), 2);
        assert!(paths.contains(&vec!["a", "b"]));
        assert!(paths.contains(&vec!["a", "c"]));
        assert_eq!(g.paths_between("d", "a").len(), 0);
        let mut nodes = g.nodes_between("a", "d");
        nodes.sort();
        assert_eq!(nodes, vec!["a", "b", "c"]);

        let g = example2();
        let paths = g.paths_between("a", "e");
        assert_eq!(paths.len(), 2);
        let paths_size = g.all_paths_size();
        assert_eq!(
            paths_size,
            vec![
                vec![0, 1, 1, 2, 2],
                vec![0, 0, 1, 1, 1],
                vec![0, 0, 0, 1, 1],
                vec![0, 0, 0, 0, 1],
                vec![0, 0, 0, 0, 0]
            ]
        );

        let g = example_cyclic();
        let paths = g.paths_between("a", "a");
        assert_eq!(paths.len(), 1);
    }

    #[test]
    fn test_decorated_graph() {
        let mut g: DecoratedGraph<u8, &str> = DecoratedGraph::new_with_nodes(vec![3, 1, 2]);
        assert_eq!(g.len(), 3);
        assert_eq!(g.node_idx(&3), 0);
        g.add_node(4);
        assert_eq!(g.len(), 4);
        g.add_edge(3, 4, "a");
        assert_eq!(g.edges_from(&3), vec![(4, "a")]);
    }

    #[test]
    fn test_paths_between() {
        let g = example1();
        let aidx = g.node_idx(&"a");
        let didx = g.node_idx(&"d");
        let paths = g.all_paths();
        assert_eq!(paths.get(&(aidx, didx)).unwrap().len(), 2);
    }

    #[test]
    fn test_distance_between() {
        let g = example1();
        assert_eq!(g.distance_between(&"a", &"b"), Some(1));
        assert_eq!(g.distance_between(&"a", &"d"), Some(2));
        assert_eq!(g.distance_between(&"d", &"a"), None);
        assert_eq!(
            g.nodes
                .iter()
                .filter(|&b| g.there_is_a_path_between(&"a", b))
                .count(),
            3
        );
        let distances = g.all_distances();
        assert_eq!(distances[0][3], 2);
        assert_eq!(distances[3][0], usize::MAX);
        assert_eq!(distances[0][1], 1);
        assert_eq!(distances[1][3], 1);
        let distances = g.distances_between(&"b", &vec![&"a", &"d"]);
        assert_eq!(distances, vec![None, Some(1)]);

        let maze: Vec<&str> = vec!["...", ".#.", "..."];
        let graph = Graph::from_maze(&maze, &".", '#');
        assert_eq!(graph.distance_between((0, 0), (0, 2)), Some(2));
        let distances = graph.distances_between(
            (0, 0),
            &vec![(0, 0), (1, 0), (0, 1), (0, 2), (2, 0), (2, 1), (2, 2)],
        );
        assert_eq!(
            distances,
            vec![
                Some(0),
                Some(1),
                Some(1),
                Some(2),
                Some(2),
                Some(3),
                Some(4),
            ]
        );
    }

    #[test]
    fn test_from_maze() {
        let maze: Vec<&str> = vec!["..#.", ".###", "...."];
        let graph = Graph::from_maze(&maze, &".", '#');
        assert_eq!(graph.len(), 8);
        assert_eq!(graph.edges.len(), 7);
        let edges_starts: HashSet<usize> = graph.edges.keys().map(|&n| n).collect();
        let expected_edges_start: Vec<(isize, isize)> =
            vec![(0, 0), (0, 1), (1, 0), (2, 0), (2, 1), (2, 2), (2, 3)];
        let expected_edges_start_idx: HashSet<usize> = expected_edges_start
            .iter()
            .map(|n| graph.node_idx(n))
            .collect();
        assert_eq!(edges_starts, expected_edges_start_idx);
        let distances = graph.distances_between((0, 0), &vec![(0, 0), (1, 0), (2, 3), (0, 3)]);
        assert_eq!(distances, vec![Some(0), Some(1), Some(5), None]);
    }

    #[test]
    fn test_bfs() {
        let maze: Vec<&str> = vec!["...", ".#.", "..."];
        let graph = Graph::from_maze(&maze, &".", '#');
        let paths = graph.bfs_acyclic_paths((0, 0), (2, 0));
        assert_eq!(paths.len(), 2);
        let expected: HashSet<Vec<usize>> = HashSet::from_iter([
            [(0, 0), (1, 0), (2, 0)]
                .iter()
                .map(|n| graph.node_idx(n))
                .collect(),
            [(0, 0), (0, 1), (0, 2), (1, 2), (2, 2), (2, 1), (2, 0)]
                .iter()
                .map(|n| graph.node_idx(n))
                .collect(),
        ]);
        assert_eq!(HashSet::from_iter(paths.clone()), expected);
    }
}
