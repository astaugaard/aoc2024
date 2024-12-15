use crate::day;
use itertools::Itertools;
use once_cell::sync::Lazy;

#[derive(Debug, PartialOrd, PartialEq, Eq, Ord, Clone, Copy)]
pub enum Loc {
    Wall,
    Box,
    Bot,
    Empty,
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Ord, Clone, Copy)]
pub enum Instr {
    Up,
    Down,
    Left,
    Right,
}

type Input = (Vec<Vec<Loc>>, Vec<Instr>);

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    let (field, instrs) = input.split_once("\n\n").unwrap();

    let instrs = instrs
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '^' => Instr::Up,
                '<' => Instr::Left,
                '>' => Instr::Right,
                'v' => Instr::Down,
                _ => panic!("invalid input"),
            })
        })
        .collect_vec();

    let field = field
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Loc::Wall,
                    'O' => Loc::Box,
                    '@' => Loc::Bot,
                    '.' => Loc::Empty,
                    _ => panic!("invalid input"),
                })
                .collect_vec()
        })
        .collect_vec();

    Ok((field, instrs))
}

fn part_a(input: &Input) -> Option<String> {
    let mut field = input.0.clone();

    let robot_loc = field
        .iter_mut()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter_mut()
                .enumerate()
                .filter_map(|(c, l): (usize, &mut Loc)| {
                    let res = *l == Loc::Bot;
                    if res {
                        *l = Loc::Empty;
                        Some((c as i32, row as i32))
                    } else {
                        None
                    }
                })
                .next()
        })
        .next()
        .unwrap();

    simulate(robot_loc, &input.1, &mut field);

    Some(score_field(field).to_string())
}

fn score_field(field: Vec<Vec<Loc>>) -> u32 {
    field
        .into_iter()
        .enumerate()
        .map(|(r, line)| {
            line.into_iter()
                .enumerate()
                .map(|(c, l)| match l {
                    Loc::Box => r as u32 * 100 + c as u32,
                    _ => 0,
                })
                .sum::<u32>()
        })
        .sum::<u32>()
}

fn simulate(mut robot_loc: (i32, i32), instrs: &[Instr], field: &mut [Vec<Loc>]) {
    for i in instrs {
        simulate_instr(&mut robot_loc, *i, field)
    }
}

fn simulate_instr(robot_loc: &mut (i32, i32), i: Instr, field: &mut [Vec<Loc>]) {
    let dir = match i {
        Instr::Up => (0, -1),
        Instr::Down => (0, 1),
        Instr::Left => (-1, 0),
        Instr::Right => (1, 0),
    };

    let nx = dir.0 + robot_loc.0;
    let ny = dir.1 + robot_loc.1;

    if field[ny as usize][nx as usize] == Loc::Empty {
        robot_loc.0 = nx;
        robot_loc.1 = ny;
        return;
    }

    let mut sx = nx;
    let mut sy = ny;

    while field[sy as usize][sx as usize] == Loc::Box {
        sx += dir.0;
        sy += dir.1;
    }

    match field[sy as usize][sx as usize] {
        Loc::Box => panic!("should never happen do to while loop above"),
        Loc::Wall => return,
        Loc::Bot => panic!("should be removed already"),
        Loc::Empty => {
            field[sy as usize][sx as usize] = Loc::Box;
            field[ny as usize][nx as usize] = Loc::Empty;

            robot_loc.0 = nx;
            robot_loc.1 = ny;
        }
    }
}

enum NLoc {
    Wall,
    BoxL,
    BoxR,
    Empty,
}

fn part_b(input: &Input) -> Option<String> {
    let mut field = input.0.clone();

    let robot_loc = field
        .iter_mut()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter_mut()
                .enumerate()
                .filter_map(|(c, l): (usize, &mut Loc)| {
                    let res = *l == Loc::Bot;
                    if res {
                        *l = Loc::Empty;
                        Some((c as i32 * 2, row as i32))
                    } else {
                        None
                    }
                })
                .next()
        })
        .next()
        .unwrap();

    let mut field = field
        .into_iter()
        .map(|row| {
            row.into_iter()
                .flat_map(|loc| match loc {
                    Loc::Wall => [NLoc::Wall, NLoc::Wall],
                    Loc::Box => [NLoc::BoxL, NLoc::BoxR],
                    Loc::Bot => panic!("should have been removed"),
                    Loc::Empty => [NLoc::Empty, NLoc::Empty],
                })
                .collect_vec()
        })
        .collect_vec();

    simulate2(robot_loc, &input.1, &mut field);

    // for line in field.iter() {
    //     println!(
    //         "{}",
    //         line.iter()
    //             .map(|l| match l {
    //                 NLoc::Wall => '#',
    //                 NLoc::BoxL => '[',
    //                 NLoc::BoxR => ']',
    //                 NLoc::Empty => '.',
    //             })
    //             .collect::<String>()
    //     );
    // }

    Some(score_field2(field).to_string())
}

fn score_field2(field: Vec<Vec<NLoc>>) -> u32 {
    field
        .into_iter()
        .enumerate()
        .map(|(r, line)| {
            line.into_iter()
                .enumerate()
                .map(|(c, l)| match l {
                    NLoc::BoxL => r as u32 * 100 + c as u32,
                    _ => 0,
                })
                .sum::<u32>()
        })
        .sum::<u32>()
}

fn simulate2(mut robot_loc: (i32, i32), instrs: &[Instr], field: &mut [Vec<NLoc>]) {
    for i in instrs {
        simulate_instr2(&mut robot_loc, *i, field)
    }
}

fn simulate_instr2(robot_loc: &mut (i32, i32), i: Instr, field: &mut [Vec<NLoc>]) {
    let dir = match i {
        Instr::Up => (0, -1),
        Instr::Down => (0, 1),
        Instr::Left => (-1, 0),
        Instr::Right => (1, 0),
    };

    let nx = dir.0 + robot_loc.0;
    let ny = dir.1 + robot_loc.1;

    if can_push(nx, ny, dir, field) {
        push(nx, ny, dir, field);
        robot_loc.0 = nx;
        robot_loc.1 = ny;
    }
}

fn push(nx: i32, ny: i32, dir: (i32, i32), field: &mut [Vec<NLoc>]) {
    match field[ny as usize][nx as usize] {
        NLoc::Wall => panic!("should never hit this branch"),
        NLoc::BoxL => {
            if dir.0 == 0 {
                // verticle
                push(nx, ny + dir.1, dir, field);
                field[(ny + dir.1) as usize][nx as usize] = NLoc::BoxL;

                push(nx + 1, ny + dir.1, dir, field);
                field[(ny + dir.1) as usize][(nx + 1) as usize] = NLoc::BoxR;

                field[ny as usize][(nx + 1) as usize] = NLoc::Empty;
            } else {
                // horizontal
                if dir.0 == -1 {
                    push(nx - 1, ny, dir, field);
                    field[ny as usize][(nx - 1) as usize] = NLoc::BoxL;
                } else {
                    push(nx + 2, ny, dir, field);
                    field[ny as usize][(nx + 1) as usize] = NLoc::BoxL;
                    field[ny as usize][(nx + 2) as usize] = NLoc::BoxR;
                }
            }
        }
        NLoc::BoxR => {
            if dir.0 == 0 {
                // vertical
                push(nx - 1, ny + dir.1, dir, field);
                field[(ny + dir.1) as usize][(nx - 1) as usize] = NLoc::BoxL;

                push(nx, ny + dir.1, dir, field);
                field[(ny + dir.1) as usize][nx as usize] = NLoc::BoxR;

                field[ny as usize][(nx - 1) as usize] = NLoc::Empty;
            } else {
                // horizontal
                if dir.0 == -1 {
                    push(nx - 2, ny, dir, field);
                    field[ny as usize][(nx - 1) as usize] = NLoc::BoxR;
                    field[ny as usize][(nx - 2) as usize] = NLoc::BoxL;
                } else {
                    push(nx + 1, ny, dir, field);
                    field[ny as usize][(nx + 1) as usize] = NLoc::BoxR;
                }
            }
        }
        NLoc::Empty => {}
    }

    field[ny as usize][nx as usize] = NLoc::Empty;
}

fn can_push(nx: i32, ny: i32, dir: (i32, i32), field: &[Vec<NLoc>]) -> bool {
    match field[ny as usize][nx as usize] {
        NLoc::Wall => false,
        NLoc::BoxL => {
            if dir.0 == 0 {
                // verticle
                can_push(nx, ny + dir.1, dir, field) && can_push(nx + 1, ny + dir.1, dir, field)
            } else {
                // horizontal
                if dir.0 == -1 {
                    can_push(nx - 1, ny, dir, field)
                } else {
                    can_push(nx + 2, ny, dir, field)
                }
            }
        }
        NLoc::BoxR => {
            if dir.0 == 0 {
                // vertical
                can_push(nx - 1, ny + dir.1, dir, field) && can_push(nx, ny + dir.1, dir, field)
            } else {
                // horizontal
                if dir.0 == -1 {
                    can_push(nx - 2, ny, dir, field)
                } else {
                    can_push(nx + 1, ny, dir, field)
                }
            }
        }
        NLoc::Empty => true,
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
        utils::golden("day15-s", &DAY, Some("2028"), None, false);
        utils::golden("day15-e", &DAY, None, Some("618"), false);
        utils::golden("day15-l", &DAY, Some("10092"), Some("9021"), false)
    }

    #[test]
    fn finalanswer() {
        utils::finalanswer(15, &DAY, Some("1349898"), Some("1376686"), false);
    }
}
