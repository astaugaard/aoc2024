use crate::day;
use bitvec::bitvec;
use bitvec::vec::BitVec;
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq)]
pub enum Loc {
    Start,
    End,
    Empty,
    Wall,
}

type Input = Vec<Vec<Loc>>;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    Ok(input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Loc::Empty,
                    '#' => Loc::Wall,
                    'S' => Loc::Start,
                    'E' => Loc::End,
                    _ => panic!("invalid input"),
                })
                .collect_vec()
        })
        .collect_vec())
}

fn part_a(input: &Input) -> Option<String> {
    let startLoc = input
        .iter()
        .enumerate()
        .filter_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(col, loc)| match loc {
                    Loc::Start => Some((col as i32, row as i32)),
                    _ => None,
                })
                .next()
        })
        .next()
        .unwrap();

    let endLoc = input
        .iter()
        .enumerate()
        .filter_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(col, loc)| match loc {
                    Loc::End => Some((col as i32, row as i32)),
                    _ => None,
                })
                .next()
        })
        .next()
        .unwrap();

    let distance = pathfind(startLoc, endLoc, input);

    Some(distance.to_string())
}

fn dirtoind(dir: (i32, i32)) -> usize {
    ((dir.0 + 1) / 2 + dir.1 + 2) as usize
}

fn pathfind(start_loc: (i32, i32), end_loc: (i32, i32), input: &Input) -> u32 {
    let mut current_lowest = vec![vec![vec![u32::MAX; 4]; input[0].len()]; input.len()];
    let mut queue: BinaryHeap<(Reverse<u32>, (i32, i32), (i32, i32))> = BinaryHeap::new();

    current_lowest[start_loc.1 as usize][start_loc.0 as usize][dirtoind((1, 0))] = 0;

    queue.push((Reverse(0), (1, 0), start_loc));

    while let Some((Reverse(prio), facing, loc)) = queue.pop() {
        if loc == end_loc {
            return current_lowest[end_loc.1 as usize][end_loc.0 as usize][dirtoind(facing)];
        }

        let d = current_lowest[loc.1 as usize][loc.0 as usize][dirtoind(facing)];

        for (ndir, nloc) in [(-1, 0), (1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(|(dx, dy)| ((dx, dy), (dx + loc.0, dy + loc.1)))
            .filter(|(_, (x, y))| input[*y as usize][*x as usize] != Loc::Wall)
        {
            let nd = if facing == ndir { 1 + d } else { 1001 + d };
            if nd < current_lowest[nloc.1 as usize][nloc.0 as usize][dirtoind(ndir)] {
                current_lowest[nloc.1 as usize][nloc.0 as usize][dirtoind(ndir)] = nd;
                queue.push((Reverse(nd), ndir, nloc));
            }
        }
    }

    panic!("no path through maze")
}

fn part_b(input: &Input) -> Option<String> {
    let startLoc = input
        .iter()
        .enumerate()
        .filter_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(col, loc)| match loc {
                    Loc::Start => Some((col as i32, row as i32)),
                    _ => None,
                })
                .next()
        })
        .next()
        .unwrap();

    let endLoc = input
        .iter()
        .enumerate()
        .filter_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(col, loc)| match loc {
                    Loc::End => Some((col as i32, row as i32)),
                    _ => None,
                })
                .next()
        })
        .next()
        .unwrap();

    let distance = pathfindb(startLoc, endLoc, input);

    Some(distance.to_string())
}

fn pathfindb(start_loc: (i32, i32), end_loc: (i32, i32), input: &Input) -> u32 {
    let mut current_lowest =
        vec![vec![vec![(u32::MAX, Vec::new()); 4]; input[0].len()]; input.len()];
    let mut queue: BinaryHeap<(Reverse<u32>, (i32, i32), (i32, i32))> = BinaryHeap::new();

    current_lowest[start_loc.1 as usize][start_loc.0 as usize][dirtoind((1, 0))] = (0, Vec::new());

    queue.push((Reverse(0), (1, 0), start_loc));

    while let Some((Reverse(prio), facing, loc)) = queue.pop() {
        if loc == end_loc {
            break;
        }

        let d = current_lowest[loc.1 as usize][loc.0 as usize][dirtoind(facing)].0;

        for (ndir, nloc) in [(-1, 0), (1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(|(dx, dy)| ((dx, dy), (dx + loc.0, dy + loc.1)))
            .filter(|(_, (x, y))| input[*y as usize][*x as usize] != Loc::Wall)
        {
            let nd = if facing == ndir { 1 + d } else { 1001 + d };
            if nd < current_lowest[nloc.1 as usize][nloc.0 as usize][dirtoind(ndir)].0 {
                current_lowest[nloc.1 as usize][nloc.0 as usize][dirtoind(ndir)] =
                    (nd, vec![(loc, dirtoind(facing))]);
                queue.push((Reverse(nd), ndir, nloc));
            } else if nd == current_lowest[nloc.1 as usize][nloc.0 as usize][dirtoind(ndir)].0 {
                current_lowest[nloc.1 as usize][nloc.0 as usize][dirtoind(ndir)]
                    .1
                    .push((loc, dirtoind(facing)));
            }
        }
    }

    let mut count = 0;
    let mut visited = vec![vec![bitvec![0; input[0].len()]; 4]; input.len()];

    count_num_locs(&mut visited, &mut count, &current_lowest, end_loc, 0);
    count_num_locs(&mut visited, &mut count, &current_lowest, end_loc, 1);
    count_num_locs(&mut visited, &mut count, &current_lowest, end_loc, 2);
    count_num_locs(&mut visited, &mut count, &current_lowest, end_loc, 3);

    count
}

fn count_num_locs(
    visited: &mut Vec<Vec<BitVec>>,
    count: &mut u32,
    current_lowest: &[Vec<Vec<(u32, Vec<((i32, i32), usize)>)>>],
    loc: (i32, i32),
    facing: usize,
) {
    if visited[loc.1 as usize][facing][loc.0 as usize] {
        return;
    }

    if (0..4).all(|f| !visited[loc.1 as usize][f][loc.0 as usize]) {
        *count += 1;
        // println!("loc: {:?}", loc);
    }

    visited[loc.1 as usize][facing].set(loc.0 as usize, true);

    for (loc, facing) in current_lowest[loc.1 as usize][loc.0 as usize][facing]
        .1
        .iter()
    {
        count_num_locs(visited, count, current_lowest, *loc, *facing);
    }
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
        utils::golden("day16-1", &DAY, Some("7036"), Some("45"), false);
        utils::golden("day16-2", &DAY, Some("11048"), Some("64"), false);
    }

    // #[test]
    // fn finalanswer() {
    //     utils::finalanswer(1, &DAY, Some("2057374"), Some("23177084"), false);
    // }
}
