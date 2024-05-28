#[derive(PartialEq, Eq, Debug, Clone)]
struct PosDir {
    x: isize,
    y: isize,
    dir: u8,
}

impl PosDir {
    fn inside(&self, input: &Vec<&str>) -> bool {
        (self.x >= 0 && self.x < input[0].len() as isize)
            && (self.y >= 0 && self.y < input.len() as isize)
    }

    fn next(&self, input: &Vec<&str>) -> Option<PosDir> {
        let result: PosDir = match self.dir {
            1 => PosDir {
                x: self.x + 1,
                y: self.y,
                dir: self.dir,
            },
            2 => PosDir {
                x: self.x,
                y: self.y - 1,
                dir: self.dir,
            },
            4 => PosDir {
                x: self.x - 1,
                y: self.y,
                dir: self.dir,
            },
            8 => PosDir {
                x: self.x,
                y: self.y + 1,
                dir: self.dir,
            },
            _ => return None,
        };
        if result.inside(input) {
            Option::Some(result)
        } else {
            None
        }
    }

    fn reflect_slash(&self) -> u8 {
        return match self.dir {
            1 => 2,
            2 => 1,
            4 => 8,
            8 => 4,
            _ => 0,
        };
    }

    fn reflect_backslash(&self) -> u8 {
        return match self.dir {
            1 => 8,
            2 => 4,
            4 => 2,
            8 => 1,
            _ => 0,
        };
    }

    fn is_horizontal(&self) -> bool {
        self.dir == 1 || self.dir == 4
    }

    fn read(&self, input: &Vec<&str>) -> char {
        input[self.y as usize].chars().nth(self.x as usize).unwrap()
    }

    fn read_t(&self, table: &Vec<Vec<u8>>) -> u8 {
        table[self.y as usize][self.x as usize]
    }

    fn visited(&self, dir: u8, table: &Vec<Vec<u8>>) -> bool {
        let v = self.read_t(table);
        (v & dir) != 0
    }

    fn advance(&self, input: &Vec<&str>, table: &mut Vec<Vec<u8>>) -> Vec<PosDir> {
        table[self.y as usize][self.x as usize] |= self.dir;
        let next = self.next(input);
        if next.is_none() {
            return vec![];
        }
        let next = next.unwrap();
        let next_dirs = self.next_dirs(&next, input);
        next_dirs
            .iter()
            .filter(|&&dir| !next.visited(dir, table))
            .map(|&dir| PosDir {
                x: next.x,
                y: next.y,
                dir,
            })
            .collect()
    }

    fn next_dirs(&self, next: &PosDir, input: &Vec<&str>) -> Vec<u8> {
        match next.read(input) {
            '.' => vec![next.dir],
            '/' => vec![self.reflect_slash()],
            '\\' => vec![self.reflect_backslash()],
            '-' => {
                if next.is_horizontal() {
                    vec![next.dir]
                } else {
                    vec![1, 4]
                }
            }
            '|' => {
                if next.is_horizontal() {
                    vec![2, 8]
                } else {
                    vec![next.dir]
                }
            }
            _ => vec![],
        }
    }
}

fn visited(table: &Vec<Vec<u8>>) -> u32 {
    table
        .iter()
        .map(|row| row.iter().filter(|&&x| x != 0).count())
        .sum::<usize>() as u32
}

fn prob1(input: &Vec<&str>) -> u32 {
    let start = PosDir { x: 0, y: 0, dir: 1 };
    visited_starting_from(input, start)
}

fn prob2(input: &Vec<&str>) -> u32 {
    let rows = input.len() as isize;
    let cols = input[0].len() as isize;
    let border = (0..cols)
        .map(|x| PosDir { x, y: 0, dir: 8 })
        .chain((0..cols).map(|x| PosDir {
            x,
            y: rows - 1,
            dir: 2,
        }))
        .chain((0..rows).map(|y| PosDir { x: 0, y, dir: 1 }))
        .chain((0..rows).map(|y| PosDir {
            x: cols - 1,
            y,
            dir: 4,
        }));
    border
        .map(|s| visited_starting_from(input, s))
        .max()
        .unwrap()
}

fn visited_starting_from(input: &Vec<&str>, start: PosDir) -> u32 {
    let mut table: Vec<Vec<u8>> = vec![vec![0; input[0].len()]; input.len()];
    let next_dirs = start.next_dirs(&start, input);
    let mut moving: Vec<PosDir> = next_dirs
        .iter()
        .map(|&d| PosDir {
            x: start.x,
            y: start.y,
            dir: d,
        })
        .collect();
    while !moving.is_empty() {
        let pd = moving.remove(0);
        let next_pds = pd.advance(input, &mut table);
        moving.extend(next_pds.clone());
    }
    visited(&table)
}

pub fn main() {
    let input: Vec<&str> = include_str!("../day_16_input").lines().collect();
    println!("prob1: {}", prob1(&input));
    println!("prob2: {}", prob2(&input));
}

#[cfg(test)]
mod tests {
    use crate::day_16::{prob1, PosDir};

    fn example() -> Vec<&'static str> {
        vec![
            r".|...\....",
            r"|.-.\.....",
            r".....|-...",
            r"........|.",
            r"..........",
            r".........\",
            r"..../.\\..",
            r".-.-/..|..",
            r".|....-|.\",
            r"..//.|....",
        ]
    }

    #[test]
    fn test_read() {
        assert_eq!(PosDir { x: 1, y: 1, dir: 8 }.read(&example()), '.');
        assert_eq!(PosDir { x: 1, y: 0, dir: 8 }.read(&example()), '|');
    }

    #[test]
    fn test_advance() {
        let input = example();
        let mut table: Vec<Vec<u8>> = vec![vec![0; input[0].len()]; input.len()];
        let first = PosDir { x: 0, y: 0, dir: 1 };
        let second = first.next(&input).unwrap();
        assert_eq!(second, PosDir { x: 1, y: 0, dir: 1 });
        assert_eq!(second.read(&input), '|');
        assert!(second.is_horizontal());
        let two = first.advance(&input, &mut table);
        assert_eq!(first.read_t(&table), 1);
        assert_eq!(two.len(), 2);
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(&example()), 46);
    }
}
