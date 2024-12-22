use crate::day;
use bitvec::bitvec;
use bitvec::vec::BitVec;
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::collections::{BinaryHeap, VecDeque};

type Input = Vec<(usize, usize)>;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    Ok(input
        .lines()
        .map(|line| {
            let (bef, aft) = line.split_once(',').unwrap();
            (bef.parse::<usize>().unwrap(), aft.parse::<usize>().unwrap())
        })
        .collect_vec())
}

fn part_a(input: &Input) -> Option<String> {
    Some(part_a_with(1024, 70, input))
}

fn part_a_with(num: usize, max: usize, input: &Input) -> String {
    let mut locs: Vec<BitVec> = vec![bitvec![0; max + 1]; max + 1];

    for (x, y) in input[0..num].iter() {
        locs[*y].set(*x, true);
    }

    let len = shortest_path(&locs);

    len.to_string()
}

fn shortest_path(locs: &[BitVec]) -> u32 {
    let mut queue = VecDeque::from([(0i32, 0i32, 0u32)]);
    let mut visited = vec![bitvec![0; locs.len()]; locs.len()];

    while let Some((x, y, d)) = queue.pop_front() {
        if x as usize == locs.len() - 1 && y as usize == locs.len() - 1 {
            return d;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, 1), (0, -1)].into_iter() {
            let nx = x + dx;
            let ny = y + dy;

            if nx < 0 || ny < 0 || nx as usize >= locs.len() || ny as usize >= locs.len() {
                continue;
            }

            if visited[ny as usize][nx as usize] || locs[ny as usize][nx as usize] {
                continue;
            }

            visited[ny as usize].set(nx as usize, true);

            queue.push_back((nx, ny, d + 1));
        }
    }

    panic!("no path found")
}

fn path_exist(locs: &[BitVec]) -> bool {
    // let mut queue = Vec::new();
    // queue.push((0i32, 0i32));
    // let mut visited = vec![bitvec![0; locs.len()]; locs.len()];

    // while let Some((x, y)) = queue.pop() {
    //     if x as usize == locs.len() - 1 && y as usize == locs.len() - 1 {
    //         return true;
    //     }

    //     for (dx, dy) in [(-1, 0), (1, 0), (0, 1), (0, -1)].into_iter() {
    //         let nx = x + dx;
    //         let ny = y + dy;

    //         if nx < 0 || ny < 0 || nx as usize >= locs.len() || ny as usize >= locs.len() {
    //             continue;
    //         }

    //         if visited[ny as usize][nx as usize] || locs[ny as usize][nx as usize] {
    //             continue;
    //         }

    //         visited[ny as usize].set(nx as usize, true);

    //         queue.push((nx, ny));
    //     }
    // }

    // false
    let mut queue: BinaryHeap<(i32, (i32, i32))> = BinaryHeap::new();

    let mut visited = vec![bitvec![0; locs.len()]; locs.len()];

    queue.push((0, (0, 0)));

    while let Some((prio, loc)) = queue.pop() {
        if loc.0 as usize == locs.len() - 1 && loc.1 as usize == locs.len() - 1 {
            return true;
        }

        for nloc in [(-1i32, 0i32), (1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(|(dx, dy)| (dx + loc.0, dy + loc.1))
            .filter(|(x, y)| {
                *x >= 0
                    && *y >= 0
                    && (*x as usize) < locs.len()
                    && (*y as usize) < locs.len()
                    && !locs[*y as usize][*x as usize]
            })
        {
            if !visited[nloc.1 as usize][nloc.0 as usize] {
                queue.push((nloc.0 + nloc.1, nloc));
                visited[nloc.1 as usize].set(nloc.0 as usize, true)
            }
        }
    }

    false
}

fn part_b_with(max: usize, input: &Input) -> String {
    let mut locs = vec![bitvec![0; max + 1]; max + 1];

    for (_, (x, y)) in input.iter().enumerate() {
        locs[*y].set(*x, true);
        if !path_exist(&locs) {
            return format!("{x},{y}");
        }
    }

    "".to_string()
}

fn part_b(input: &Input) -> Option<String> {
    Some(part_b_with(70, input))
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
            "day18",
            &parser,
            &|input: &Input| part_a_with(12, 6, input),
            "22",
            false,
        );
        utils::set_function(
            "day18",
            &parser,
            &|input: &Input| part_b_with(6, input),
            "6,1",
            false,
        );
    }

    #[test]
    fn finalanswer() {
        utils::finalanswer(18, &DAY, Some("276"), Some("60,37"), false);
    }
}
