use crate::day;
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::collections::HashMap;

type Input = Vec<u64>;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    Ok(input
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .map(|num| num.parse::<u64>().unwrap())
        .collect_vec())
}

fn part_a(input: &Input) -> Option<String> {
    let mut cache = HashMap::new();

    Some(
        input
            .iter()
            .map(|num| count_final_cached(*num, 25, &mut cache))
            .sum::<u64>()
            .to_string(),
    )
}

fn digits(b: u64) -> u32 {
    b.ilog(10) + 1
}

fn split(a: u64) -> (u64, u64) {
    let d = 10u64.pow(digits(a) / 2);
    (a % d, a / d)
}

fn count_final_cached(num: u64, times: u32, cache: &mut HashMap<(u64, u32), u64>) -> u64 {
    if times == 0 {
        return 1;
    }

    if let Some(a) = cache.get(&(num, times)) {
        return *a;
    }

    let res = if num == 0 {
        count_final_cached(1, times - 1, cache)
    } else if digits(num) % 2 == 0 {
        let (a, b) = split(num);
        count_final_cached(a, times - 1, cache) + count_final_cached(b, times - 1, cache)
    } else {
        count_final_cached(num * 2024, times - 1, cache)
    };

    cache.insert((num, times), res);

    res
}

fn part_b(input: &Input) -> Option<String> {
    let mut cache = HashMap::new();

    Some(
        input
            .iter()
            .map(|num| count_final_cached(*num, 75, &mut cache))
            .sum::<u64>()
            .to_string(),
    )
}

pub static DAY: Lazy<day::Day<Input>> = Lazy::new(|| day::Day {
    // do not touch
    parser: Box::new(parser),
    part_a: Box::new(part_a),
    part_b: Box::new(part_b),
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn goldens() {
        utils::golden("day11", &DAY, Some("55312"), Some("65601038650482"), false)
    }

    #[test]
    fn split253000() {
        assert_eq!(split(253000), (0, 253));
    }

    #[test]
    fn finalanswer() {
        utils::finalanswer(11, &DAY, Some("197357"), Some("234568186890978"), false);
    }
}
