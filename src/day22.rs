use crate::day;
use intmap::IntMap;
use itertools::Itertools;
use once_cell::sync::Lazy;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

type Input = Vec<u64>;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    Ok(input
        .lines()
        .map(|a| a.parse::<u64>().unwrap())
        .collect_vec())
}

fn sim_step(mut a: u64) -> u64 {
    a ^= a * 64;
    a %= 16777216;
    a ^= a / 32;
    a %= 16777216;
    a ^= a * 2048;
    a %= 16777216;
    a
}

fn sim_steps(mut a: u64, steps: u64) -> u64 {
    for _ in 0..steps {
        a = sim_step(a);
    }
    a
}

fn part_a(input: &Input) -> Option<String> {
    Some(
        input
            .iter()
            .map(|num| sim_steps(*num, 2000))
            .sum::<u64>()
            .to_string(),
    )
}

#[derive(Debug, Clone, Copy)]
struct Trader {
    secret: u64,
}

impl Iterator for Trader {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.secret;
        self.secret = sim_step(self.secret);
        Some(a)
    }
}

fn convert_to_key(a: i8, b: i8, c: i8, d: i8) -> u64 {
    let a = a as u8 as u64;
    let b = (b as u8 as u64) << 8;
    let c = (c as u8 as u64) << 16;
    let d = (d as u8 as u64) << 24;

    a + b + c + d
}

fn add_num(num: u64, map: &mut IntMap<u64>) {
    let mut visited: IntMap<()> = IntMap::new();

    for ((a, _), (b, _), (c, _), (d, val)) in (Trader { secret: num })
        .take(2000)
        .map(|a| a % 10)
        .tuple_windows()
        .map(|(a, b)| ((b as i8 - a as i8), b % 10))
        .tuple_windows()
    {
        if visited.contains_key(convert_to_key(a, b, c, d)) {
            continue;
        }

        match map.get_mut(convert_to_key(a, b, c, d)) {
            Some(loc) => *loc += val,
            None => {
                map.insert(convert_to_key(a, b, c, d), val);
            }
        };

        // *map.entry().or_insert(0) += val;
        visited.insert(convert_to_key(a, b, c, d), ());
    }
}

fn part_b(input: &Input) -> Option<String> {
    let map = input
        .par_iter()
        .map(|num| {
            let mut map: IntMap<u64> = IntMap::new();
            add_num(*num, &mut map);
            map
        })
        .reduce(
            || IntMap::new(),
            |mut m1, m2| {
                for (k, v) in m2.into_iter() {
                    match m1.get_mut(k) {
                        Some(loc) => *loc += v,
                        None => {
                            m1.insert(k, v);
                        }
                    }
                }
                m1
            },
        );

    Some(map.values().max().unwrap().to_string())
}

fn exe(_input: &Input) {
    todo!()
}

pub static DAY: Lazy<day::Day<Input>> = Lazy::new(|| day::Day {
    // do not touch
    parser: Box::new(parser),
    part_a: Box::new(part_a),
    part_b: Box::new(part_b),
    exe: Box::new(exe),
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn goldens() {
        utils::golden("day22", &DAY, Some("37327623"), None, false);
        utils::golden("day22-2", &DAY, None, Some("23"), false);
    }

    #[test]
    fn sim_step_123() {
        assert_eq!(sim_step(123), 15887950);
    }

    #[test]
    fn finalanswer() {
        utils::finalanswer(22, &DAY, Some("20506453102"), Some("2423"), false);
    }
}
