use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn make_map<'a>(input: &'a [&'a str]) -> HashMap<&'a str, (&'a str, &'a str)> {
    let re = Regex::new(r"([A-Z][A-Z][A-Z]) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();
    HashMap::from_iter(input.get(2..).unwrap().iter().map(|l| {
        let c = re.captures(l).unwrap();
        (
            c.get(1).unwrap().as_str(),
            (c.get(2).unwrap().as_str(), c.get(3).unwrap().as_str()),
        )
    }))
}

fn step_until<'a>(
    map: &HashMap<&'a str, (&'a str, &'a str)>,
    instructions: &[u8],
    start: &'a str,
    start_instruction_idx: usize,
    end_condition: fn(&'a str) -> bool,
    must_make_steps: bool,
) -> (usize, &'a str) {
    let mut step: usize = 0;
    let mut state = start;
    let stop_step = map.len() * instructions.len() + 1;
    while !end_condition(state) & (step < stop_step) | ((step == 0) & must_make_steps) {
        let i = instructions[(step + start_instruction_idx) % instructions.len()];
        let v = map.get(state).unwrap();
        state = if i == b'L' { v.0 } else { v.1 };
        step += 1;
    }
    (step, state)
}

fn prob1(input: Vec<&str>) -> usize {
    let instructions = input.first().unwrap().as_bytes();
    let map = make_map(&input);
    step_until(&map, instructions, "AAA", 0, |s| s == "ZZZ", false).0
}

fn make_graph(
    map: &HashMap<&str, (&str, &str)>,
    states: &[&str],
    instructions: &[u8],
) -> Vec<Vec<(usize, usize)>> {
    let mut ret: Vec<Vec<(usize, usize)>> = vec![vec![(0, 0); instructions.len()]; states.len()];
    for (i, key) in states.iter().enumerate() {
        for (j, _) in instructions.iter().enumerate() {
            let (steps, end_state) =
                step_until(map, instructions, key, j, |s| s.ends_with('Z'), true);
            let goes_to = states.iter().position(|&s| s == end_state).unwrap_or(i);
            ret[i][j] = (goes_to, steps);
        }
    }
    ret
}

fn prob2_alt(input: Vec<&str>) -> usize {
    // This takes about 16 minutes compiled with --release
    // It can be done faster by computing length of cycles of the
    // graph. But that would make the code even more difficult.
    let map = make_map(&input);
    let start_states: Vec<&str> = input[2..]
        .iter()
        .filter(|&s| s.as_bytes()[2] == b'A')
        .map(|&s| s.get(..3).unwrap())
        .collect();
    let end_states: Vec<&str> = input[2..]
        .iter()
        .filter(|&s| s.as_bytes()[2] == b'Z')
        .map(|&s| s.get(..3).unwrap())
        .collect();
    let instructions = input[0].as_bytes().to_vec();
    let ends_in_z: fn(&str) -> bool = |s| s.ends_with('Z');
    let mut states_steps: HashMap<&str, (usize, &str)> = HashMap::new();
    for state in start_states {
        states_steps.insert(
            state,
            step_until(&map, &instructions, state, 0, ends_in_z, true),
        );
    }
    let graph = make_graph(&map, &end_states, &instructions);
    let mut steps_values: HashSet<usize> = states_steps.values().map(|&(s, _)| s).collect();
    while steps_values.len() != 1 {
        let (min_steps_state, (steps, end_state)) =
            states_steps.iter().min_by_key(|(_, (s, _))| s).unwrap();

        let i = end_states.iter().position(|s| s == end_state).unwrap();
        let j = steps % instructions.len();
        let (next_state_idx, added_steps) = graph[i][j];

        states_steps.insert(
            min_steps_state,
            (steps + added_steps, end_states[next_state_idx]),
        );

        steps_values = states_steps.values().map(|&(s, _)| s).collect();
    }
    let ret = steps_values.drain().next().unwrap();
    ret
}

fn _prob2(input: Vec<&str>) -> usize {
    // This takes too much time. The answer is ~1e14 and this goes one by one
    let instructions = input.first().unwrap().as_bytes();
    let map = make_map(&input);
    let mut step: usize = 0;
    let mut states: Vec<&str> = map.keys().filter(|s| s.ends_with('A')).copied().collect();
    while !states.iter().all(|s| s.ends_with('Z')) {
        let i: for<'a> fn((&'a str, &'a str)) -> &'a str =
            if instructions[step % instructions.len()] == b'L' {
                |s| s.0
            } else {
                |s| s.1
            };
        states = states.iter().map(|s| i(*map.get(s).unwrap())).collect();
        step += 1;
        // if step % 1000 == 0 {
        if states.iter().filter(|s| s.ends_with('Z')).count() > 2 {
            println!(
                "{}: {:?}",
                step,
                std::str::from_utf8(&states.iter().map(|s| s.as_bytes()[2]).collect::<Vec<u8>>())
            )
        }
    }
    step
}

pub fn main() {
    let input = fs::read_to_string("day_8_input").expect("Error reading file");
    let input: Vec<&str> = input.trim().split('\n').collect();
    let p1 = prob1(input.clone());
    println!("problem 1: {}", p1);
    let p2 = prob2_alt(input);
    println!("problem 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use crate::day_8::{_prob2, make_graph, make_map, prob1, prob2_alt};

    fn example1() -> Vec<&'static str> {
        vec![
            "RL",
            "",
            "AAA = (BBB, CCC)",
            "BBB = (DDD, EEE)",
            "CCC = (ZZZ, GGG)",
            "DDD = (DDD, DDD)",
            "EEE = (EEE, EEE)",
            "GGG = (GGG, GGG)",
            "ZZZ = (ZZZ, ZZZ)",
        ]
    }
    fn example2() -> Vec<&'static str> {
        vec![
            "LLR",
            "",
            "AAA = (BBB, BBB)",
            "BBB = (AAA, ZZZ)",
            "ZZZ = (ZZZ, ZZZ)",
        ]
    }
    fn example3() -> Vec<&'static str> {
        vec![
            "LR",
            "",
            "EEA = (EEB, XXX)",
            "EEB = (XXX, EEZ)",
            "EEZ = (EEB, XXX)",
            "FFA = (FFB, XXX)",
            "FFB = (FFC, FFC)",
            "FFC = (FFZ, FFZ)",
            "FFZ = (FFB, FFB)",
            "XXX = (XXX, XXX)",
        ]
    }

    #[test]
    fn test_make_map() {
        let ex = example1();
        let m = make_map(&ex);
        assert_eq!(m.len(), 7);
        assert_eq!(m["AAA"], ("BBB", "CCC"));
    }

    #[test]
    fn test_main() {
        assert_eq!(prob1(example1()), 2);
        assert_eq!(prob1(example2()), 6);
        assert_eq!(_prob2(example3()), 6);
        assert_eq!(prob2_alt(example3()), 6);
    }

    #[test]
    fn test_make_graph() {
        let ex = example3();
        let m = make_map(&ex);
        let instructions = ex[0].as_bytes().to_vec();
        let states: Vec<&str> = example3()[2..]
            .iter()
            .filter(|&s| s.as_bytes()[2] == b'Z')
            .map(|&s| s.get(..3).unwrap())
            .collect();
        let graph = make_graph(&m, &states, &instructions);
        assert_eq!(graph.len(), 2);
        assert_eq!(graph[0].len(), 2);
        assert_eq!(graph[1].len(), 2);
        assert_eq!(graph[0][0], (0, 2));
        assert_eq!(graph[1][0], (1, 3));
        assert_eq!(graph[1][1], (1, 3));
    }
}
