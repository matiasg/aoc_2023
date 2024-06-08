use std::collections::HashMap;

use graph::Graph;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
struct State {
    values: [u64; 4],
}

impl State {
    fn from(line: &'static str) -> Self {
        // "{x=787,m=2655,a=1222,s=2876}",
        let values: Vec<u64> = line
            .get(1..line.len() - 1)
            .unwrap()
            .split(",")
            .map(|s| s[2..].parse::<u64>().unwrap())
            .collect();
        Self {
            values: values.try_into().unwrap(),
        }
    }
    fn total_value(self) -> u64 {
        self.values.iter().sum()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Continuation {
    Reject,
    Accept,
    Goto(&'static str),
    NextInstruction,
}

impl Continuation {
    fn from(c: &'static str) -> Self {
        match c {
            "R" => Continuation::Reject,
            "A" => Continuation::Accept,
            x => Continuation::Goto(x),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    box_: usize,
    less_than: bool,
    comp_value: u64,
    then: Continuation,
}

impl Instruction {
    fn from(ins: &'static str) -> Self {
        let re = Regex::new(r"([xmas])([<>])(\d+):([a-zAR]+)$").unwrap();
        let c = re.captures(ins).unwrap();
        let box_ = match c.get(1).unwrap().as_str() {
            "x" => 0,
            "m" => 1,
            "a" => 2,
            "s" => 3,
            _ => panic!("Bad input"),
        };
        let less_than = c.get(2).unwrap().as_str() == "<";
        let comp_value = c.get(3).unwrap().as_str().parse::<u64>().unwrap();
        let then = Continuation::from(c.get(4).unwrap().as_str());
        Self {
            box_,
            less_than,
            comp_value,
            then,
        }
    }
    fn satisfies(&self, state: &State) -> bool {
        let st_value = state.values[self.box_];
        if self.less_than {
            return st_value < self.comp_value;
        } else {
            return st_value > self.comp_value;
        }
    }

    fn next(&self, state: &State) -> Continuation {
        if self.satisfies(state) {
            return self.then;
        }
        return Continuation::NextInstruction;
    }

    fn neg(&self) -> Self {
        if self.less_than {
            Self {
                box_: self.box_,
                less_than: false,
                comp_value: self.comp_value - 1,
                then: self.then,
            }
        } else {
            Self {
                box_: self.box_,
                less_than: true,
                comp_value: self.comp_value + 1,
                then: self.then,
            }
        }
    }
}

struct Instructions {
    name: &'static str,
    instructions: Vec<Instruction>,
    otherwise: Continuation,
}

impl Instructions {
    fn from(line: &'static str) -> Self {
        // "px{a<2006:qkq,m>2090:A,rfg}"
        let re = Regex::new(r"^([a-z]+)\{(.+),([a-zAR]+)\}$").unwrap();
        let captures = re.captures(line).unwrap();
        let name = captures.get(1).unwrap().as_str();
        let instructions: Vec<Instruction> = captures
            .get(2)
            .unwrap()
            .as_str()
            .split(",")
            .map(|ins| Instruction::from(ins))
            .collect();
        let otherwise = Continuation::from(captures.get(3).unwrap().as_str());
        Self {
            name,
            instructions,
            otherwise,
        }
    }

    fn next(&self, state: &State) -> Continuation {
        for i in self.instructions.iter() {
            let cont = i.next(state);
            if cont != Continuation::NextInstruction {
                return cont;
            }
        }
        return self.otherwise;
    }

    fn clean(&self) -> Self {
        let mut new_instructions = self.instructions.clone();
        while let Some(ins) = new_instructions.last() {
            if ins.then == self.otherwise {
                new_instructions.pop();
            } else {
                break;
            }
        }
        Self {
            name: self.name,
            instructions: new_instructions,
            otherwise: self.otherwise,
        }
    }
}

struct InstructionSet {
    map: HashMap<&'static str, Instructions>,
}

impl InstructionSet {
    fn from(input: &[&'static str]) -> Self {
        let mut map = HashMap::new();
        for ins_str in input {
            let ins = Instructions::from(ins_str);
            map.insert(ins.name, ins);
        }
        Self { map }
    }

    fn accepts(&self, state: &State) -> bool {
        let mut name = Continuation::Goto("in");
        while let Continuation::Goto(n) = name {
            name = self.map[n].next(state);
        }
        matches!(name, Continuation::Accept)
    }

    fn clean(&mut self) {
        for ins in self.map.values_mut() {
            *ins = ins.clean();
        }
    }
}

fn prob1(input: &[&'static str]) -> u64 {
    let split = get_split(input);
    let insset = InstructionSet::from(&input[0..split]);
    input[split + 1..]
        .iter()
        .map(|&l| State::from(l))
        .filter(|s| insset.accepts(s))
        .map(|s| s.total_value() as u64)
        .sum()
}

fn get_split(input: &[&str]) -> usize {
    let split = input
        .iter()
        .enumerate()
        .filter(|(_, l)| l.is_empty())
        .next()
        .unwrap()
        .0;
    split
}

fn prob2(input: &[&'static str]) -> u64 {
    let split = get_split(input);
    let insset = InstructionSet::from(&input[0..split]);
    let graph = Graph::from(&insset);
    println!("end constructing graph with {} nodes", graph.len());
    let all_paths = graph.paths_between("in", "ACCEPT", &insset, 4000);
    println!("all paths in -> A: {}", all_paths.len());
    let result = Graph::all_cases(&all_paths);
    println!("prob2: {}", result);

    // Previous code: it takes 1 hour by using .clean() and 4 hours without
    // insset.clean();
    // let mut cuts: Vec<Vec<u64>> = vec![vec![1, 4001], vec![1, 4001], vec![1, 4001], vec![1, 4001]];
    // insset.map.values().for_each(|inss| {
    //     inss.instructions.iter().for_each(|ins| {
    //         cuts[ins.box_].push(ins.comp_value as u64);
    //         cuts[ins.box_].push(ins.comp_value + 1 as u64);
    //     })
    // });
    // cuts.iter_mut().for_each(|c| c.sort());
    // cuts.iter_mut().for_each(|c| c.dedup());
    // let mut result: u64 = 0;
    // for (x0, x1) in cuts[0].iter().zip(cuts[0].iter().skip(1)) {
    //     println!("{x0}");
    //     for (m0, m1) in cuts[1].iter().zip(cuts[1].iter().skip(1)) {
    //         for (a0, a1) in cuts[2].iter().zip(cuts[2].iter().skip(1)) {
    //             for (s0, s1) in cuts[3].iter().zip(cuts[3].iter().skip(1)) {
    //                 let state = State {
    //                     values: [*x0, *m0, *a0, *s0],
    //                 };
    //                 if insset.accepts(&state) {
    //                     let size = cuts_size((x0, x1), (m0, m1), (a0, a1), (s0, s1));
    //                     result += size;
    //                 }
    //             }
    //         }
    //     }
    // }
    result
}

fn cuts_size(x: (&u64, &u64), m: (&u64, &u64), a: (&u64, &u64), s: (&u64, &u64)) -> u64 {
    (x.1 - x.0) * (m.1 - m.0) * (a.1 - a.0) * (s.1 - s.0)
}

pub fn main() {
    let input: Vec<&str> = include_str!("../day_19_input").trim().split("\n").collect();
    println!("prob1: {}", prob1(input.as_slice()));
    println!("prob2: {}", prob2(input.as_slice()));
}

mod graph {
    use std::collections::HashMap;

    use super::{Continuation, Instruction, InstructionSet, Instructions};

    #[derive(Debug)]
    pub(super) struct Graph {
        nodes: Vec<&'static str>,
        edges: Vec<Vec<(usize, usize)>>,
    }

    impl Graph {
        fn to_label(cont: &Continuation) -> &'static str {
            match cont {
                Continuation::Reject => "REJECT",
                Continuation::Accept => "ACCEPT",
                Continuation::Goto(label) => label,
                Continuation::NextInstruction => {
                    panic!("next instruction is not possible")
                }
            }
        }
        pub(super) fn from(insset: &InstructionSet) -> Self {
            let mut nodes: Vec<&str> = insset.map.values().map(|ins| ins.name).collect();
            let mut label_to_idx: HashMap<&str, usize> =
                nodes.iter().enumerate().map(|(i, n)| (*n, i)).collect();
            nodes.push("ACCEPT");
            label_to_idx.insert("ACCEPT", nodes.len() - 1);
            nodes.push("REJECT");
            label_to_idx.insert("REJECT", nodes.len() - 1);
            let mut edges: Vec<Vec<(usize, usize)>> = vec![vec![]; nodes.len()];
            for (label, insts) in insset.map.iter() {
                let node_idx = label_to_idx[label];
                for (iidx, ins) in insts.instructions.iter().enumerate() {
                    let next_label = Graph::to_label(&ins.then);
                    edges[node_idx].push((label_to_idx[next_label], iidx));
                }
                let next_label = Graph::to_label(&insts.otherwise);
                edges[node_idx].push((label_to_idx[next_label], insts.instructions.len()));
            }
            Self { nodes, edges }
        }
        pub(super) fn len(&self) -> usize {
            self.nodes.len()
        }
        pub(super) fn edges_from(&self, node: &str) -> Vec<(usize, usize)> {
            self.nodes
                .iter()
                .zip(self.edges.iter())
                .filter(|(&n, _)| n == node)
                .map(|(_, es)| es)
                .next()
                .unwrap()
                .clone()
        }
        pub(super) fn edges_from_labels(&self, node: &str) -> Vec<(&str, usize)> {
            self.edges_from(node)
                .iter()
                .map(|&(n, idx)| (self.nodes[n], idx))
                .collect()
        }

        fn idx_of(&self, node: &str) -> usize {
            self.nodes.iter().position(|&n| n == node).unwrap()
        }

        pub(super) fn paths_between(
            &self,
            start: &str,
            end: &str,
            insset: &InstructionSet,
            max_comp_value: u64,
        ) -> Vec<Vec<Instruction>> {
            let start = self.idx_of(start);
            let start_instructions: Vec<Instruction> = (0..4)
                .map(|b| Instruction {
                    box_: b,
                    less_than: true,
                    comp_value: max_comp_value + 1,
                    then: Continuation::Accept,
                })
                .collect();
            let end = self.idx_of(end);
            let mut visited: Vec<(usize, Vec<Vec<Instruction>>)> =
                vec![(start, vec![start_instructions])];
            let mut result: Vec<Vec<Instruction>> = vec![];

            while let Some((node, visiting_instrs_vec)) = visited.pop() {
                for &(after, afteridx) in self.edges[node].iter() {
                    let mut after_instrs: Vec<Instruction> = vec![];
                    let current_label = self.nodes[node];
                    let current_instrs: &Instructions = insset.map.get(current_label).unwrap();
                    (0..afteridx)
                        .for_each(|i| after_instrs.push(current_instrs.instructions[i].neg()));
                    if afteridx < current_instrs.instructions.len() {
                        after_instrs.push(current_instrs.instructions[afteridx]);
                    }
                    // make a copy of visiting_instrs_vec and add after_instrs to all
                    let after_instrs_vec_to_add: Vec<Vec<Instruction>> = visiting_instrs_vec
                        .iter()
                        .map(|vi| vi.iter().chain(after_instrs.iter()).cloned().collect())
                        .collect();
                    if after == end {
                        result.extend(after_instrs_vec_to_add);
                    } else {
                        match visited.iter_mut().filter(|(n, _)| *n == after).next() {
                            Some((_, ntimes)) => {
                                ntimes.extend(after_instrs_vec_to_add);
                            }
                            None => visited.push((after, after_instrs_vec_to_add)),
                        }
                    }
                }
            }
            result
        }
        pub(super) fn number_of_paths_between(&self, start: &str, end: &str) -> u64 {
            let start = self.idx_of(start);
            let end = self.idx_of(end);
            let mut visited: Vec<(usize, u64)> = vec![(start, 1u64)];
            let mut result = 0u64;
            while let Some((node, times)) = visited.pop() {
                for &(after, _) in self.edges[node].iter() {
                    if after == end {
                        result += times
                    } else {
                        match visited
                            .iter()
                            .enumerate()
                            .filter(|(_, &(n, _))| n == after)
                            .next()
                        {
                            Some((pos, &(n, ntimes))) => visited[pos] = (n, times + ntimes),
                            None => visited.push((after, times)),
                        }
                    }
                }
            }
            result
        }
        pub(crate) fn cases(instructions: &Vec<Instruction>) -> u64 {
            let mut result = 1u64;
            for box_ in 0..4 {
                let mut min_val = 1u64;
                let mut max_val = u64::MAX;
                for i in instructions.iter() {
                    if i.box_ == box_ {
                        if i.less_than {
                            max_val = max_val.min(i.comp_value - 1);
                        } else {
                            min_val = min_val.max(i.comp_value + 1);
                        }
                    }
                }
                result *= (max_val - min_val) + 1;
            }
            result
        }

        pub(crate) fn all_cases(insvec: &Vec<Vec<Instruction>>) -> u64 {
            insvec.iter().map(Graph::cases).sum()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::graph::Graph;
    use crate::day_19::{prob1, prob2, Continuation, InstructionSet, Instructions, State};

    fn example() -> Vec<&'static str> {
        vec![
            "px{a<2006:qkq,m>2090:A,rfg}",
            "pv{a>1716:R,A}",
            "lnx{m>1548:A,A}",
            "rfg{s<537:gd,x>2440:R,A}",
            "qs{s>3448:A,lnx}",
            "qkq{x<1416:A,crn}",
            "crn{x>2662:A,R}",
            "in{s<1351:px,qqz}",
            "qqz{s>2770:qs,m<1801:hdj,R}",
            "gd{a>3333:R,R}",
            "hdj{m>838:A,pv}",
            "",
            "{x=787,m=2655,a=1222,s=2876}",
            "{x=1679,m=44,a=2067,s=496}",
            "{x=2036,m=264,a=79,s=2244}",
            "{x=2461,m=1339,a=466,s=291}",
            "{x=2127,m=1623,a=2188,s=1013}",
        ]
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(&example()), 19114);
    }

    #[test]
    fn test_from() {
        let ins = Instructions::from(example()[0]);
        assert_eq!(ins.name, "px");
        assert_eq!(ins.instructions.len(), 2);

        assert_eq!(ins.instructions[0].box_, 2);
        assert!(ins.instructions[0].less_than);
        assert_eq!(ins.instructions[0].comp_value, 2006);
        assert_eq!(ins.instructions[0].then, Continuation::Goto("qkq"));

        assert_eq!(ins.instructions[1].box_, 1);
        assert!(!ins.instructions[1].less_than);
        assert_eq!(ins.instructions[1].comp_value, 2090);
        assert_eq!(ins.instructions[1].then, Continuation::Accept);

        assert_eq!(ins.otherwise, Continuation::Goto("rfg"));
    }

    #[test]
    fn test_insset() {
        let e = example();
        let insset = InstructionSet::from(&e[0..11]);
        assert_eq!(insset.map.len(), 11);
        assert!(insset.map.contains_key("px"));
        assert!(insset.map.contains_key("hdj"));
    }

    #[test]
    fn test_state_and_next() {
        let e = example();
        let insset = InstructionSet::from(&e[0..11]);
        let state = State::from(e[12]);
        assert_eq!(
            state,
            State {
                values: [787, 2655, 1222, 2876]
            }
        );
        assert_eq!(insset.map["in"].next(&state), Continuation::Goto("qqz"));
        assert_eq!(insset.map["qqz"].next(&state), Continuation::Goto("qs"));
        assert_eq!(insset.map["qs"].next(&state), Continuation::Goto("lnx"));
        assert_eq!(insset.map["lnx"].next(&state), Continuation::Accept);

        assert!(insset.accepts(&state));
    }

    #[test]
    fn test_prob2() {
        assert_eq!(prob2(&example()), 167409079868000);
    }

    #[test]
    fn test_prob2_graph() {
        let insset = InstructionSet::from(&example()[0..11]);
        let graph = Graph::from(&insset);
        let paths = graph.paths_between("in", "ACCEPT", &insset, 4000);
        assert_eq!(paths.len(), 9);
        let cases = Graph::all_cases(&paths);
        assert_eq!(cases, 167409079868000);
    }

    #[test]
    fn test_graph() {
        let insset = InstructionSet::from(&example()[0..11]);
        let graph = Graph::from(&insset);
        assert_eq!(graph.len(), 13);
        assert_eq!(graph.edges_from("crn").len(), 2);
        assert!(graph.edges_from_labels("hdj").contains(&("ACCEPT", 0)));
        assert!(graph.edges_from_labels("hdj").contains(&("pv", 1)));
    }

    #[test]
    fn test_paths() {
        let inst: Vec<&str> = vec!["in{a<1:b,x<3:c,A}", "b{a>1:R,x>0:c,R}", "c{m>1:A,R}"];
        let insset = InstructionSet::from(inst.as_slice());
        let graph = Graph::from(&insset);
        assert_eq!(graph.number_of_paths_between("in", "ACCEPT"), 3);
        assert_eq!(graph.number_of_paths_between("in", "REJECT"), 4);

        let insset = InstructionSet::from(&example()[0..11]);
        let graph = Graph::from(&insset);
        assert_eq!(graph.number_of_paths_between("hdj", "ACCEPT"), 2);
        assert_eq!(graph.number_of_paths_between("qqz", "ACCEPT"), 5);
        assert_eq!(graph.number_of_paths_between("in", "ACCEPT"), 5 + 4);
    }

    #[test]
    fn test_clean() {
        let mut insset = InstructionSet::from(&example()[0..11]);
        insset.clean();
        assert_eq!(insset.map.len(), 11);
        assert!(insset.map.get("lnx").unwrap().instructions.is_empty());
    }

    #[test]
    fn test_paths_between() {
        let inst: Vec<&str> = vec!["in{a<1:b,x<3:c,A}", "b{a>1:R,x>0:c,R}", "c{m>1:A,R}"];
        let insset = InstructionSet::from(inst.as_slice());
        let graph = Graph::from(&insset);
        let paths = graph.paths_between("in", "ACCEPT", &insset, 4000);
        assert_eq!(paths.len(), 3);
        let paths = graph.paths_between("in", "REJECT", &insset, 4000);
        assert_eq!(paths.len(), 4);

        let insset = InstructionSet::from(&example()[0..11]);
        let graph = Graph::from(&insset);
        assert_eq!(graph.paths_between("in", "ACCEPT", &insset, 4000).len(), 9);
    }
}
