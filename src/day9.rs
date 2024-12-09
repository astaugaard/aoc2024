use std::collections::{VecDeque};

use crate::day;
use itertools::Itertools;
use once_cell::sync::Lazy;

type Input = Vec<u8>;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    Ok(input
        .lines()
        .next() // to strip any trailing new lines
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect_vec())
}

struct GetNBack<'a> {
    input: &'a mut VecDeque<(usize, u8, bool)>,
    left: u8,
}

impl<'a> Iterator for GetNBack<'a> {
    type Item = (usize, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.left == 0 {
            return None;
        }

        let back = self.input.back()?;
        if !back.2 {
            self.input.pop_back();
        }

        let back = self.input.back_mut()?;

        if back.1 <= self.left {
            let (id, num, _) = *back;
            self.input.pop_back();
            self.left -= num;
            Some((id, num))
        } else {
            let used = self.left;
            back.1 -= used;
            self.left = 0;
            Some((back.0, used))
        }
    }
}

fn part_a(input: &Input) -> Option<String> {
    let mut input = input
        .iter()
        .zip([true, false].iter().cycle())
        .enumerate()
        .map(|(id, (num, used))| (id / 2, *num, *used))
        .collect::<VecDeque<_>>();

    let mut sum = 0;
    let mut i = 0;

    while !input.is_empty() {
        let (id, num, used) = input.pop_front().unwrap();

        let num = num as usize;

        if used {
            sum += id * (num * i + (num * (num - 1)) / 2);
            i += num;
        } else {
            for (id, num) in getNback(&mut input, num) {
                let num = num as usize;
                sum += id * (num * i + (num * (num - 1)) / 2);
                i += num;
            }
        }
    }

    Some(sum.to_string())
}

fn getNback(input: &mut VecDeque<(usize, u8, bool)>, num: usize) -> GetNBack {
    GetNBack {
        input,
        left: num as u8,
    }
}

fn part_b(input: &Input) -> Option<String> {
    let mut free_space = Vec::with_capacity(input.len() / 2 + 1);
    let mut files = Vec::with_capacity(input.len() / 2 + 1);

    let mut i = 0;

    for (id, num, used) in input
        .iter()
        .zip([true, false].iter().cycle())
        .enumerate()
        .map(|(id, (num, used))| (id / 2, *num, *used))
    {
        if used {
            files.push((id, num, i))
        } else {
            free_space.push((i, num))
        }

        i += num as usize;
    }

    let mut sum = 0;

    for (id, num, loc) in files.into_iter().rev() {
        let loc = match try_alloc_free_before(&mut free_space, num, loc) {
            Some(loc) => loc,
            None => loc,
        };

        let num = num as usize;

        sum += id * (num * loc + (num * (num - 1)) / 2);
    }

    Some(sum.to_string())
}

fn try_alloc_free_before(free_space: &mut [(usize, u8)], num: u8, loc: usize) -> Option<usize> {
    for (floc,fnum) in free_space.iter_mut() {
        if *floc >= loc {
            return None;
        }
        if *fnum >= num {
            let loc = *floc;

            *fnum -= num;
            *floc += num as usize;

            return Some(loc);
        } 
    }

    return None;
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
        utils::golden("day9", &DAY, Some("1928"), Some("2858"), false)
    }

    #[test]
    fn finalanswer() {
    	utils::finalanswer(9, &DAY, Some("6367087064415"), Some("6390781891880"), false);
    }
}
