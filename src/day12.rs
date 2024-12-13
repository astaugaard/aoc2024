use num::rational::Ratio;
use once_cell::sync::Lazy;

use crate::day;

type Input = Vec<((u32, u32), (u32, u32), (u32, u32))>;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    Ok(input
        .split("\n\n")
        .map(|section| {
            let lines = sections.lines();
            let (ax, ay) = lines.next().unwrap()[12..].split_once(", Y+");
            let ax = ax.parse::<u32>().unwrap();
            let ay = ay.parse::<u32>().unwrap();
            let (bx, by) = lines.next().unwrap()[12..].split_once(", Y+");
            let bx = bx.parse::<u32>().unwrap();
            let by = by.parse::<u32>().unwrap();
            let (px, py) = lines.next().unwrap()[8..].split_once(", Y=");
            let px = px.parse::<u32>().unwrap();
            let py = py.parse::<u32>().unwrap();
            ((ax, ay), (bx, by), (px, py))
        })
        .collect_vec())
}

fn optimal_solution(
    ((ax, ay), (bx, by), (px, py)): ((u32, u32), (u32, u32), (u32, u32)),
) -> Option<u32> {
    let mut ax = Ratio::new_raw(ax, 1);
    let mut ay = Ratio::new_raw(ay, 1);
    let mut bx = Ratio::new_raw(bx, 1);
    let mut by = Ratio::new_raw(by, 1);
    let mut px = Ratio::new_raw(px, 1);
    let mut py = Ratio::new_raw(py, 1);

    if ax == 0 {
        mem::swap(&mut ax, &mut ay);
        mem::swap(&mut bx, &mut by);
        mem::swap(&mut px, &mut py);
    }

    if ax == 0 {
        if px != 0 {
            return None;
        }
        todo!("even");
    }

    if ay == 0 {
        return None;
    }

    // add multiple of x row to y row
    by -= ax.recip() * ay * bx;
    py -= ax.recip() * ay * px;
    ay = 0;

    if by == 0 && py != 0 {
        return None;
    }

    if by == 0 && py == 0 {
        todo!()
    }

    // multiply bottom row by recip of by to make by spot one

    py = py * by.recip();
    by = 1;

    if py.denom() != 1 {
        return None;
    }

    px -= py * bx;
    bx = 0;

    px *= ax.recip();
    ax = 1;

    if px.denom() != 1 {
        return None;
    }

    Some(3 * *px.numer() + *py.numer())
}

fn part_a(input: &Input) -> Option<String> {
    Some(
        input
            .iter()
            .map(|a| optimal_solution(*a).unwrap_or(0))
            .sum::<u32>()
            .to_string(),
    )
}

fn part_b(_input: &Input) -> Option<String> {
    None
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
        utils::golden("day12", &DAY, Some("480"), None, false);
    }

    // #[test]
    // fn finalanswer() {
    //     utils::finalanswer(1, &DAY, Some("2057374"), Some("23177084"), false);
    // }
}
