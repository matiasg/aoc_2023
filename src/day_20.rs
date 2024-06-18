use itertools::iproduct;
use num::Integer;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::graphs::Graph;
use crate::math::chinese_reminder_theorem;

struct Pulse<'a> {
    from: &'a str,
    to: &'a str,
    is_high: bool,
}

fn once(
    graph: &Graph<&str>,
    fliflops: &Vec<bool>,
    is_on: &mut Vec<bool>,
    last_signals: &mut HashMap<usize, Vec<(usize, bool)>>,
    loops: &mut u64,
) -> (u64, u64) {
    *loops += 1;
    let mut low_pulses = 1;
    let mut high_pulses = 0;
    let brdc = "broadcaster";
    let mut to_check: VecDeque<Pulse> = graph
        .edges_from(&brdc)
        .into_iter()
        .map(|n| Pulse {
            from: brdc,
            to: n,
            is_high: false,
        })
        .collect();
    while let Some(p) = to_check.pop_front() {
        match p.is_high {
            true => high_pulses += 1,
            false => low_pulses += 1,
        };
        let to_idx = graph.node_idx(&p.to);
        if fliflops[to_idx] {
            // high pulses are ignored by fliflops
            // a low pulse on a fliflop inverts it and makes it emit pulses with the new state
            if !p.is_high {
                is_on[to_idx] = !is_on[to_idx];
                to_check.extend(graph.edges_from(&p.to).into_iter().map(|n| Pulse {
                    from: p.to,
                    to: n,
                    is_high: is_on[to_idx],
                }));
            }
        } else {
            // a conjunction updates the received signal and emits a NAND
            let from_idx = graph.node_idx(&p.from);
            let conj_signals = last_signals.get_mut(&to_idx).unwrap();
            let sig_pos = conj_signals
                .iter()
                .position(|(f, _)| *f == from_idx)
                .unwrap();
            conj_signals[sig_pos].1 = p.is_high;
            let emit = !conj_signals.iter().all(|(_, s)| *s);
            to_check.extend(graph.edges_from(&p.to).into_iter().map(|n| Pulse {
                from: p.to,
                to: n,
                is_high: emit,
            }));
        }
    }
    (low_pulses, high_pulses)
}

fn prob1(input: &[&str]) -> u64 {
    let (fliflops, graph, mut is_on, mut last_signals) = prepare_prob1(input);
    let mut low_pulses = 0u64;
    let mut high_pulses = 0u64;
    let mut loops = 0;
    for _ in 0..1000 {
        let (olp, ohp) = once(&graph, &fliflops, &mut is_on, &mut last_signals, &mut loops);
        low_pulses += olp;
        high_pulses += ohp;
    }
    assert_eq!(loops, 1000);
    low_pulses * high_pulses
}

fn prepare_prob1<'a>(
    input: &[&'a str],
) -> (
    Vec<bool>,
    Graph<&'a str>,
    Vec<bool>,
    HashMap<usize, Vec<(usize, bool)>>,
) {
    let mut nodes: Vec<&str> = vec![];
    let mut points: Vec<Vec<&str>> = vec![];
    for line in input {
        let (n, p) = line.split_once(" -> ").unwrap();
        nodes.push(n);
        points.push(p.split(", ").map(|s| s.trim_start()).collect());
    }
    let mut fliflops: Vec<bool> = nodes.iter().map(|n| n.starts_with('%')).collect();
    let mut nodes: Vec<&str> = nodes
        .iter()
        .map(|n| n.trim_start_matches(['%', '&']))
        .collect();

    let mut graph: Graph<&str> = Graph::new_with_nodes(nodes.clone());
    let mut added_nodes: Vec<&str> = vec![];
    for (n, ps) in nodes.iter().zip(points.iter()) {
        for p in ps {
            if !graph.nodes.contains(p) {
                graph.add_node(p);
                fliflops.push(true);
                added_nodes.push(p);
            }
            graph.add_edge(n, p);
        }
    }
    nodes.extend(added_nodes.iter());
    let is_on: Vec<bool> = vec![false; nodes.len()];
    let last_signals = get_last_signals(&fliflops, &graph);
    (fliflops, graph, is_on, last_signals)
}

fn get_last_signals(
    fliflops: &Vec<bool>,
    graph: &Graph<&str>,
) -> HashMap<usize, Vec<(usize, bool)>> {
    fliflops
        .iter()
        .enumerate()
        .filter(|(_, &f)| !f) // retain conjunctions
        .map(|(i, _)| {
            (i, {
                let n = graph.nodes[i];
                let et = graph.edges_to(&n);
                et.iter().map(|f| (graph.node_idx(f), false)).collect()
            })
        })
        .collect()
}

fn subraph<'a>(graph: &Graph<&'a str>, nodes: &Vec<&'a str>) -> (Graph<&'a str>, Vec<usize>) {
    let sub_idxs: Vec<usize> = nodes.iter().map(|n| graph.node_idx(&n)).collect();
    let mut new_graph: Graph<&str> = Graph::new_with_nodes(nodes.iter().cloned());
    for &frm_idxs in sub_idxs.iter() {
        graph
            .edges_from_idxs(frm_idxs)
            .iter()
            .filter(|n| sub_idxs.contains(*n))
            .for_each(|n| {
                new_graph.add_edge(&graph.nodes[frm_idxs], &graph.nodes[*n]);
            });
    }
    (new_graph, sub_idxs)
}

fn as_u32(
    is_on: &[bool],
    fliflops: &[bool],
    last_signals: &HashMap<usize, Vec<(usize, bool)>>,
) -> u32 {
    let mut result = 0u32;
    let mut i = 0u8;
    for (bidx, &b) in is_on.iter().enumerate() {
        if fliflops[bidx] {
            if b {
                result += 1 << i;
            }
            i += 1;
        }
    }
    let mut lsk: Vec<usize> = last_signals.keys().cloned().collect();
    lsk.sort();
    for k in lsk {
        let mut lsv = last_signals.get(&k).unwrap().clone();
        lsv.sort_by_key(|&(n, _)| n);
        for (_, b) in lsv.iter() {
            if *b {
                result += 1 << i;
            }
            i += 1;
        }
    }

    result
}

fn prob2(input: &[&str]) -> u64 {
    let (fliflops, graph, _, _) = prepare_prob1(input);

    let distances = graph.all_distances();
    let to_rx = graph.edges_to(&"rx");
    assert_eq!(to_rx.len(), 1);
    let prev_rx = to_rx[0];
    let prev_prev_rx = graph.edges_to(&prev_rx);
    let mut broad_to_prevs: Vec<Vec<&str>> = prev_prev_rx
        .iter()
        .map(|&t| {
            (0..graph.len())
                .filter(|&f| distances[f][graph.node_idx(&t)] < usize::MAX)
                .map(|n| graph.nodes[n])
                .collect::<Vec<&str>>()
        })
        .collect();
    // checked that the previous ones do not intersect except in broadcaster

    let mut on_at: Vec<HashSet<i64>> = vec![];
    let mut moduli: Vec<u64> = vec![];
    for (i, sub_graph_lbls) in broad_to_prevs.iter_mut().enumerate() {
        let two_to_rx = prev_prev_rx[i];
        sub_graph_lbls.push(two_to_rx);
        sub_graph_lbls.push(prev_rx);
        sub_graph_lbls.push(&"rx");

        let (sub_graph, labels_map) = subraph(&graph, sub_graph_lbls);
        let sub_fliflops: Vec<bool> = labels_map.iter().map(|&i| fliflops[i]).collect();
        let mut sub_is_on: Vec<bool> = vec![false; sub_graph.len()];
        let mut sub_last_signals = get_last_signals(&sub_fliflops, &sub_graph);

        let rx_idx = sub_graph.node_idx(&"rx");
        let mut checking_fliflops: Vec<bool> = sub_fliflops.clone();
        checking_fliflops[rx_idx] = false;

        let mut wanted_was_on: HashSet<i64> = HashSet::new();
        let mut loops = 0u64;
        let mut seen: HashMap<u32, u64> = HashMap::from([(
            as_u32(&sub_is_on, &checking_fliflops, &sub_last_signals),
            0u64,
        )]);
        let mut previous: Option<u64> = None;
        while previous.is_none() {
            once(
                &sub_graph,
                &sub_fliflops,
                &mut sub_is_on,
                &mut sub_last_signals,
                &mut loops,
            );
            if sub_is_on[rx_idx] {
                wanted_was_on.insert(loops as i64);
            }
            previous = seen.insert(
                as_u32(&sub_is_on, &checking_fliflops, &sub_last_signals),
                loops,
            );
        }
        println!(
            "prev: {}, previous: {}, current: {}, on: {:?}",
            two_to_rx,
            previous.unwrap(),
            loops,
            wanted_was_on
        );
        moduli.push(loops - previous.unwrap());
        on_at.push(
            wanted_was_on
                .iter()
                .filter(|&&l| l >= previous.unwrap() as i64)
                .map(|&l| l)
                .collect(),
        );
    }
    let po = iproduct![
        on_at[0].iter(),
        on_at[1].iter(),
        on_at[2].iter(),
        on_at[3].iter()
    ];
    println!(
        "to find: {:?} x {:?} x {:?} x {:?}. Moduli: {:?}",
        on_at[0], on_at[1], on_at[2], on_at[3], moduli
    );
    let min_values: Vec<u64> = on_at
        .iter()
        .map(|v| *(v.iter().min().unwrap()) as u64)
        .collect();
    po.map(|rems| {
        let rems: Vec<i64> = vec![*rems.0, *rems.1, *rems.2, *rems.3];
        chinese_reminder_with_min_condition(&rems, moduli.as_slice(), min_values.as_slice())
            .unwrap()
    })
    .min()
    .unwrap() as u64
}

fn chinese_reminder_with_min_condition(
    reminders: &[i64],
    moduli: &[u64],
    min_values: &[u64],
) -> Option<u64> {
    let result = chinese_reminder_theorem(reminders, moduli)?;
    let lcm = moduli.iter().fold(1, |a, &b| a.lcm(&b));
    let min_allowed: u64 = min_values.iter().map(|&m| m).max().unwrap();
    Some(
        (0..min_allowed)
            .map(|q| result + q * lcm)
            .filter(|&r| r >= min_allowed)
            .next()
            .unwrap() as u64,
    )
}

pub fn main() {
    let input: Vec<&str> = include_str!("../day_20_input").trim().split('\n').collect();
    println!("prob1: {}", prob1(&input));
    println!("prob2: {}", prob2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    fn example0() -> Vec<&'static str> {
        vec![
            "%a -> b",
            "%b -> c",
            "broadcaster -> a, b, c",
            "%c -> inv",
            "&inv -> a",
        ]
    }

    fn example1() -> Vec<&'static str> {
        vec![
            "broadcaster -> a",
            "%a -> inv, con",
            "&inv -> b",
            "%b -> con",
            "&con -> output",
        ]
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(&example0()), 32000000);
        assert_eq!(prob1(&example1()), 11687500);
    }

    #[test]
    fn test_prepare() {
        let (fliflops, graph, is_on, last_signals) = prepare_prob1(&example0());
        assert_eq!(graph.nodes, vec!["a", "b", "broadcaster", "c", "inv"]);
        assert_eq!(fliflops, vec![true, true, false, true, false]);
        let mut ef = graph.edges_from(&"broadcaster");
        ef.sort();
        assert_eq!(ef, vec!["a", "b", "c"]);
        assert_eq!(graph.edges_to(&"inv"), vec!["c"]);
        assert_eq!(is_on, vec![false, false, false, false, false]);
        assert_eq!(last_signals[&graph.node_idx(&"inv")], vec![(3, false)]);

        let (fliflops, graph, is_on, last_signals) = prepare_prob1(&example1());
        assert_eq!(
            graph.nodes,
            vec!["broadcaster", "a", "inv", "b", "con", "output"]
        );
        assert_eq!(fliflops, vec![false, true, false, true, false, true]);
        assert_eq!(is_on, vec![false, false, false, false, false, false]);
        assert_eq!(last_signals[&graph.node_idx(&"inv")], vec![(1, false)]);
        let mut ls = last_signals.get(&graph.node_idx(&"con")).unwrap().clone();
        ls.sort();
        assert_eq!(ls, vec![(1, false), (3, false)]);
    }

    #[test]
    fn test_once() {
        let (fliflops, graph, mut is_on, mut last_signals) = prepare_prob1(&example0());
        let mut loops = 0u64;
        let (lp, hp) = once(&graph, &fliflops, &mut is_on, &mut last_signals, &mut loops);
        assert_eq!(lp, 8);
        assert_eq!(hp, 4);
        assert_eq!(loops, 1);
        let (fliflops, graph, mut is_on, mut last_signals) = prepare_prob1(&example1());
        assert_eq!(
            once(&graph, &fliflops, &mut is_on, &mut last_signals, &mut loops),
            (4, 4)
        );
        assert_eq!(
            once(&graph, &fliflops, &mut is_on, &mut last_signals, &mut loops),
            (4, 2)
        );
        assert_eq!(
            once(&graph, &fliflops, &mut is_on, &mut last_signals, &mut loops),
            (5, 3)
        );
        assert_eq!(
            once(&graph, &fliflops, &mut is_on, &mut last_signals, &mut loops),
            (4, 2)
        );
    }

    fn real_example() -> Vec<&'static str> {
        vec![
            "%tr -> rm",
            "%lc -> hr",
            "%rm -> pf, ml",
            "%sx -> qc, pf",
            "%qc -> pf",
            "broadcaster -> sr",
            "%xq -> dj",
            "%zd -> pf, lc",
            "%hr -> pm",
            "%ml -> pf, xq",
            "%sr -> pf, zd",
            "%pm -> tr",
            "%dj -> pf, sx",
            "&pf -> tr, hr, zf, sr, xq, pm, lc",
            "&zf -> rg",
            "&rg -> rx",
        ]
    }
    #[test]
    fn test_as_u32() {
        let subgraph: Vec<&str> = real_example();
        let (fliflops, _, is_on, mut last_signals) = prepare_prob1(&subgraph.as_slice());
        assert_eq!(as_u32(&is_on, &fliflops, &last_signals), 0);
        let f = last_signals.get_mut(&13).unwrap();
        f[0].1 = true;
        println!("{:?}", last_signals.clone());
        assert!(as_u32(&is_on, &fliflops, &last_signals) != 0);
        let f = last_signals.get_mut(&13).unwrap();
        f[0].1 = false;
        assert!(as_u32(&is_on, &fliflops, &last_signals) == 0);
    }
}
