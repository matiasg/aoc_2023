use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug)]
struct CodeNum<'a> {
    code: &'a str,
    number: u32,
}

struct Boxes<'a> {
    boxes: HashMap<u8, Vec<CodeNum<'a>>>,
}

impl<'a> Boxes<'a> {
    fn new() -> Self {
        Self {
            boxes: HashMap::new(),
        }
    }

    fn insert(&mut self, code_op_num: &'a str) {
        if code_op_num.ends_with("-") {
            let code = code_op_num.get(..code_op_num.len() - 1).unwrap();
            let hash = u8::try_from(hash_code(code)).unwrap();
            let box_vec = self.boxes.entry(hash).or_insert(Vec::new());
            let i_cn = box_vec
                .iter()
                .enumerate()
                .filter(|(_, cn)| cn.code == code)
                .next();
            if let Some((i, _)) = i_cn {
                box_vec.remove(i);
            }
        } else {
            let coden: Vec<&str> = code_op_num.split('=').collect();
            let code = coden[0];
            let number: u32 = coden[1].parse().unwrap();
            let hash = u8::try_from(hash_code(code)).unwrap();
            let box_vec = self.boxes.entry(hash).or_insert(Vec::new());
            let i_cn = box_vec
                .iter()
                .enumerate()
                .filter(|(_, cn)| cn.code == code)
                .next();
            let codenum = CodeNum { code, number };
            if let Some((i, _)) = i_cn {
                box_vec[i] = codenum;
            } else {
                box_vec.push(codenum);
            }
        }
    }
}

fn hash_code(input: &str) -> u32 {
    let mut ret = 0;
    for c in input.chars() {
        ret += c as u32;
        ret *= 17;
        ret %= 256;
    }
    ret
}

fn prob1(input: &str) -> u32 {
    input.split(',').map(hash_code).sum()
}

fn prob2(input: &str) -> u32 {
    let mut boxes = Boxes::new();
    for b in input.split(',') {
        boxes.insert(b);
    }
    let mut ret: u32 = 0;
    for (&i, iboxes) in boxes.boxes.iter() {
        for (j, b) in iboxes.iter().enumerate() {
            let boxval = (i as u32 + 1) * (j as u32 + 1) * b.number as u32;
            ret += boxval;
        }
    }
    ret
}

pub fn main() {
    let input = include_str!("../day_15_input").trim();
    println!("prob1: {}", prob1(input));
    println!("prob2: {}", prob2(input));
}

#[cfg(test)]
mod tests {
    use crate::day_15::{hash_code, prob1, prob2, Boxes, CodeNum};

    fn example() -> &'static str {
        "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(example()), 1320);
    }

    #[test]
    fn test_prob2() {
        assert_eq!(prob2(example()), 145);
    }

    #[test]
    fn test_hash() {
        assert_eq!(hash_code("HASH"), 52);
    }

    #[test]
    fn test_insert() {
        let mut b = Boxes::new();
        b.insert(&mut "rn=1");
        assert_eq!(b.boxes.len(), 1);
        b.insert(&mut "cm-");
        assert_eq!(b.boxes.len(), 1);
        b.insert(&mut "qp=3");
        assert_eq!(b.boxes.len(), 2);
        b.insert(&mut "cm=2");
        assert_eq!(b.boxes.len(), 2);
        assert_eq!(b.boxes[&0].len(), 2);
        assert_eq!(b.boxes[&1].len(), 1);
        b.insert(&mut "qp-");
        assert_eq!(b.boxes[&1].len(), 0);
        b.insert(&mut "rn=3");
        assert_eq!(b.boxes[&0].len(), 2);
        assert_eq!(
            b.boxes[&0][0],
            CodeNum {
                code: "rn",
                number: 3
            }
        );
    }
}
