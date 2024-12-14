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

            let (a, p) = score(input, nx as usize, ny as usize, visited);

            area += a;
            perimiter += p;
        }
    }

    (area, perimiter - connections)
}

fn part_b(input: &Input) -> Option<String> {
    let mut visited = vec![bitvec![0; input[0].len()]; input.len()];

    let mut sum = 0;

    for x in 0..input[0].len() {
        for y in 0..input.len() {
            if visited[y][x] {
                continue;
            }

            let (a, p) = score2(input, x, y, &mut visited);

            sum += a * p;
        }
    }

    Some(sum.to_string())
}

fn score2(input: &[Vec<char>], x: usize, y: usize, visited: &mut [BitVec]) -> (u32, u32) {
    if visited[y][x] {
        return (0, 0);
    }

    visited[y].set(x, true);

    let t = input[y][x];

    let y = y as i32;
    let x = x as i32;

    let mut area = 1;

    let mut perimiter = [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .into_iter()
        .cycle()
        .take(5)
        .tuple_windows()
        .map(|((dx1, dy1), (dx2, dy2))| {
            let x1 = x + dx1;
            let y1 = y + dy1;
            let x2 = x + dx2;
            let y2 = y + dy2;
            // I hate this code that follows it sucks but I can't be bothered
            if (y1 < input.len() as i32
                && y1 >= 0
                && x1 < input[0].len() as i32
                && x1 >= 0
                && input[y1 as usize][x1 as usize] == t
                && y2 < input.len() as i32
                && y2 >= 0
                && x2 < input[0].len() as i32
                && x2 >= 0
                && input[y2 as usize][x2 as usize] == t
                && (input[(dy2 + dy1 + y) as usize][(dx2 + dx1 + x) as usize] != t))
                || ((y1 >= input.len() as i32
                    || y1 < 0
                    || x1 >= input[0].len() as i32
                    || x1 < 0
                    || input[y1 as usize][x1 as usize] != t)
                    && (y2 >= input.len() as i32
                        || y2 < 0
                        || x2 >= input[0].len() as i32
                        || x2 < 0
                        || input[y2 as usize][x2 as usize] != t))
            {
                1
            } else {
                0
            }
        })
        .sum::<u32>();

    for (nx, ny) in [(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)].into_iter() {
        if ny < input.len() as i32
            && ny >= 0
            && nx < input[0].len() as i32
            && nx >= 0
            && input[ny as usize][nx as usize] == t
        {
            if visited[ny as usize][nx as usize] {
                continue;
            }

            let (a, p) = score2(input, nx as usize, ny as usize, visited);

            area += a;
            perimiter += p;
        }
    }

    (area, perimiter)
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
        utils::golden("day12-1", &DAY, Some("140"), Some("80"), false);
        utils::golden("day12-2", &DAY, Some("1930"), Some("1206"), false);
    }

       #[test]
       fn finalanswer() {
           utils::finalanswer(12, &DAY, Some("1465968"), Some("897702"), false);
       }
}
