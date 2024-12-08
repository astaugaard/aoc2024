use crate::day;
use bitvec::vec::BitVec;
use bitvec::*;
use itertools::Itertools;
use once_cell::sync::Lazy;

type Input = Vec<Vec<char>>;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    Ok(input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec())
}

fn part_a(input: &Input) -> Option<String> {
    let h = input.len();
    let w = input[0].len();

    let mut set: Vec<BitVec> = vec![bitvec![0; w]; h];

    let mut table: Vec<Vec<(usize, usize)>> = vec![Vec::new(); 256]; // no unicode support

    let mut count = 0;

    for (y, row) in input.iter().enumerate() {
        for (x, loc) in row.iter().enumerate() {
            if *loc != '.' {
                add_location(&mut set, &mut table[*loc as usize], x, y, &mut count)
            }
        }
    }

    Some(count.to_string())
}

fn add_location(
    freqs_locs: &mut [BitVec],
    match_locs: &mut Vec<(usize, usize)>,
    x: usize,
    y: usize,
    count: &mut usize,
) {
    for (x1, y1) in match_locs.iter() {
        if let Some((nx, ny)) = matching_loc(x, y, *x1, *y1) {
            if !freqs_locs
                .get(ny)
                .map_or(true, |r| *r.get(nx).as_deref().unwrap_or(&true))
            {
                freqs_locs[ny].set(nx, true);
                *count += 1;
            }
        }

        if let Some((nx, ny)) = matching_loc(*x1, *y1, x, y) {
            if !freqs_locs
                .get(ny)
                .map_or(true, |r| *r.get(nx).as_deref().unwrap_or(&true))
            {
                freqs_locs[ny].set(nx, true);
                *count += 1;
            }
        }
    }

    match_locs.push((x, y))
}

fn matching_loc(x: usize, y: usize, x1: usize, y1: usize) -> Option<(usize, usize)> {
    Some(((2 * x).checked_sub(x1)?, (2 * y).checked_sub(y1)?))
}

fn part_b(input: &Input) -> Option<String> {
    let h = input.len();
    let w = input[0].len();

    let mut set: Vec<BitVec> = vec![bitvec![0; w]; h];

    let mut table: Vec<Vec<(usize, usize)>> = vec![Vec::new(); 256]; // no unicode support

    let mut count = 0;

    for (y, row) in input.iter().enumerate() {
        for (x, loc) in row.iter().enumerate() {
            if *loc != '.' {
                add_locations(&mut set, &mut table[*loc as usize], x, y, &mut count)
            }
        }
    }

    Some(count.to_string())
}

fn add_locations(
    freqs_locs: &mut [BitVec],
    match_locs: &mut Vec<(usize, usize)>,
    x: usize,
    y: usize,
    count: &mut usize,
) {
    for (x1, y1) in match_locs.iter() {
        for (nx, ny) in matching_locs(
            x,
            y,
            *x1,
            *y1,
            freqs_locs[0].len() - 1,
            freqs_locs.len() - 1,
        ) {
            if !freqs_locs
                .get(ny)
                .map_or(true, |r| *r.get(nx).as_deref().unwrap_or(&true))
            {
                freqs_locs[ny].set(nx, true);
                *count += 1;
            }
        }

        for (nx, ny) in matching_locs(
            *x1,
            *y1,
            x,
            y,
            freqs_locs[0].len() - 1,
            freqs_locs.len() - 1,
        ) {
            if !freqs_locs
                .get(ny)
                .map_or(true, |r| *r.get(nx).as_deref().unwrap_or(&true))
            {
                freqs_locs[ny].set(nx, true);
                *count += 1;
            }
        }
    }

    match_locs.push((x, y))
}

fn matching_locs(
    x: usize,
    y: usize,
    x1: usize,
    y1: usize,
    mx: usize,
    my: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let x = x as i64;
    let y = y as i64;
    let x1 = x1 as i64;
    let y1 = y1 as i64;

    let dx = x1 - x;
    let dy = y1 - y;

    let xs = (0..)
        .map(move |i: i64| dx * i + x1)
        .take_while(move |x| *x >= 0 && *x <= mx as i64)
        .map(|i| i as usize);
    let ys = (0..)
        .map(move |i: i64| dy * i + y1)
        .take_while(move |y| *y >= 0 && *y <= my as i64)
        .map(|i| i as usize);

    xs.zip(ys)
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
        utils::golden("day8", &DAY, Some("14"), Some("34"), false)
    }

    #[test]
    fn finalanswer() {
        utils::finalanswer(8, &DAY, Some("369"), Some("1169"), false);
    }
}
