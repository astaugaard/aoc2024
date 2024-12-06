use crate::day;
use crate::utils;
use itertools::Itertools;
use once_cell::sync::Lazy;

type Input = Vec<Vec<char>>;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    Ok(input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec())
}

fn ismatch(i: i32, j: i32, di: i32, dj: i32, input: &[Vec<char>]) -> bool {
    "XMAS".chars().enumerate().all(|(ind, c)| {
        input
            .get((i + (ind as i32) * di) as usize)
            .map_or(None, |r| r.get((j + (ind as i32) * dj) as usize))
            == Some(&c)
    })
}

fn countmatches(i: i32, j: i32, input: &[Vec<char>]) -> usize {
    (-1..=1i32)
        .flat_map(|di| (-1..=1i32).filter(move |dj| ismatch(i, j, di, *dj, &input)))
        .count()
}

fn part_a(input: &Input) -> Option<String> {
    let mut matches = 0;

    for i in 0..input.len() {
        for j in 0..input.len() {
            matches += countmatches(i as i32, j as i32, input)
        }
    }

    Some(matches.to_string())
}

fn ismatchx(i: usize, j: usize, input: &[Vec<char>]) -> bool {
    if input[i][j] != 'A' {
        return false;
    }

    if i == 0 || j == 0 || i == input.len() - 1 || j == input.len() - 1 {
        return false;
    }

    if !((input[i - 1][j - 1] == 'M' && 'S' == input[i + 1][j + 1])
        || (input[i - 1][j - 1] == 'S' && 'M' == input[i + 1][j + 1]))
    {
        return false;
    }

    if !((input[i + 1][j - 1] == 'M' && 'S' == input[i - 1][j + 1])
        || (input[i + 1][j - 1] == 'S' && 'M' == input[i - 1][j + 1]))
    {
        return false;
    }

    return true;
}

fn part_b(input: &Input) -> Option<String> {
    let mut matches = 0;

    for i in 0..input.len() {
        for j in 0..input.len() {
            if ismatchx(i, j, input) {
                matches += 1;
            }
        }
    }

    Some(matches.to_string())
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

    #[test]
    fn goldens() {
        utils::golden("day4", &DAY, Some("18"), Some("9"), false)
    }

    #[test]
    fn finalanswer() {
        utils::finalanswer(4, &DAY, Some("2496"), Some("1967"), false);
    }
}
