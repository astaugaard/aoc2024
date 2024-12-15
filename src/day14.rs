use crate::day;
use image::Rgb;
use itertools::Itertools;
use once_cell::sync::Lazy;

type Input = Vec<(i32, i32, i32, i32)>;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    Ok(input
        .lines()
        .map(|line| {
            let (pos, vel) = line[2..].split_once(" v=").unwrap();
            let (x, y) = pos.split_once(',').unwrap();
            let (vx, vy) = vel.split_once(',').unwrap();
            (
                x.parse::<i32>().unwrap(),
                y.parse::<i32>().unwrap(),
                vx.parse::<i32>().unwrap(),
                vy.parse::<i32>().unwrap(),
            )
        })
        .collect_vec())
}

fn solve_steps(input: &Input, sx: i32, sy: i32, steps: i32) -> (u32, u32, u32, u32) {
    let stepsy = steps.rem_euclid(sy);
    let stepsx = steps.rem_euclid(sx);
    let cx = sx / 2; // assumes that sx is odd
    let cy = sy / 2; // assumes that sy is odd

    input
        .iter()
        .flat_map(|(x, y, vx, vy)| {
            let fx = (vx * stepsx + x).rem_euclid(sx);
            let fy = (vy * stepsy + y).rem_euclid(sy);

            if fx > cx && fy < cy {
                Some(0)
            } else if fx < cx && fy < cy {
                Some(1)
            } else if fx < cx && fy > cy {
                Some(2)
            } else if fx > cx && fy > cy {
                Some(3)
            } else {
                None
            }
        })
        .fold((0, 0, 0, 0), |mut acc, quad| {
            match quad {
                0 => acc.0 += 1,
                1 => acc.1 += 1,
                2 => acc.2 += 1,
                3 => acc.3 += 1,
                _ => panic!("not possible"),
            }
            acc
        })
}

fn part_a(input: &Input) -> Option<String> {
    let (a, b, c, d) = solve_steps(input, 101, 103, 100);

    Some((a * b * c * d).to_string())
}

fn run_steps(input: &Input, sx: i32, sy: i32, steps: i32) {
    let stepsy = steps.rem_euclid(sy);
    let stepsx = steps.rem_euclid(sx);

    let mut img = image::ImageBuffer::new(sx as u32, sy as u32);

    for (fx, fy) in input.iter().map(|(x, y, vx, vy)| {
        let fx = (vx * stepsx + x).rem_euclid(sx);
        let fy = (vy * stepsy + y).rem_euclid(sy);

        (fx, fy)
    }) {
        img.put_pixel(fx as u32, fy as u32, Rgb([255u8, 255, 255]));
    }

    img.save(format!("/home/a/day14result/{steps}.png")).unwrap();
}

fn part_b(_input: &Input) -> Option<String> {
    None
}

fn exe(input: &Input) {
    let mut i = 1;

    loop {
        println!("i = {i}");
        run_steps(input, 101, 103, i);
        i += 1;

        if i > 10000 {
            break;
        }
    }
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
    use crate::utils::set_function;

    #[test]
    fn goldens() {
        set_function(
            "day14",
            &parser,
            &|input: &Input| {
                let (a, b, c, d) = solve_steps(input, 11, 7, 100);
                (a * b * c * d).to_string()
            },
            "12",
            false,
        );
    }

    // #[test]
    // fn finalanswer() {
    //     utils::finalanswer(1, &DAY, Some("2057374"), Some("23177084"), false);
    // }
}
