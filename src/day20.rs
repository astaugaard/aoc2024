use crate::day;
use itertools::Itertools;
use once_cell::sync::Lazy;

#[derive(Debug, PartialEq, Eq)]
pub enum Loc {
    Wall,
    Start,
    End,
    Empty,
}

type Input = Vec<Vec<Loc>>;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    Ok(input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Loc::Wall,
                    'S' => Loc::Start,
                    'E' => Loc::End,
                    '.' => Loc::Empty,
                    _ => panic!("invalid input"),
                })
                .collect_vec()
        })
        .collect_vec())
}

fn part_a(input: &Input) -> Option<String> {
    Some(parts_with(100, 2, input))
}

struct Path<'a> {
    x: i32,
    lx: i32,
    y: i32,
    ly: i32,
    input: &'a Input,
}

impl<'a> Iterator for Path<'a> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.input[self.y as usize][self.x as usize] == Loc::End {
            return None;
        }

        let (nx, ny) = [(-1, 0), (1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(|(dx, dy)| (self.x + dx, self.y + dy))
            .filter(|(x, y)| {
                (*x != self.lx || *y != self.ly)
                    && *x > 0
                    && *y > 0
                    && *x as usize <= self.input[0].len()
                    && *y as usize <= self.input.len()
                    && self.input[*y as usize][*x as usize] != Loc::Wall
            })
            .next()
            .unwrap();

        self.lx = self.x;
        self.ly = self.y;
        self.x = nx;
        self.y = ny;

        Some((self.x, self.y))
    }
}

fn path_from(x: usize, y: usize, input: &Input) -> Path {
    Path {
        x: x as i32,
        lx: 0,
        y: y as i32,
        ly: 0,
        input,
    }
}

fn parts_with(cutoff: u32, md: i32, input: &Input) -> String {
    let start_loc = input
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.iter().enumerate().filter_map(move |(x, c)| match c {
                Loc::Start => Some((x, y)),
                _ => None,
            })
        })
        .next()
        .unwrap();

    let mut distances = vec![vec![0; input[0].len()]; input.len()];
    let mut d = 0;

    for (x, y) in path_from(start_loc.0, start_loc.1, input) {
        d += 1;
        distances[y as usize][x as usize] = d;
    }

    let mut skips = 0;

    check_skipsb(
        &mut skips,
        cutoff,
        &distances,
        start_loc.0 as i32,
        start_loc.1 as i32,
        input,
        md,
    );

    for (x, y) in path_from(start_loc.0, start_loc.1, input) {
        check_skipsb(&mut skips, cutoff, &distances, x, y, input, md);
    }

    skips.to_string()
}

fn check_skipsb(
    skips: &mut usize,
    cutoff: u32,
    dists: &[Vec<u32>],
    x: i32,
    y: i32,
    input: &Input,
    md: i32,
) {
    let loc_dist = dists[y as usize][x as usize];

    let num_skip = (-md..=md)
        .flat_map(|y| {
            let dx = md - y.abs();
            (-dx..=dx).map(move |x| (x, y))
        })
        .into_iter()
        .map(|(dx, dy)| (x + dx, y + dy, dx.abs() + dy.abs()))
        .filter(|(x, y, t)| {
            *t > 0
                && *x > 0
                && *y > 0
                && (*x as usize) < input[0].len()
                && (*y as usize) < input.len()
                && dists[*y as usize][*x as usize] >= (loc_dist + cutoff + (*t as u32 - 1))
        })
        .count();

    *skips += num_skip;
}

fn part_b(input: &Input) -> Option<String> {
    Some(parts_with(100, 20, input))
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
        utils::set_function(
            "day20",
            &parser,
            &|input| parts_with(12, 2, input),
            "8",
            false,
        );

        utils::set_function(
            "day20",
            &parser,
            &|input| parts_with(50, 20, input),
            "285",
            false,
        );
    }

    #[test]
    fn finalanswer() {
        utils::finalanswer(20, &DAY, Some("1530"), Some("1033983"), false);
    }
}
