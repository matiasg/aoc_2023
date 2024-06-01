use std::collections::{BinaryHeap, HashMap};
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn iter() -> impl Iterator<Item = Self> {
        [Self::North, Self::East, Self::South, Self::West]
            .iter()
            .copied()
    }
    fn following(&self, x: isize, y: isize) -> (isize, isize) {
        match self {
            Self::North => (x, y - 1),
            Self::East => (x + 1, y),
            Self::South => (x, y + 1),
            Self::West => (x - 1, y),
        }
    }
}

impl fmt::Debug for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::North => "N",
                Direction::East => "E",
                Direction::South => "S",
                Direction::West => "W",
            }
        )
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct DirPos {
    dir: Direction,
    pos: (isize, isize),
}

impl DirPos {
    fn new(dir: Direction, i: isize, j: isize) -> Self {
        Self { dir, pos: (i, j) }
    }
    fn following(&self, input_u32: &Vec<Vec<u32>>) -> Vec<Self> {
        let max_pos = (input_u32.len() as isize, input_u32[0].len() as isize);
        let mut result = Vec::new();
        for dir in Direction::iter() {
            if dir == self.dir {
                continue;
            }
            let extension: Vec<Self> = match dir {
                Direction::North => {
                    let start_i = 0.max(self.pos.0 - 10);
                    let stop_i = 0.max(self.pos.0 - 3);
                    (start_i..stop_i)
                        .map(|i| DirPos {
                            dir,
                            pos: (i, self.pos.1),
                        })
                        .collect()
                }
                Direction::East => {
                    let start_j = max_pos.1.min(self.pos.1 + 4);
                    let stop_j = max_pos.1.min(self.pos.1 + 11);
                    (start_j..stop_j)
                        .map(|j| DirPos {
                            dir,
                            pos: (self.pos.0, j),
                        })
                        .collect()
                }
                Direction::South => {
                    let start_i = max_pos.0.min(self.pos.0 + 4);
                    let stop_i = max_pos.0.min(self.pos.0 + 11);
                    (start_i..stop_i)
                        .map(|i| DirPos {
                            dir,
                            pos: (i, self.pos.1),
                        })
                        .collect()
                }
                Direction::West => {
                    let start_j = 0.max(self.pos.1 - 10);
                    let stop_j = 0.max(self.pos.1 - 3);
                    (start_j..stop_j)
                        .map(|j| DirPos {
                            dir,
                            pos: (self.pos.0, j),
                        })
                        .collect()
                }
            };
            result.extend(extension);
        }
        result
    }

    fn cmp(&self, other: &DirPos) -> std::cmp::Ordering {
        self.pos
            .0
            .cmp(&other.pos.0)
            .then_with(|| self.pos.1.cmp(&other.pos.1))
            .then_with(|| self.dir.cmp(&other.dir))
    }
}

#[derive(PartialEq, Eq, Debug)]
struct DirPosCost {
    dp: DirPos,
    cost: u32,
}
impl DirPos {
    fn get_until(&self, next: &DirPos, input_u32: &[Vec<u32>]) -> u32 {
        if next.pos.0 < self.pos.0 {
            return (next.pos.0..self.pos.0)
                .map(|i| input_u32[i as usize][next.pos.1 as usize])
                .sum();
        };
        if next.pos.0 > self.pos.0 {
            return (self.pos.0 + 1..=next.pos.0)
                .map(|i| input_u32[i as usize][next.pos.1 as usize])
                .sum();
        }
        if next.pos.1 < self.pos.1 {
            return (next.pos.1..self.pos.1)
                .map(|j| input_u32[next.pos.0 as usize][j as usize])
                .sum();
        }
        if next.pos.1 > self.pos.1 {
            return (self.pos.1 + 1..=next.pos.1)
                .map(|j| input_u32[next.pos.0 as usize][j as usize])
                .sum();
        }
        0
    }
}

impl Ord for DirPosCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.dp.cmp(&other.dp))
    }
}

impl PartialOrd for DirPosCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct ConsecutiveDir {
    dir: Direction,
    consecutive: isize,
}

impl ConsecutiveDir {
    fn following(&self) -> Vec<Self> {
        let mut res: Vec<Self> = Vec::new();
        for dir in Direction::iter() {
            if dir != self.dir || self.consecutive < 3 {
                let j = if dir == self.dir {
                    self.consecutive + 1
                } else {
                    1
                };
                res.push(Self {
                    dir,
                    consecutive: j,
                });
            }
        }
        res
    }
}

impl fmt::Debug for ConsecutiveDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{}", self.dir, self.consecutive)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct CDPos {
    cd: ConsecutiveDir,
    x: isize,
    y: isize,
}

impl Ord for CDPos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cd
            .consecutive
            .cmp(&other.cd.consecutive)
            .then_with(|| self.y.cmp(&other.y))
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.cd.dir.cmp(&other.cd.dir))
    }
}

impl PartialOrd for CDPos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, Debug)]
struct CDPosWithCost {
    cdp: CDPos,
    cost: u32,
}

impl CDPosWithCost {
    fn new(dir: Direction, consecutive: isize, x: isize, y: isize, cost: u32) -> Self {
        Self {
            cdp: CDPos {
                cd: ConsecutiveDir { dir, consecutive },
                x,
                y,
            },
            cost,
        }
    }
}

impl Ord for CDPosWithCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.cdp.cmp(&other.cdp))
    }
}

impl PartialOrd for CDPosWithCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl CDPosWithCost {
    fn start_heap() -> BinaryHeap<CDPosWithCost> {
        let mut heap: BinaryHeap<CDPosWithCost> = BinaryHeap::new();
        heap.push(CDPosWithCost {
            cdp: CDPos::start(),
            cost: 0,
        });
        heap
    }
}

impl CDPos {
    fn inside(&self, input: &Vec<&str>) -> bool {
        self.x >= 0
            && self.x < input[0].len() as isize
            && self.y >= 0
            && self.y < input.len() as isize
    }

    fn following(&self, input: &Vec<&str>) -> Vec<Self> {
        self.cd
            .following()
            .into_iter()
            .map(|cdp| {
                let (x, y) = cdp.dir.following(self.x, self.y);
                CDPos { cd: cdp, x, y }
            })
            .filter(|cdp| cdp.inside(input))
            .collect()
    }

    fn get(&self, input_u32: &Vec<Vec<u32>>) -> u32 {
        input_u32[self.y as usize][self.x as usize]
    }

    fn value_in(&self, graph: &HashMap<Direction, Vec<Vec<Vec<u32>>>>) -> u32 {
        graph[&self.cd.dir][self.cd.consecutive.max(1) as usize - 1][self.y as usize]
            [self.x as usize]
    }

    fn start() -> Self {
        Self {
            cd: {
                ConsecutiveDir {
                    dir: Direction::East,
                    consecutive: 0,
                }
            },
            x: 0,
            y: 0,
        }
    }
}

impl fmt::Debug for CDPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{}) {:?}", self.y, self.x, self.cd)
    }
}

fn make_graph(input: &Vec<&str>) -> HashMap<Direction, Vec<Vec<Vec<u32>>>> {
    let rows = input.len();
    let cols = input[0].len();
    Direction::iter()
        .map(|d| (d, vec![vec![vec![u32::MAX; cols]; rows]; 3]))
        .collect()
}

fn prob1(input: &Vec<&str>) -> u32 {
    let input_u32 = convert_to_u32(input);
    let mut graph = make_graph(input);
    let mut least_cost = CDPosWithCost::start_heap();
    while let Some(cdpwc) = least_cost.pop() {
        if cdpwc.cost > cdpwc.cdp.value_in(&graph) {
            continue;
        }
        for next in cdpwc.cdp.following(input) {
            let posible_next_cost = cdpwc.cost + next.get(&input_u32);
            if posible_next_cost < next.value_in(&graph) {
                graph.get_mut(&next.cd.dir).unwrap()[next.cd.consecutive as usize - 1]
                    [next.y as usize][next.x as usize] = posible_next_cost;
                least_cost.push(CDPosWithCost::new(
                    next.cd.dir,
                    next.cd.consecutive,
                    next.x,
                    next.y,
                    posible_next_cost,
                ));
            }
        }
    }
    let last_row = input.len() - 1;
    let last_col = input[0].len() - 1;
    graph
        .values()
        .map(|dcg| dcg.iter().map(|g| g[last_row][last_col]).min().unwrap())
        .min()
        .unwrap()
}

fn prob2(input: &Vec<&str>) -> u32 {
    let input_u32 = convert_to_u32(input);
    let stop_pos = (input.len() as isize, input[0].len() as isize);
    let mut costs: HashMap<Direction, Vec<Vec<u32>>> = Direction::iter()
        .map(|dir| {
            (
                dir,
                vec![vec![u32::MAX; stop_pos.1 as usize]; stop_pos.0 as usize],
            )
        })
        .collect();
    let mut costs_heap: BinaryHeap<DirPosCost> = BinaryHeap::new();
    costs_heap.push(DirPosCost {
        dp: DirPos::new(Direction::North, 0, 0),
        cost: 0,
    });
    while let Some(dpc) = costs_heap.pop() {
        for next in dpc.dp.following(&input_u32) {
            let posible_next_cost = dpc.cost + dpc.dp.get_until(&next, &input_u32);
            if posible_next_cost < costs[&next.dir][next.pos.0 as usize][next.pos.1 as usize] {
                costs.get_mut(&next.dir).unwrap()[next.pos.0 as usize][next.pos.1 as usize] =
                    posible_next_cost;
                costs_heap.push(DirPosCost {
                    dp: next.clone(),
                    cost: posible_next_cost,
                });
            }
        }
    }
    costs
        .values()
        .map(|dc| dc[stop_pos.0 as usize - 1][stop_pos.1 as usize - 1])
        .min()
        .unwrap()
}

fn convert_to_u32(input: &[&str]) -> Vec<Vec<u32>> {
    input
        .iter()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u32).collect())
        .collect()
}

pub fn main() {
    let input = include_str!("../day_17_input").lines().collect();
    println!("prob1: {}", prob1(&input));
    println!("prob2: {}", prob2(&input));
}

#[cfg(test)]
mod tests {
    use std::collections::BinaryHeap;

    use crate::day_17::{
        convert_to_u32, prob1, prob2, CDPos, CDPosWithCost, ConsecutiveDir, DirPos, Direction,
    };

    fn example0() -> Vec<&'static str> {
        vec!["12", "34"]
    }

    fn example() -> Vec<&'static str> {
        vec![
            "2413432311323",
            "3215453535623",
            "3255245654254",
            "3446585845452",
            "4546657867536",
            "1438598798454",
            "4457876987766",
            "3637877979653",
            "4654967986887",
            "4564679986453",
            "1224686865563",
            "2546548887735",
            "4322674655533",
        ]
    }

    fn example2() -> Vec<&'static str> {
        vec![
            "111111111111",
            "999999999991",
            "999999999991",
            "999999999991",
            "999999999991",
        ]
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(&example0()), 6);
        assert_eq!(prob1(&example()), 101);
    }

    #[test]
    fn test_consecutive() {
        let dc = ConsecutiveDir {
            dir: Direction::East,
            consecutive: 3,
        };
        assert_eq!(dc.following().len(), 3);
        assert!(dc.following().iter().all(|fdc| fdc.dir != Direction::East));
    }

    #[test]
    fn test_orders() {
        assert!(Direction::East < Direction::South);
        assert!(Direction::North != Direction::West);
        let cd1 = ConsecutiveDir {
            dir: Direction::East,
            consecutive: 2,
        };
        let cd2 = ConsecutiveDir {
            dir: Direction::East,
            consecutive: 3,
        };
        let cdp1 = CDPos {
            cd: cd1,
            x: 0,
            y: 0,
        };
        let cdp2 = CDPos {
            cd: cd2,
            x: 0,
            y: 0,
        };
        let cdp3 = CDPos {
            cd: cd1,
            x: 0,
            y: 1,
        };
        assert!(cdp1 < cdp2);
        assert!(cdp1 < cdp3);
        let cdpc1 = CDPosWithCost { cdp: cdp1, cost: 0 };
        let cdpc2 = CDPosWithCost { cdp: cdp1, cost: 1 };
        assert!(cdpc1 > cdpc2);
    }

    #[test]
    fn test_following() {
        let mut cdp = CDPos {
            cd: ConsecutiveDir {
                dir: Direction::East,
                consecutive: 3,
            },
            x: 0,
            y: 0,
        };
        assert_eq!(cdp.following(&example()).len(), 1);
        cdp.cd.consecutive = 2;
        assert_eq!(cdp.following(&example()).len(), 2);
        cdp.x = 12;
        assert_eq!(cdp.following(&example()).len(), 2);
    }

    #[test]
    fn test_binheap() {
        let cdpwc1 = CDPosWithCost::new(Direction::East, 1, 0, 0, 0);
        let cdpwc2 = CDPosWithCost::new(Direction::East, 1, 0, 0, 1);
        let cdpwc3 = CDPosWithCost::new(Direction::East, 3, 0, 0, 1);
        let cdpwc4 = CDPosWithCost::new(Direction::East, 1, 1, 0, 1);
        let mut bh = BinaryHeap::from(vec![&cdpwc2, &cdpwc4, &cdpwc1, &cdpwc3]);
        assert_eq!(bh.pop().unwrap(), &cdpwc1);
        assert_eq!(bh.pop().unwrap(), &cdpwc3);
        assert_eq!(bh.pop().unwrap(), &cdpwc4);
        assert_eq!(bh.pop().unwrap(), &cdpwc2);
        assert!(bh.is_empty());
    }

    #[test]
    fn test_prob2() {
        assert_eq!(prob2(&example()), 94);
        assert_eq!(prob2(&example2()), 23);
    }

    #[test]
    fn test_get_until() {
        let iu32 = convert_to_u32(&example());
        let dp1 = DirPos::new(Direction::East, 1, 3);
        let dp2 = DirPos::new(Direction::East, 1, 8);
        assert_eq!(dp1.get_until(&dp2, &iu32), 20);
        assert_eq!(dp2.get_until(&dp1, &iu32), 22);
        let dp3 = DirPos::new(Direction::East, 3, 3);
        assert_eq!(dp1.get_until(&dp3, &iu32), 11);
    }

    #[test]
    fn test_following2() {
        let iu32 = convert_to_u32(&example());
        let dp1 = DirPos::new(Direction::South, 4, 8);
        let following = dp1.following(&iu32);
        assert!(following.contains(&DirPos::new(Direction::North, 0, 8)));
        assert!(following.contains(&DirPos::new(Direction::East, 4, 12)));
        assert_eq!(following.len(), 7);
    }
}
