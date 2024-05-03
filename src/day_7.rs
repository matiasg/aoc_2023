use regex::Regex;
use std::{cmp::Ordering, collections::HashMap, fs};

fn parse_hands(input: Vec<&str>) -> Vec<(String, i64)> {
    let space = Regex::new("^([0-9TJQKA]{5}) ([0-9]+)$").unwrap();
    let mut ret: Vec<(String, i64)> = Vec::new();
    for hand in input {
        let m = space.captures(hand).unwrap();
        ret.push((
            m.get(1).unwrap().as_str().to_string(),
            m.get(2).unwrap().as_str().parse().unwrap(),
        ));
    }
    ret
}

fn count_cards(hand: String) -> HashMap<char, u8> {
    let mut quantities: HashMap<char, u8> = HashMap::new();
    for c in hand.chars() {
        *quantities.entry(c).or_insert(0) += 1;
    }
    quantities
}

fn detect_hand(hand: String) -> u8 {
    // highcard: 0, pair: 1, double pair: 2, trio: 3, full: 4, four: 5, five: 6
    let quantities = count_cards(hand);
    let m = quantities.values().max().unwrap();
    if *m >= 4 {
        return m + 1;
    }
    if (*m == 3) & (quantities.len() == 2) {
        return 4;
    };
    if *m == 3 {
        return 3;
    }
    if (*m == 2) & (quantities.len() == 3) {
        return 2;
    }
    if *m == 2 {
        return 1;
    };
    return 0;
}

fn conversor(h0: String, conv_map: Vec<(&str, &str)>) -> String {
    let mut hand = h0.clone();
    for (f, t) in conv_map {
        hand = hand.as_str().replace(f, t).to_string();
    }
    hand
}

fn prob1_conversor(h0: String) -> String {
    conversor(
        h0,
        vec![("T", "a"), ("J", "b"), ("Q", "c"), ("K", "d"), ("A", "e")],
    )
}

fn prob2_conversor(h0: String) -> String {
    conversor(
        h0,
        vec![("T", "a"), ("J", "&"), ("Q", "c"), ("K", "d"), ("A", "e")],
    )
}

fn compare_hands(
    h1: String,
    h2: String,
    detector: impl Fn(String) -> u8,
    conversor: impl Fn(String) -> String,
) -> Ordering {
    (detector(h1.clone()), conversor(h1)).cmp(&(detector(h2.clone()), conversor(h2)))
}

fn prob1(input: Vec<&str>) -> i64 {
    let mut hands_bids = parse_hands(input);
    hands_bids.sort_by(|hb1, hb2| {
        compare_hands(hb1.0.clone(), hb2.0.clone(), detect_hand, prob1_conversor)
    });
    hands_bids
        .iter()
        .enumerate()
        .map(|(pos, (_, b))| (pos as i64 + 1) * b)
        .sum()
}

fn convert_js(hand: String) -> String {
    let quantities = count_cards(hand.clone());
    if (quantities.len() == 1) & (hand.starts_with('J')) {
        return hand.clone();
    }
    let max_card_no_j = quantities
        .iter()
        .filter(|(c, _)| **c != 'J')
        .max_by(|(c1, q1), (c2, q2)| (q1, c1).cmp(&(q2, c2)))
        .unwrap()
        .0;
    hand.replace('J', &max_card_no_j.to_string())
}

fn prob2(input: Vec<&str>) -> i64 {
    let mut hands_bids = parse_hands(input);
    hands_bids.sort_by(|hb1, hb2| {
        compare_hands(
            hb1.0.clone(),
            hb2.0.clone(),
            |h| detect_hand(convert_js(h.clone())),
            prob2_conversor,
        )
    });
    hands_bids
        .iter()
        .enumerate()
        .map(|(pos, (_, b))| (pos as i64 + 1) * b)
        .sum()
}

pub fn main() {
    let input0: String = fs::read_to_string("day_7_input").unwrap();
    let input: Vec<&str> = input0.trim().split("\n").collect();
    println!("problem 1: {}", prob1(input));
    let input: Vec<&str> = input0.trim().split("\n").collect();
    println!("problem 2: {}", prob2(input));
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::day_7::{
        compare_hands, convert_js, detect_hand, parse_hands, prob1, prob1_conversor, prob2,
    };

    fn example() -> Vec<&'static str> {
        vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ]
    }

    #[test]
    fn test_parsehands() {
        let hands = parse_hands(example());
        assert_eq!(hands.len(), 5);
        let bids: Vec<i64> = hands.iter().map(|hb| hb.1).collect();
        assert_eq!(bids, vec![765, 684, 28, 220, 483]);
    }

    #[test]
    fn test_parsehand() {
        let detects: Vec<u8> = parse_hands(example())
            .iter()
            .map(|hb| detect_hand(hb.0.clone()))
            .collect();
        assert_eq!(detects, vec![1, 3, 2, 2, 3]);
    }

    #[test]
    fn test_compare_hands() {
        let e = example();
        assert_eq!(
            compare_hands(
                e[0].to_string(),
                e[1].to_string(),
                detect_hand,
                prob1_conversor
            ),
            Ordering::Less
        );
        assert_eq!(
            compare_hands(
                e[2].to_string(),
                e[3].to_string(),
                detect_hand,
                prob1_conversor
            ),
            Ordering::Greater
        );
    }

    #[test]
    fn test_convert_js() {
        assert_eq!(convert_js("00112".to_string()), "00112");
        assert_eq!(convert_js("0001J".to_string()), "00010");
        assert_eq!(convert_js("0011J".to_string()), "00111");
        assert_eq!(convert_js("1100J".to_string()), "11001");
        assert_eq!(convert_js("00J1J".to_string()), "00010");
        assert_eq!(convert_js("J0J1J".to_string()), "10111");
        assert_eq!(convert_js("JJJ1J".to_string()), "11111");
        assert_eq!(convert_js("JJJJJ".to_string()), "JJJJJ");
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(example()), 6440);
    }

    #[test]
    fn test_prob2() {
        assert_eq!(prob2(example()), 5905);
    }
}
