use crate::day;
use itertools::Itertools;
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct Input {
    a: usize,
    b: usize,
    c: usize,
    program: Vec<u8>,
}
impl Input {
    fn combo(&self, operand: u8) -> usize {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("reserved"),
            _ => panic!("invalid combo operand"),
        }
    }
}

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    let mut lines = input.lines();
    let a = lines.next().unwrap()[12..].parse::<usize>().unwrap();
    let b = lines.next().unwrap()[12..].parse::<usize>().unwrap();
    let c = lines.next().unwrap()[12..].parse::<usize>().unwrap();
    lines.next();
    let program = lines.next().unwrap()[9..]
        .split(',')
        .map(|c| c.parse::<u8>().unwrap())
        .collect_vec();

    Ok(Input { a, b, c, program })
}

fn part_a(input: &Input) -> Option<String> {
    let loc = 0;

    let mut input = input.clone();

    let res = simulate(loc, &mut input);

    let res = res.iter().map(|a| a.to_string()).join(",");

    Some(res)
}

fn simulate(mut loc: usize, input: &mut Input) -> Vec<u8> {
    let mut res = Vec::new();

    while loc < (input.program.len() - 1) {
        let opcode = input.program[loc];
        let operand = input.program[loc + 1];

        match opcode {
            0 => {
                let operand = input.combo(operand);
                input.a = input.a >> operand;
                loc += 2;
            }
            1 => {
                input.b = input.b ^ operand as usize;
                loc += 2;
            }
            2 => {
                let operand = input.combo(operand);
                input.b = operand % 8;
                loc += 2
            }
            3 => {
                if input.a == 0 {
                    loc += 2
                } else {
                    loc = operand as usize;
                }
            }
            4 => {
                input.b = input.b ^ input.c;
                loc += 2;
            }
            5 => {
                let operand = input.combo(operand);
                res.push((operand % 8) as u8);
                loc += 2;
            }
            6 => {
                let operand = input.combo(operand);
                input.b = input.a >> operand;
                loc += 2;
            }
            7 => {
                let operand = input.combo(operand);
                input.c = input.a >> operand;
                loc += 2;
            }
            _ => panic!("invalid number for opcode"),
        }
    }

    res
}

fn simulate_till_out(mut loc: usize, input: &mut Input) -> u8 {
    while loc < (input.program.len() - 1) {
        let opcode = input.program[loc];
        let operand = input.program[loc + 1];

        match opcode {
            0 => {
                let operand = input.combo(operand);
                input.a = input.a >> operand;
                loc += 2;
            }
            1 => {
                input.b = input.b ^ operand as usize;
                loc += 2;
            }
            2 => {
                let operand = input.combo(operand);
                input.b = operand % 8;
                loc += 2
            }
            3 => {
                if input.a == 0 {
                    loc += 2
                } else {
                    loc = operand as usize;
                }
            }
            4 => {
                input.b = input.b ^ input.c;
                loc += 2;
            }
            5 => {
                let operand = input.combo(operand);
                return ((operand % 8) as u8);
            }
            6 => {
                let operand = input.combo(operand);
                input.b = input.a >> operand;
                loc += 2;
            }
            7 => {
                let operand = input.combo(operand);
                input.c = input.a >> operand;
                loc += 2;
            }
            _ => panic!("invalid number for opcode"),
        }
    }

    panic!("no out")
}

fn part_b(input: &Input) -> Option<String> {
    // specialized to my input
    // assumes a is only divede by 8 during the course of the program before looping
    // also assumes that the program only loops at the end

    let a = search(input.program.len() - 1, 0, input).unwrap();

    //    for i in input.program.iter().rev() {
    //        println!("a before shift {a}");
    //        a <<= 3;
    //        println!("a after shift {a}");
    //        for p in 0..8usize {
    //            let mut input = input.clone();
    //            input.a = dbg!(p + a);
    //            if *i == simulate_till_out(0, &mut input) {
    //                println!("a before add: {a}");
    //                a += dbg!(p);
    //                println!("a after add: {a}");
    //                println!("hello");
    ////                break;
    //            }
    //        }
    //        println!("done loop")
    //    }

    Some(a.to_string())
}

fn search(i: usize, a: usize, input: &Input) -> Option<usize> {
    for p in 0..8usize {
        let mut input2 = input.clone();
        input2.a = (a << 3) + p;
        if input.program[i] == simulate_till_out(0, &mut input2) {
            if i == 0 {
                return Some((a << 3) + p);
            } else if let Some(val) = search(i - 1, (a << 3) + p, input) {
                return Some(val);
            }
        }
    }

    None
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

// Register A: 52042868
// Register B: 0
// Register C: 0

// Program: 3,0
//
// B = A % 8
// B = B ^ 7
// C = A / B
// A = A / 8
// B = B ^ C
// B = B ^ 7
// output (B % 8)
// if A != 0 {
//      goto 0
// }
//

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn goldens() {
        utils::golden("day17", &DAY, Some("4,6,3,5,6,3,5,2,1,0"), None, false);
        utils::golden("day17-2", &DAY, None, Some("117440"), false);
    }

    #[test]
    fn finalanswer() {
        utils::finalanswer(17, &DAY, Some("2,1,0,1,7,2,5,0,3"), Some("267265166222235"), false);
    }
}
