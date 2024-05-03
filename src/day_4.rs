use regex::Regex;
use std::fs;

fn common(input: &Vec<&str>) -> Vec<(u32, u32)> {
    let mut total: Vec<(u32, u32)> = Vec::new();
    let winner_have = Regex::new("^Card *(\\d+): *([0-9 ]+) \\| ([0-9 ]+)$").unwrap();
    let spaces = Regex::new(" +").unwrap();
    for card in input {
        if card.len() == 0 {
            break;
        };
        let ch = winner_have.captures(card).unwrap();
        let have = spaces.split(ch.get(3).unwrap().as_str());
        let winner: Vec<&str> = spaces.split(ch.get(2).unwrap().as_str()).collect();
        let wh: u32 = have.filter(|h| winner.contains(h)).count() as u32;
        let card: u32 = ch.get(1).unwrap().as_str().parse().unwrap();
        total.push((card, wh));
    }
    total
}

fn problema1(input: &Vec<&str>) -> u32 {
    common(input)
        .iter()
        .map(|(_, w)| if *w > 0 { 2u32.pow(w - 1) } else { 0 })
        .sum()
}

fn problema2(input: &Vec<&str>) -> u32 {
    let mut total_hand: Vec<u32> = vec![1; input.len() + 1];
    total_hand[0] = 0;
    for (card, wins) in common(input) {
        let mult = total_hand[card as usize];
        let f = (card + 1) as usize;
        let t = (card + wins) as usize;
        for have in total_hand[f..=t].iter_mut() {
            *have += mult;
        }
    }
    total_hand.iter().sum()
}

pub fn main() {
    let input: String = fs::read_to_string("day_4_input").unwrap();
    let input: Vec<&str> = input.trim().split("\n").collect();
    println!("total for problem 1: {}", problema1(&input));
    println!("total for problem 2: {}", problema2(&input));
}
