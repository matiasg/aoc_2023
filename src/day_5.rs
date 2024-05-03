use regex::Regex;
use std::fs;

fn get_maps(input: Vec<&str>) -> (Vec<i64>, Vec<Vec<(i64, i64, i64)>>) {
    let srg = Regex::new("^seeds: ([ \\d]+)$").unwrap();
    let spaces = Regex::new(" +").unwrap();
    let map_title = Regex::new("^[a-z-]*-to-[a-z-]* map:$").unwrap();
    let mapline = Regex::new("^(\\d+) (\\d+) (\\d+)$").unwrap();

    let seeds = srg.captures(input[0]).unwrap().get(1).unwrap().as_str();
    let seeds: Vec<i64> = spaces.split(seeds).map(|s| s.parse().unwrap()).collect();
    let mut maps: Vec<Vec<(i64, i64, i64)>> = vec![];
    let mut last_map: Vec<(i64, i64, i64)> = vec![];
    for line in input[2..].iter() {
        if map_title.is_match(line) {
            continue;
        }
        if line.len() == 0 {
            maps.push(last_map);
            last_map = vec![];
            continue;
        }
        let tar_from_len = mapline.captures(line).unwrap();
        let tar: i64 = tar_from_len[1].to_string().parse().unwrap();
        let fro: i64 = tar_from_len[2].to_string().parse().unwrap();
        let len: i64 = tar_from_len[3].to_string().parse().unwrap();
        last_map.push((fro, len, tar - fro));
    }
    maps.push(last_map);
    (seeds, maps)
}

fn apply(maps: &Vec<Vec<(i64, i64, i64)>>, seed: i64) -> i64 {
    let mut ret = seed;
    for map in maps {
        for (fro, len, dif) in map {
            if (*fro <= ret) && (ret < fro + len) {
                ret += dif;
                break;
            }
        }
    }
    ret
}

fn apply_one_step_prob_2(maps: &Vec<(i64, i64, i64)>, seeds: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut ret: Vec<(i64, i64)> = vec![];
    let mut seeds = seeds.clone();
    while let Some((from, len)) = seeds.pop() {
        let mut applied_some_map = false;
        for (mapfrom, maplen, mapdif) in maps {
            if *mapfrom <= from {
                if (mapfrom + maplen) <= from {
                    continue;
                } else if mapfrom + maplen < from + len {
                    let cutlen = mapfrom + maplen - from;
                    ret.push((from + mapdif, cutlen));
                    seeds.push((from + cutlen, len - cutlen));
                    applied_some_map = true;
                    break;
                } else {
                    ret.push((from + mapdif, len));
                    applied_some_map = true;
                    break;
                }
            } else if *mapfrom < from + len {
                if mapfrom + maplen < from + len {
                    ret.push((mapfrom + mapdif, *maplen));
                    seeds.push((from, mapfrom - from));
                    seeds.push((mapfrom + maplen, from + len - (mapfrom + maplen)));
                    applied_some_map = true;
                    break;
                } else {
                    ret.push((mapfrom + mapdif, from + len - mapfrom));
                    seeds.push((from, mapfrom - from));
                    applied_some_map = true;
                    break;
                }
            } else {
                continue;
            }
        }
        if !applied_some_map {
            ret.push((from, len));
        }
    }
    ret
}

pub fn main() {
    let input: String = fs::read_to_string("day_5_input").unwrap();
    let input: Vec<&str> = input.trim().split("\n").collect();
    let (seeds, maps) = get_maps(input);
    let prob1 = seeds.iter().map(|s| apply(&maps, *s)).min().unwrap();
    println!("result to prob 1: {}", prob1);
    let mut as_pairs: Vec<(i64, i64)> = (0..seeds.len())
        .step_by(2)
        .map(|i| (seeds[i], seeds[i + 1]))
        .collect();
    for map in &maps {
        as_pairs = apply_one_step_prob_2(&map, &as_pairs);
    }
    let prob2 = as_pairs.iter().map(|(f, _)| f).min().unwrap();
    println!("result to prob 2: {}", prob2);
}
