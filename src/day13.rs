use itertools::Itertools;
use num::rational::Ratio;
use once_cell::sync::Lazy;
use std::mem;

use crate::day;

type Input = Vec<((i64, i64), (i64, i64), (i64, i64))>;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    Ok(input
        .split("\n\n")
        .map(|section| {
            let mut lines = section.lines();
            let (ax, ay) = lines.next().unwrap()[12..].split_once(", Y+").unwrap();
            let ax = ax.parse::<i64>().unwrap();
            let ay = ay.parse::<i64>().unwrap();
            let (bx, by) = lines.next().unwrap()[12..].split_once(", Y+").unwrap();
            let bx = bx.parse::<i64>().unwrap();
            let by = by.parse::<i64>().unwrap();
            let (px, py) = &lines.next().unwrap()[9..].split_once(", Y=").unwrap();
            let px = px.parse::<i64>().unwrap();
            let py = py.parse::<i64>().unwrap();
            ((ax, ay), (bx, by), (px, py))
        })
        .collect_vec())
}

fn optimal_solution(
    ((ax, ay), (bx, by), (px, py)): ((i64, i64), (i64, i64), (i64, i64)),
) -> Option<i64> {
    let mut ax = Ratio::new_raw(ax, 1);
    let mut ay = Ratio::new_raw(ay, 1);
    let mut bx = Ratio::new_raw(bx, 1);
    let mut by = Ratio::new_raw(by, 1);
    let mut px = Ratio::new_raw(px, 1);
    let mut py = Ratio::new_raw(py, 1);

    if ax == 0.into() {
        mem::swap(&mut ax, &mut ay);
        mem::swap(&mut bx, &mut by);
        mem::swap(&mut px, &mut py);
    }

    if ax == 0.into() {
        if px != 0.into() {
            return None;
        }
        todo!("even");
    }

    if ay == 0.into() {
        return None;
    }

    // add multiple of x row to y row
    by -= ax.recip() * ay * bx;
    py -= ax.recip() * ay * px;

    if by == 0.into() && py != 0.into() {
        return None;
    }

    if by == 0.into() && py == 0.into() {
        todo!("meow")
    }

    // multiply bottom row by recip of by to make by spot one

    py = py * by.recip();

    if *py.denom() != 1 {
        return None;
    }

    px -= py * bx;

    px *= ax.recip();

    if *px.denom() != 1 {
        return None;
    }

    Some(3 * *px.numer() + *py.numer())
}

fn part_a(input: &Input) -> Option<String> {
    Some(
        input
            .iter()
            .map(|a| optimal_solution(*a).unwrap_or(0))
            .sum::<i64>()
            .to_string(),
    )
}

fn part_b(input: &Input) -> Option<String> {
    Some(
        input
            .iter()
            .map(|(a, b, (px, py))| {
                optimal_solution((*a, *b, (px + 10000000000000, py + 10000000000000))).unwrap_or(0)
            })
            .sum::<i64>()
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
        utils::golden("day13", &DAY, Some("480"), Some("875318608908"), false);
    }

    #[test]
    fn finalanswer() {
        utils::finalanswer(13, &DAY, Some("32067"), Some("92871736253789"), false);
    }
}
