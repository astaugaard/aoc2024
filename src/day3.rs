use crate::day;
use once_cell::sync::Lazy;
use regex::Regex;

type Input = String;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    Ok(input)
}

fn part_a(input: &Input) -> Option<String> {
    let re = Regex::new("mul\\((\\d{1,3}),(\\d{1,3})\\)").unwrap();

    let res = re
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [a, b])| a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap())
        .sum::<u32>();

    Some(res.to_string())
}

fn part_b(input: &Input) -> Option<String> {
    let re = Regex::new("mul\\(\\d{1,3},\\d{1,3}\\)|do\\(\\)|don't\\(\\)").unwrap();

    let mut mult = true;
    let mut sum = 0;

    for i in re.find_iter(input).map(|a| a.as_str()) {
        if i.starts_with("don") {
            mult = false;
        } else if i.starts_with('d') {
            mult = true;
        }

        if mult && i.starts_with("mul") {
            let (a, b) = i
                .split_once('(')
                .unwrap()
                .1
                .split_once(')')
                .unwrap()
                .0
                .split_once(',')
                .unwrap();

            sum += a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap();
        }
    }

    Some(sum.to_string())
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
        utils::golden("day3", &DAY, Some("161"), None, false)
    }

    #[test]
    fn finalanswer() {
        utils::finalanswer(3, &DAY, Some("179571322"), Some("103811193"), false)
    }
}
