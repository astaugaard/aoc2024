use bitvec::{bitvec, vec::BitVec};
use itertools::Itertools;
use once_cell::sync::Lazy;

use crate::day;

type Input = Vec<Vec<char>>;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    Ok(input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec())
}

fn part_a(input: &Input) -> Option<String> {
    let mut visited = vec![bitvec![0; input[0].len()]; input.len()];

    let mut sum = 0;

    for x in 0..input[0].len() {
        for y in 0..input.len() {
            if visited[y][x] {
                continue;
            }

            let (a, p) = score(input, x, y, &mut visited);

            sum += a * p;
        }
    }

    Some(sum.to_string())
}

fn score(input: &[Vec<char>], x: usize, y: usize, visited: &mut [BitVec]) -> (u32, u32) {
    if visited[y][x] {
        return (0, 0);
    }

    visited[y].set(x, true);

    let t = input[y][x];

    let y = y as i32;
    let x = x as i32;

    let mut area = 1;
    let mut perimiter = 4;

    let mut connections = 0;

    for (nx, ny) in [(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)].into_iter() {
        if ny < input.len() as i32
            && ny >= 0
            && nx < input[0].len() as i32
            && nx >= 0
            && input[ny as usize][nx as usize] == t
        {
            connections += 1;
            if visited[ny as usize][nx as usize] {
                continue;
            }

            let (a,p) = score(input, nx as usize, ny as usize, visited);

            area += a;
            perimiter += p;
        }
    }

    (area, perimiter - connections)
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
        utils::golden("day12-1", &DAY, Some("140"), None, false);
        utils::golden("day12-2", &DAY, Some("1930"), None, false);
    }

    //    #[test]
    //    fn finalanswer() {
    //        utils::finalanswer(1, &DAY, Some("2057374"), Some("23177084"), false);
    //    }
}
