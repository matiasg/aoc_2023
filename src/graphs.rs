use itertools::iproduct;
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

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
        let mut to_visit: BinaryHeap<(usize, usize)> = BinaryHeap::new();
        visited[aidx] = true;
        to_visit.push((0, aidx));
        while let Some((dist, node)) = to_visit.pop() {
            let next_dist = dist + 1;
            for next in self.edges_from_idxs(node) {
                if visited[next] {
                    continue;
                }
                if next == bidx {
                    return Some(next_dist);
                }
                visited[next] = true;
                to_visit.push((next_dist, next));
            }
        }
        None
    }
    pub fn there_is_a_path_between(&self, a: N, b: N) -> bool {
        self.distance_between(a, b).is_some()
    }
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
    fn node_idx(&self, node: &N) -> usize {
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
    }
}
