use crate::day;
use itertools::{chain, repeat_n, Itertools};
use once_cell::sync::Lazy;
use std::cmp;
use std::collections::HashMap;
use typed_arena::Arena;

type Input = Vec<String>;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    Ok(input.lines().map(|a| a.to_string()).collect_vec())
}

struct NumPad<'a> {
    // might not need to store it but doing it for now in case I do need to
    // probaly actually doesn't store anything
    keypad: &'a mut dyn KeyPadTrait,
}

#[derive(Debug, Clone, Copy)]
enum KeyPadButtons {
    Left,
    Right,
    Up,
    Down,
    A,
}

trait KeyPadTrait {
    fn dist(&mut self, presses: &mut dyn Iterator<Item = KeyPadButtons>) -> u64;
}

struct KeyPad<'a> {
    cache: HashMap<((u32, u32), (u32, u32)), u64>,
    keypad: &'a mut dyn KeyPadTrait,
}

impl<'a> KeyPadTrait for KeyPad<'a> {
    fn dist(&mut self, presses: &mut dyn Iterator<Item = KeyPadButtons>) -> u64 {
        let mut x = 2u32;
        let mut y = 0u32;
        let mut num = 0;

        for b in presses.into_iter() {
            let (nx, ny) = match b {
                KeyPadButtons::A => (2, 0),
                KeyPadButtons::Up => (1, 0),
                KeyPadButtons::Down => (1, 1),
                KeyPadButtons::Left => (0, 1),
                KeyPadButtons::Right => (2, 1),
            };

            if let Some(cost) = self.cache.get(&((x, y), (nx, ny))) {
                num += *cost;
                x = nx;
                y = ny;
                continue;
            }

            let dx = x.abs_diff(nx);
            let dy = y.abs_diff(ny);
            let vert = if ny < y {
                KeyPadButtons::Up
            } else {
                KeyPadButtons::Down
            };
            let horz = if nx < x {
                KeyPadButtons::Left
            } else {
                KeyPadButtons::Right
            };

            let dy = dy as usize;
            let dx = dx as usize;

            let low = if dx == 0 {
                self.keypad
                    .dist(&mut chain!(repeat_n(vert, dy), [KeyPadButtons::A]).into_iter())
            } else if dy == 0 {
                self.keypad
                    .dist(&mut chain!(repeat_n(horz, dx), [KeyPadButtons::A]).into_iter())
            } else {
                if x == 0 {
                    self.keypad.dist(
                        &mut chain!(repeat_n(horz, dx), repeat_n(vert, dy), [KeyPadButtons::A])
                            .into_iter(),
                    )
                } else if nx == 0 {
                    self.keypad.dist(
                        &mut chain!(repeat_n(vert, dy), repeat_n(horz, dx), [KeyPadButtons::A])
                            .into_iter(),
                    )
                } else {
                    cmp::min(
                        self.keypad.dist(
                            &mut chain!(repeat_n(vert, dy), repeat_n(horz, dx), [KeyPadButtons::A])
                                .into_iter(),
                        ),
                        self.keypad.dist(
                            &mut chain!(repeat_n(horz, dx), repeat_n(vert, dy), [KeyPadButtons::A])
                                .into_iter(),
                        ),
                    )
                }
            };

            num += low;

            self.cache.insert(((x, y), (nx, ny)), low);
            x = nx;
            y = ny;
        }

        num
    }
}

struct FinalKeyPad {}

impl KeyPadTrait for FinalKeyPad {
    fn dist(&mut self, presses: &mut dyn Iterator<Item = KeyPadButtons>) -> u64 {
        presses.into_iter().count() as u64
    }
}

impl<'a> NumPad<'a> {
    fn new(a: &'a mut dyn KeyPadTrait) -> Self {
        Self { keypad: a }
    }

    fn num_dist(&mut self, input: &str) -> u64 {
        let mut x: u32 = 2;
        let mut y: u32 = 3;
        let mut num = 0;

        for b in input.chars() {
            let (nx, ny) = match b {
                '0' => (1, 3),
                '1' => (0, 2),
                '2' => (1, 2),
                '3' => (2, 2),
                '4' => (0, 1),
                '5' => (1, 1),
                '6' => (2, 1),
                '7' => (0, 0),
                '8' => (1, 0),
                '9' => (2, 0),
                'A' => (2, 3),
                _ => panic!("invalid character"),
            };

            let dx = x.abs_diff(nx);
            let dy = y.abs_diff(ny);
            let vert = if ny < y {
                KeyPadButtons::Up
            } else {
                KeyPadButtons::Down
            };
            let horz = if nx < x {
                KeyPadButtons::Left
            } else {
                KeyPadButtons::Right
            };

            let dy = dy as usize;
            let dx = dx as usize;

            if dx == 0 {
                num += self
                    .keypad
                    .dist(&mut chain!(repeat_n(vert, dy), [KeyPadButtons::A]).into_iter());
            } else if dy == 0 {
                num += self
                    .keypad
                    .dist(&mut chain!(repeat_n(horz, dx), [KeyPadButtons::A]).into_iter());
            } else {
                if y == 3 && nx == 0 {
                    num += self.keypad.dist(
                        &mut chain!(repeat_n(vert, dy), repeat_n(horz, dx), [KeyPadButtons::A])
                            .into_iter(),
                    );
                } else if ny == 3 && x == 0 {
                    num += self.keypad.dist(
                        &mut chain!(repeat_n(horz, dx), repeat_n(vert, dy), [KeyPadButtons::A])
                            .into_iter(),
                    );
                } else {
                    num += cmp::min(
                        self.keypad.dist(
                            &mut chain!(repeat_n(vert, dy), repeat_n(horz, dx), [KeyPadButtons::A])
                                .into_iter(),
                        ),
                        self.keypad.dist(
                            &mut chain!(repeat_n(horz, dx), repeat_n(vert, dy), [KeyPadButtons::A])
                                .into_iter(),
                        ),
                    );
                }
            }

            x = nx;
            y = ny;
        }

        num
    }
}

impl<'a> KeyPad<'a> {
    fn new(a: &'a mut dyn KeyPadTrait) -> Self {
        Self {
            keypad: a,
            cache: HashMap::new(),
        }
    }
}

impl FinalKeyPad {
    fn new() -> Self {
        Self {}
    }
}

fn part_a(input: &Input) -> Option<String> {
    let bump = Arena::new();
    let mut keypad = FinalKeyPad::new();
    let mut keypad = make_key_pad(2, &mut keypad, &bump);

    Some(
        input
            .iter()
            .map(|a| keypad.num_dist(&a) * a[0..a.len() - 1].parse::<u64>().unwrap())
            .sum::<u64>()
            .to_string(),
    )
}

fn make_key_pad<'a>(num: usize, f: &'a mut FinalKeyPad, bump: &'a Arena<KeyPad<'a>>) -> NumPad<'a> {
    let mut current: &mut dyn KeyPadTrait = f;

    for _ in 0..num {
        current = bump.alloc(KeyPad::new(current));
    }

    NumPad::new(current)
}

fn part_b(input: &Input) -> Option<String> {
    let bump = Arena::new();
    let mut keypad = FinalKeyPad::new();
    let mut keypad = make_key_pad(25, &mut keypad, &bump);

    Some(
        input
            .iter()
            .map(|a| keypad.num_dist(&a) * a[0..a.len() - 1].parse::<u64>().unwrap())
            .sum::<u64>()
            .to_string(),
    )
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
        utils::golden(
            "day21",
            &DAY,
            Some("126384"),
            Some("154115708116294"),
            false,
        )
    }

    #[test]
    fn t029a() {
        let bump = Arena::new();
        let mut keypad = FinalKeyPad::new();
        let mut keypad = make_key_pad(2, &mut keypad, &bump);

        assert_eq!(keypad.num_dist("029A"), 68)
    }

    #[test]
    fn t029a25() {
        let bump = Arena::new();
        let mut keypad = FinalKeyPad::new();
        let mut keypad = make_key_pad(25, &mut keypad, &bump);

        assert_eq!(keypad.num_dist("029A"), 82050061710)
    }

    #[test]
    fn finalanswer() {
        utils::finalanswer(21, &DAY, Some("123096"), Some("154517692795352"), false);
    }
}
