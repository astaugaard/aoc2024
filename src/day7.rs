use crate::day;
use itertools::Itertools;
use num::CheckedMul;
use once_cell::sync::Lazy;
use std::cmp;

type Input = Vec<(u64, Vec<u64>)>;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    Ok(input
        .lines()
        .map(|line| {
            let (before, after) = line.split_once(": ").unwrap();
            let equation = after
                .split(' ')
                .map(|a| a.parse::<u64>().unwrap())
                .collect_vec();
            (before.parse::<u64>().unwrap(), equation)
        })
        .collect_vec())
}

fn part_a(input: &Input) -> Option<String> {
    let res = input
        .iter()
        .filter(|(r, vs)| solvable(*r, vs))
        .map(|(r, _)| r)
        .sum::<u64>();

    Some(res.to_string())
}

fn solvable(r: u64, vs: &[u64]) -> bool {
    let upper = vs[0..] // upper bound
        .iter()
        .scan(0, |state, x| {
            *state = cmp::max(*state + x, *state * x);

            if *state > r {
                return None;
            }

            Some(*state)
        })
        .collect_vec();

    let lower = vs[0..] // lower bound
        .iter()
        .scan(1, |state, x| {
            *state = cmp::min(*state + x, *state * x);

            Some(*state)
        })
        .collect_vec();

    let target = vs[0];

    check_solvable(target, r, &upper, &lower, &vs[1..], vs.len() - 2)
}

fn check_solvable(
    target: u64,
    current: u64,
    upper: &[u64],
    lower: &[u64],
    rev: &[u64],
    i: usize,
) -> bool {
    if current > *upper.get(i + 1).unwrap_or(&std::u64::MAX) {
        return false;
    }

    if current < lower[i + 1] {
        return false;
    }

    if current % rev[i] == 0 {
        let next = current / rev[i];
        if i == 0 {
            if next == target {
                return true;
            }
        } else if check_solvable(target, next, upper, lower, rev, i - 1) {
            return true;
        }
    }

    let next = current - rev[i];

    if i == 0 {
        return next == target;
    }

    check_solvable(target, next, upper, lower, rev, i - 1)
}

fn part_b(input: &Input) -> Option<String> {
    let res = input
        .iter()
        .filter(|(r, vs)| solvableb(*r, vs))
        .map(|(r, _)| r)
        .sum::<u64>();

    Some(res.to_string())
}

fn digits(b: u64) -> u32 {
    b.ilog(10) + 1
}

fn concat(a: u64, b: u64) -> Option<u64> {
    a.checked_mul(10u64.pow(digits(b))).map(|h| h + b)
}

fn solvableb(r: u64, vs: &[u64]) -> bool {
    let upper = vs[0..] // upper bound
        .iter()
        .scan(0, |state, x| {
            *state = cmp::max(
                cmp::max(*state + x, state.checked_mul(*x).unwrap_or(std::u64::MAX)),
                concat(*state, *x).unwrap_or(std::u64::MAX),
            );

            if *state > r {
                return None;
            }

            Some(*state)
        })
        .collect_vec();

    let lower = vs[0..] // lower bound
        .iter()
        .scan(1, |state, x| {
            *state = cmp::min(
                cmp::min(*state + x, state.checked_mul(*x).unwrap_or(std::u64::MAX)),
                concat(*state, *x).unwrap_or(std::u64::MAX),
            );

            Some(*state)
        })
        .collect_vec();

    let target = vs[0];

    if vs.len() == 1 {
        return vs[0] == r;
    }

    check_solvableb(target, r, &upper, &lower, &vs[1..], vs.len() - 2)
}

fn check_solvableb(
    target: u64,
    current: u64,
    upper: &[u64],
    lower: &[u64],
    rev: &[u64],
    i: usize,
) -> bool {
    if current > *upper.get(i + 1).unwrap_or(&std::u64::MAX) {
        return false;
    }

    if current < lower[i + 1] {
        return false;
    }

    if current % rev[i] == 0 {
        let next = current / rev[i];
        if i == 0 {
            if next == target {
                return true;
            }
        } else if check_solvableb(target, next, upper, lower, rev, i - 1) {
            return true;
        }
    }


    if current % 10u64.pow(digits(rev[i])) == rev[i] {
        let next = current / 10u64.pow(digits(rev[i]));

        if i == 0 {
            if next == target {
                return true;
            }
        } else if check_solvableb(target, next, upper, lower, rev, i - 1) {
            return true;
        }
    }

    let next = current - rev[i];

    if i == 0 {
        return next == target;
    }

    check_solvableb(target, next, upper, lower, rev, i - 1)
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
    use itertools::Itertools;
    use proptest::collection;
    use proptest::prelude::*;

    #[test]
    fn goldens() {
        utils::golden("day7", &DAY, Some("3749"), Some("11387"), false)
    }

    #[test]
    fn finalanswer() {
        utils::finalanswer(7, &DAY, Some("28730327770375"), Some("424977609625985"), false);
    }

    #[derive(Debug, Clone)]
    enum Operators {
        Concat,
        Mult,
        Add,
    }

    fn operators_strategy() -> impl Strategy<Value = Operators> {
        prop_oneof![
            Just(Operators::Add),
            Just(Operators::Mult),
            Just(Operators::Concat),
        ]
    }

    prop_compose! {
        fn passing_input_maybe()(len in 2..10usize)(
            operators in (collection::vec(operators_strategy(), len - 1)),
            nums in collection::vec(1..1000u64, len),
        ) -> Option<(u64, Vec<u64>, Vec<Operators>)> {
            let r = nums[1..].iter().zip(operators.iter()).fold(Some(nums[0]), |a, (b, o)| {
                match o {
                    Operators::Add => Some(a? + b),
                    Operators::Mult => a?.checked_mul(*b),
                    Operators::Concat => concat(a?,*b),
                }
            })?;

            Some((r, nums, operators))
        }
    }

    proptest! {
        #[test]
        fn concat_works(a in 1..1000u64, b in 1..1000u64) {
            prop_assert!(concat(a,b) == Some((a.to_string() + &b.to_string()).parse::<u64>().unwrap()))
        }
    }

    proptest! {
        #[test]
        fn all_passing(i in passing_input_maybe()) {
            match i {
                Some(i) => prop_assert!(solvableb(i.0, &i.1)),
                None => prop_assume!(false),
            }

        }

    }

    #[test]
    fn passing111() {
        assert!(solvableb(111, &[1, 1, 1]))
    }

    #[test]
    fn concat11() {
        assert_eq!(concat(1, 1), Some(11));
    }

    #[test]
    fn digits() {
        assert_eq!(super::digits(1), 1);
    }
}
