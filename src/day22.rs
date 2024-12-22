use crate::day;
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};

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

fn add_num(num: u64, map: &mut HashMap<(i8, i8, i8, i8), u64>) {
    let mut visited: HashSet<(i8, i8, i8, i8)> = HashSet::new();

    for ((a, _), (b, _), (c, _), (d, val)) in (Trader { secret: num })
        .take(2000)
        .map(|a| a % 10)
        .tuple_windows()
        .map(|(a, b)| ((b as i8 - a as i8), b % 10))
        .tuple_windows()
    {
        if visited.contains(&(a, b, c, d)) {
            continue;
        }

        *map.entry((a, b, c, d)).or_insert(0) += val;
        visited.insert((a, b, c, d));
    }
}

fn part_b(input: &Input) -> Option<String> {
    let mut map: HashMap<(i8, i8, i8, i8), u64> = HashMap::new();

    for num in input {
        add_num(*num, &mut map);
    }

    Some(map.into_values().max().unwrap().to_string())
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

    // #[test]
    // fn finalanswer() {
    //     utils::finalanswer(1, &DAY, Some("2057374"), Some("23177084"), false);
    // }
}
