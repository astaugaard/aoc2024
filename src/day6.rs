use crate::day;
use crate::utils;
use bitvec::bitvec;
use bitvec::vec::BitVec;
use itertools::Itertools;
use once_cell::sync::Lazy;

type Input = (Vec<BitVec>, (usize, usize));

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    let loc = input
        .lines()
        .enumerate()
        .filter_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(x, c)| *c == '^')
                .next()
                .map(|(x, _)| (x, y))
        })
        .next()
        .unwrap();

    Ok((
        input
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect::<BitVec>())
            .collect_vec(),
        loc,
    ))
}

fn find_num(
    field: &Vec<BitVec>,
    mut visited: Vec<BitVec>,
    mut x: i32,
    mut y: i32,
    mut dx: i32,
    mut dy: i32,
    count: &mut usize,
) {
    if y >= (field.len() as i32) || y < 0 || x < 0 || x >= (field[0].len() as i32) {
        return;
    }

    if !visited[y as usize][x as usize] {
        *count += 1;
    }

    visited[y as usize].set(x as usize, true);

    advance(field, &mut x, &mut y, &mut dx, &mut dy);

    find_num(field, visited, x, y, dx, dy, count);
}

fn advance(field: &Vec<BitVec>, x: &mut i32, y: &mut i32, dx: &mut i32, dy: &mut i32) {
    let nx = *dx + *x;
    let ny = *dy + *y;

    if ny >= (field.len() as i32) || ny < 0 || nx < 0 || nx >= (field[0].len() as i32) {
        *x = nx;
        *y = ny;
        return;
    }

    if !field[ny as usize][nx as usize] {
        *x = nx;
        *y = ny;
        return;
    }

    let odx = *dx;
    *dx = *dy * -1;
    *dy = odx;

    advance(field, x, y, dx, dy);
}

fn part_a(input: &Input) -> Option<String> {
    let field = &input.0;

    let mut visited = vec![bitvec![0; field[0].len()]; field.len()];

    let mut res = 0;

    find_num(
        field,
        visited,
        input.1 .0 as i32,
        input.1 .1 as i32,
        0,
        -1,
        &mut res,
    );

    Some(res.to_string())
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

    #[test]
    fn goldens() {
        utils::golden("day6", &DAY, Some("41"), None, false)
    }

    // #[test]
    // fn finalanswer() {
    //     utils::finalanswer(1, &DAY, Some("2057374"), Some("23177084"), false);

    // }
}
