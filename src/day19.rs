use crate::day;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

type Input = (Vec<String>, Vec<String>);

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    let mut lines = input.lines();
    let towels = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|a| a.to_string())
        .collect_vec();
    lines.next();

    let displays = lines.map(|a| a.to_string()).collect_vec();

    Ok((towels, displays))
}

fn part_a(input: &Input) -> Option<String> {
    let matcher = Regex::new(&format!("^({})*$", input.0.join("|"))).unwrap();

    let res = input.1.iter().filter(|line| matcher.is_match(line)).count();

    Some(res.to_string())
}

fn part_b(input: &Input) -> Option<String> {
    let words = input
        .0
        .iter()
        .map(|p| p.chars().collect_vec())
        .collect_vec();

    Some(
        input
            .1
            .iter()
            .map(|a| num_matches(a, &words))
            .sum::<u64>()
            .to_string(),
    )
}

fn num_matches(a: &str, towels: &[Vec<char>]) -> u64 {
    let a = a.chars().collect_vec();
    let mut matches = vec![0; a.len()];

    for i in (0..a.len()).rev() {
        let mut loc_count = 0;
        for t in towels {
            let len = t.len();
            if len > a.len() - i {
                continue;
            }

            if t.iter().zip(a[i..].iter()).all(|(a, b)| *a == *b) {
                if len == a.len() - i {
                    loc_count += 1;
                } else {
                    loc_count += matches[t.len() + i];
                }
            }
        }

        matches[i] = loc_count;
    }

    matches[0]
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
        utils::golden("day19", &DAY, Some("6"), Some("16"), false)
    }

    #[test]
    fn testcase() {
        let words = vec![
            vec!['r'],
            vec!['w', 'r'],
            vec!['b'],
            vec!['g'],
            vec!['b', 'w', 'u'],
            vec!['r', 'b'],
            vec!['g', 'b'],
            vec!['b', 'r'],
        ];

        assert_eq!(num_matches("brwrr", &words), 2);
        assert_eq!(num_matches("bggr", &words), 1);
        assert_eq!(num_matches("gbbr", &words), 4);
        assert_eq!(num_matches("rrbgbr", &words), 6);
        assert_eq!(num_matches("bwurrg", &words), 1);
        assert_eq!(num_matches("brgr", &words), 2);
    }

    #[test]
    fn finalanswer() {
        utils::finalanswer(19, &DAY, Some("338"), Some("841533074412361"), false);
    }
}
