use inttable::IntTable;
use itertools::Itertools;
use once_cell::sync::Lazy;

use crate::day;

type Input = Vec<(u32, u32)>;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    Ok(input
        .lines()
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .collect_vec())
}

fn part_a(input: &Input) -> Option<String> {
    let mut a = input.iter().map(|a| a.0).collect_vec();
    let mut b = input.iter().map(|a| a.1).collect_vec();

    radsort::sort(&mut a);
    radsort::sort(&mut b);

    let res = a
        .into_iter()
        .zip(b)
        .map(|(a, b)| b.abs_diff(a))
        .sum::<u32>();

    Some(format!("{}", res))
}

fn part_b(input: &Input) -> Option<String> {
    let mut times: IntTable<u32> = IntTable::with_capacity(1 << (input.len().ilog2() + 1));

    for i in input.iter().map(|a| a.1) {
        *times.entry(i as u64).or_insert(0) += 1;
    }

    let res = input
        .iter()
        .map(|a| times.get(a.0 as u64).unwrap_or(&0) * a.0)
        .sum::<u32>();

    Some(format!("{}", res))
}

fn exe(_input: &Input) {
    todo!()
}

pub static DAY: Lazy<day::Day<Input>> = Lazy::new(|| day::Day {
    // list of tests (functions that take a bool (weather to be verbose))
    // and return an Option<String>
    // None if there was no error
    // Some(err) if the test failed
    //
    // a helper function golden is supplied so that you can test your parts against real inputs
    //
    // utils::golden(golden name, day, expected output part 1, expected output part 2)
    //
    // golden name is the name of the input file in the goldens directory
    // the day is this structure so it is always &DAY
    // the expected outputs are of the forms Some(expected) or None (when you are not testing that part with this golden)
    //
    // example for aoc day5 in 2021:
    //
    // use crate::utils;

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
        utils::golden("day1", &DAY, Some("11"), Some("31"), false)
    }

    #[test]
    fn finalanswer() {
        utils::finalanswer(1, &DAY, Some("2057374"), Some("23177084"), false);
    }
}
