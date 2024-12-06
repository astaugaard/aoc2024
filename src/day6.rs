use crate::day;
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
                .filter(|(_, c)| *c == '^')
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

    let visited = vec![bitvec![0; field[0].len()]; field.len()];

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

#[derive(Copy, Clone, Debug)]
struct Pos {
    n: bool,
    e: bool,
    w: bool,
    s: bool,
}

fn checkloop_go(
    field: &Vec<BitVec>,
    mut x: i32,
    mut y: i32,
    mut dx: i32,
    mut dy: i32,
    mut prev: Vec<Vec<Pos>>,
) -> bool {
    if y >= (field.len() as i32) || y < 0 || x < 0 || x >= (field[0].len() as i32) {
        return false;
    }

    if dir_matches(prev[y as usize][x as usize], dx, dy) {
        return true;
    }

    add_dir(&mut prev[y as usize][x as usize], dx, dy);

    advance(field, &mut x, &mut y, &mut dx, &mut dy);

    checkloop_go(field, x, y, dx, dy, prev)
}

fn dir_matches(dir: Pos, dx: i32, dy: i32) -> bool {
    if dy == -1 {
        dir.n
    } else if dy == 1 {
        dir.s
    } else if dx == 1 {
        dir.e
    } else {
        dir.w
    }
}

fn add_dir(dir: &mut Pos, dx: i32, dy: i32) {
    if dy == -1 {
        dir.n = true
    } else if dy == 1 {
        dir.s = true
    } else if dx == 1 {
        dir.e = true
    } else {
        dir.w = true
    }
}

fn checkloop(field: &mut Vec<BitVec>, x: i32, y: i32, dx: i32, dy: i32) -> bool {
    field[(y + dy) as usize].set((x + dx) as usize, true);
    let prev = vec![
        vec![
            Pos {
                n: false,
                e: false,
                w: false,
                s: false
            };
            field[0].len()
        ];
        field.len()
    ];

    let res = checkloop_go(field, x, y, dx, dy, prev);

    field[(y + dy) as usize].set((x + dx) as usize, false);

    res
}

fn find_num2(
    field: &mut Vec<BitVec>,
    visited: &mut Vec<BitVec>,
    mut x: i32,
    mut y: i32,
    sx: i32,
    sy: i32,
    mut dx: i32,
    mut dy: i32,
    count: &mut usize,
) {
    if y >= (field.len() as i32) || y < 0 || x < 0 || x >= (field[0].len() as i32) {
        return;
    }

    visited[y as usize].set(x as usize, true);

    advance2(field, &visited, &mut x, &mut y, &mut dx, &mut dy, count);

    find_num2(field, visited, x, y, sx, sy, dx, dy, count);
}

fn advance2(
    field: &mut Vec<BitVec>,
    visited: &Vec<BitVec>,
    x: &mut i32,
    y: &mut i32,
    dx: &mut i32,
    dy: &mut i32,
    count: &mut usize,
) {
    let nx = *dx + *x;
    let ny = *dy + *y;

    if ny >= (field.len() as i32) || ny < 0 || nx < 0 || nx >= (field[0].len() as i32) {
        *x = nx;
        *y = ny;
        return;
    }

    if !field[ny as usize][nx as usize] {
        if !visited[ny as usize][nx as usize] && checkloop(field, *x, *y, *dx, *dy) {
            *count += 1;
        }
        *x = nx;
        *y = ny;
        return;
    }

    let odx = *dx;
    *dx = *dy * -1;
    *dy = odx;

    advance2(field, visited, x, y, dx, dy, count)
}

fn part_b(input: &Input) -> Option<String> {
    let mut field = input.0.clone();

    let mut visited = vec![bitvec![0; field[0].len()]; field.len()];

    let mut res = 0;

    find_num2(
        &mut field,
        &mut visited,
        input.1 .0 as i32,
        input.1 .1 as i32,
        input.1 .0 as i32,
        input.1 .1 as i32,
        0,
        -1,
        &mut res,
    );

    Some(res.to_string())
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
        utils::golden("day6", &DAY, Some("41"), Some("6"), false)
    }

    #[test]
    fn finalanswer() {
        utils::finalanswer(6, &DAY, Some("5444"), Some("1946"), false);
    }
}
