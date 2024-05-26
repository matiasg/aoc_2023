fn prob1(lines: &Vec<&str>) -> u32 {
    let lines = make_u8_matrix(lines);
    tilt_north_and_count_load(lines.as_slice())
}

fn tilt_north_and_count_load(lines: &[Vec<u8>]) -> u32 {
    (0..lines[0].len()).map(|j| count_column(&lines, j)).sum()
}

fn count_load_north(lines: &[Vec<u8>]) -> u32 {
    let rows = lines.len() as u32;
    lines
        .iter()
        .enumerate()
        .map(|(i, l)| (rows - i as u32) * l.iter().filter(|&&x| x == 1).count() as u32)
        .sum()
}

fn make_u8_matrix(lines: &Vec<&str>) -> Vec<Vec<u8>> {
    let lines: Vec<Vec<u8>> = lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| {
                    if c == 'O' {
                        1
                    } else if c == '#' {
                        2
                    } else {
                        0
                    }
                })
                .collect()
        })
        .collect();
    lines
}

fn count_column(lines: &[Vec<u8>], j: usize) -> u32 {
    let mut result: u32 = 0;
    let mut weight: u32 = lines.len() as u32;
    for i in 0..lines.len() {
        match lines[i][j] {
            1 => {
                result += weight;
                weight -= 1
            }
            2 => weight = lines.len() as u32 - i as u32 - 1,
            _ => {}
        }
    }
    result
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn outer_iterator(&self, table: &[Vec<u8>]) -> Box<dyn Iterator<Item = isize>> {
        match self {
            Direction::North => Box::new(0..table[0].len() as isize),
            Direction::South => Box::new(0..table[0].len() as isize),
            Direction::East => Box::new(0..table.len() as isize),
            Direction::West => Box::new(0..table.len() as isize),
        }
    }
    fn inner_iterator(&self, table: &[Vec<u8>]) -> Box<dyn DoubleEndedIterator<Item = isize>> {
        match self {
            Direction::North => Box::new(0..table.len() as isize),
            Direction::South => Box::new((0..table.len() as isize).rev()),
            Direction::East => Box::new((0..table[0].len() as isize).rev()),
            Direction::West => Box::new(0..table[0].len() as isize),
        }
    }
    fn inner_next(&self, i: isize) -> isize {
        match self {
            Direction::North => i + 1,
            Direction::South => i - 1,
            Direction::East => i - 1,
            Direction::West => i + 1,
        }
    }

    fn get(&self, table: &[Vec<u8>], i: isize, j: isize) -> u8 {
        let i = usize::try_from(i).unwrap();
        let j = usize::try_from(j).unwrap();
        match self {
            Direction::North | Direction::South => table[j][i],
            Direction::East | Direction::West => table[i][j],
        }
    }

    fn put(&self, table: &mut [Vec<u8>], i: isize, j: isize, v: u8) {
        let i = usize::try_from(i).unwrap();
        let j = usize::try_from(j).unwrap();
        match self {
            Direction::North | Direction::South => table[j][i] = v,
            Direction::East | Direction::West => table[i][j] = v,
        }
    }

    fn inner_until_end(&self, table: &[Vec<u8>], start: isize) -> Box<dyn Iterator<Item = isize>> {
        match self {
            Direction::North => Box::new(start..table.len() as isize),
            Direction::South => Box::new((0..=start).rev()),
            Direction::East => Box::new((0..=start).rev()),
            Direction::West => Box::new(start..table[0].len() as isize),
        }
    }
}

fn move_rocks(intable: &[Vec<u8>], outtable: &mut [Vec<u8>], dir: &Direction) {
    for i in dir.outer_iterator(intable) {
        let mut last_pos = dir.inner_iterator(intable).next().unwrap();
        for j in dir.inner_iterator(intable) {
            match dir.get(intable, i, j) {
                1 => {
                    dir.put(outtable, i, last_pos, 1);
                    last_pos = dir.inner_next(last_pos);
                }
                2 => {
                    while last_pos != j {
                        dir.put(outtable, i, last_pos, 0);
                        last_pos = dir.inner_next(last_pos);
                    }
                    dir.put(outtable, i, j, 2);
                    last_pos = dir.inner_next(j);
                }
                _ => (),
            }
        }
        for j in dir.inner_until_end(intable, last_pos) {
            dir.put(outtable, i, j, 0);
        }
    }
}

fn prob2(lines: &Vec<&str>, rounds: u32) -> u32 {
    let lines = make_u8_matrix(lines);
    let mut outputn = vec![vec![0u8; lines.len()]; lines[0].len()];
    let mut outputw = vec![vec![0u8; lines.len()]; lines[0].len()];
    let mut outputs = vec![vec![0u8; lines.len()]; lines[0].len()];
    let mut outpute = vec![vec![0u8; lines.len()]; lines[0].len()];
    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            outpute[i][j] = lines[i][j];
        }
    }
    for round in 1..=rounds {
        move_rocks(&outpute, &mut outputn, &Direction::North);
        move_rocks(&outputn, &mut outputw, &Direction::West);
        move_rocks(&outputw, &mut outputs, &Direction::South);
        move_rocks(&outputs, &mut outpute, &Direction::East);
        if round % 700 == 1000000000 % 700 {
            println!("rnd {}, cnt: {}", round, count_load_north(&outpute));
        }
    }
    count_load_north(&outpute)
}

pub fn main() {
    let lines: Vec<&str> = include_str!("../day_14_input").lines().collect();
    println!("prob1: {}", prob1(&lines));
    // println!("prob2: {}", prob2(&lines, 1000000000));
    println!("prob2: {}", prob2(&lines, 1_000_000_000 % 7 + 7000));
}

#[cfg(test)]
mod tests {
    use crate::day_14::{count_column, make_u8_matrix, move_rocks, prob1, prob2, Direction};

    fn example() -> Vec<&'static str> {
        vec![
            "O....#....",
            "O.OO#....#",
            ".....##...",
            "OO.#O....O",
            ".O.....O#.",
            "O.#..O.#.#",
            "..O..#O..O",
            ".......O..",
            "#....###..",
            "#OO..#....",
        ]
    }

    #[test]
    fn test_count_column() {
        let e = example();
        let e8 = make_u8_matrix(&e);
        assert_eq!(count_column(&e8, 0), 34);
        assert_eq!(count_column(&e8, 9), 12);
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(&example()), 136);
    }

    #[test]
    fn test_prob2() {
        assert_eq!(prob2(&example(), 1_000_000_000 % 7 + 350), 64);
    }
    #[test]
    fn test_get_put() {
        let mut t = make_u8_matrix(&example());
        assert_eq!(Direction::North.get(t.as_slice(), 1, 0), 0);
        assert_eq!(Direction::North.get(t.as_slice(), 2, 1), 1);
        assert_eq!(Direction::East.get(t.as_slice(), 1, 0), 1);
        assert_eq!(Direction::East.get(t.as_slice(), 1, 4), 2);
        Direction::East.put(&mut t, 1, 0, 3);
        assert_eq!(t[1][0], 3);
        Direction::North.put(&mut t, 1, 0, 4);
        assert_eq!(t[0][1], 4);
    }

    #[test]
    fn test_move_rocks() {
        let input = make_u8_matrix(&vec![".O.#", ".OO.", "...."]);
        let mut output = vec![vec![0u8; 4]; 3];

        move_rocks(input.as_slice(), &mut output, &Direction::East);
        let expected = make_u8_matrix(&vec!["..O#", "..OO", "...."]);
        assert_eq!(output, expected);

        move_rocks(input.as_slice(), &mut output, &Direction::North);
        println!("{:?}", output);
        let expected = make_u8_matrix(&vec![".OO#", ".O..", "...."]);
        assert_eq!(output, expected);

        move_rocks(input.as_slice(), &mut output, &Direction::West);
        println!("{:?}", output);
        let expected = make_u8_matrix(&vec!["O..#", "OO..", "...."]);
        assert_eq!(output, expected);

        move_rocks(input.as_slice(), &mut output, &Direction::South);
        println!("{:?}", output);
        let expected = make_u8_matrix(&vec!["...#", ".O..", ".OO."]);
        assert_eq!(output, expected);
    }
}
