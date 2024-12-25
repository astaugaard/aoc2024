use crate::day;
use itertools::Itertools;
use once_cell::sync::Lazy;

type Input = (Vec<Vec<u16>>, Vec<Vec<u16>>);

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for t in input.split("\n\n") {
        let t = t.lines().map(|l| l.chars().collect_vec()).collect_vec();

        if t[0].iter().all(|k| *k == '#') {
            let mut lock = Vec::new();
            for c in 0..t[0].len() {
                let mut num = 0;
                for r in 0..t.len() {
                    if t[r][c] == '#' {
                        num += 1;
                    } else {
                        break;
                    }
                }
                lock.push(num);
            }
            locks.push(lock);
        } else {
            let mut key = Vec::new();
            for c in 0..t[0].len() {
                let mut num = 0;
                for r in (0..t.len()).rev() {
                    if t[r][c] == '#' {
                        num += 1;
                    } else {
                        break;
                    }
                }
                key.push(num);
            }
            keys.push(key);
        }
    }

    Ok((keys, locks))
}

fn fits(a: &[u16], b: &[u16]) -> bool {
    a.iter().zip(b.iter()).all(|(a, b)| a + b <= 7)
}

fn part_a(input: &Input) -> Option<String> {
    Some(
        input
            .0
            .iter()
            .map(|key| input.1.iter().filter(|lock| fits(&key, &lock)).count())
            .sum::<usize>()
            .to_string(),
    )
}

fn part_b(_input: &Input) -> Option<String> {
    None
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
        utils::golden("day25", &DAY, Some("3"), None, false)
    }

    #[test]
    fn finalanswer() {
        utils::finalanswer(25, &DAY, Some("3483"), None, false);
    }
}
