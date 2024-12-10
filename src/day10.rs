use crate::day;
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::collections::HashSet;

type Input = Vec<Vec<u8>>;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    Ok(input
        .lines()
        .map(|line| {
            line.chars()
                .map(|d| d.to_digit(10).unwrap() as u8)
                .collect_vec()
        })
        .collect_vec())
}

fn part_a(input: &Input) -> Option<String> {
    Some(
        input
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_x, c)| **c == 0)
                    .map(|(x, _c)| find_score(input, x, y))
                    .sum::<u32>()
            })
            .sum::<u32>()
            .to_string(),
    )
}

fn find_score(input: &[Vec<u8>], x: usize, y: usize) -> u32 {
    find_score_go(input, Box::new([(x as i32, y as i32)].into_iter()), 1)
}

fn find_score_go<'a>(
    input: &'a [Vec<u8>],
    locs: Box<dyn Iterator<Item = (i32, i32)> + 'a>, // kindof hacky lol
    next: u8,
) -> u32 {
    let mut visited = HashSet::new();

    if next == 10 {
        return locs.count() as u32;
    }

    let nlocs = locs.flat_map(move |(x, y)| {
        let res_loc = [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .filter_map(|(dx, dy)| {
                let nx = dx + x;
                let ny = dy + y;

                if nx < 0
                    || (nx as usize) >= input[0].len()
                    || ny < 0
                    || (ny as usize) >= input.len()
                {
                    return None;
                }

                if input[ny as usize][nx as usize] != next {
                    return None;
                }

                if visited.contains(&(nx, ny)) {
                    return None;
                }

                visited.insert((nx, ny));

                Some((nx, ny))
            })
            .collect_vec();

        res_loc.into_iter()
    });

    find_score_go(input, Box::new(nlocs), next + 1)
}

fn part_b(input: &Input) -> Option<String> {
    Some(
        input
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_x, c)| **c == 0)
                    .map(|(x, _c)| find_score_b(input, x, y))
                    .sum::<u32>()
            })
            .sum::<u32>()
            .to_string(),
    )
}

fn find_score_b(input: &[Vec<u8>], x: usize, y: usize) -> u32 {
    find_score_go_b(input, Box::new([(x as i32, y as i32)].into_iter()), 1)
}

fn find_score_go_b<'a>(
    input: &'a [Vec<u8>],
    locs: Box<dyn Iterator<Item = (i32, i32)> + 'a>, // kindof hacky lol
    next: u8,
) -> u32 {
    if next == 10 {
        return locs.count() as u32;
    }

    let nlocs = locs.flat_map(move |(x, y)| {
        let res_loc = [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .filter_map(|(dx, dy)| {
                let nx = dx + x;
                let ny = dy + y;

                if nx < 0
                    || (nx as usize) >= input[0].len()
                    || ny < 0
                    || (ny as usize) >= input.len()
                {
                    return None;
                }

                if input[ny as usize][nx as usize] != next {
                    return None;
                }

                Some((nx, ny))
            })
            .collect_vec();

        res_loc.into_iter()
    });

    find_score_go_b(input, Box::new(nlocs), next + 1)
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
        utils::golden("day10", &DAY, Some("36"), Some("81"), false)
    }

    #[test]
    fn finalanswer() {
        utils::finalanswer(10, &DAY, Some("776"), Some("1657"), false);
    }
}
