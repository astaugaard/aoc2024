use crate::day;
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Op {
    And,
    Or,
    XOr,
}

type Input = (Vec<(String, bool)>, Vec<(String, Op, String, String)>);

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    let (init, ops) = input.split_once("\n\n").unwrap();
    let init = init
        .lines()
        .map(|r| {
            let (name, val) = r.split_once(": ").unwrap();
            (name.to_string(), val == "1")
        })
        .collect_vec();
    let ops = ops
        .lines()
        .map(|l| {
            let mut l = l.split(' ');
            let a = l.next().unwrap().to_string();
            let op = match l.next().unwrap() {
                "AND" => Op::And,
                "OR" => Op::Or,
                "XOR" => Op::XOr,
                _ => panic!("invalid value"),
            };
            let b = l.next().unwrap().to_string();
            let _ = l.next();
            let r = l.next().unwrap().to_string();
            (a, op, b, r)
        })
        .collect_vec();

    Ok((init, ops))
}

fn part_a(input: &Input) -> Option<String> {
    let mut vals: HashMap<&str, bool> = HashMap::new();
    let mut waiting_on: HashMap<&str, Vec<usize>> = HashMap::new();

    for (id, (a, _, b, _)) in input.1.iter().enumerate() {
        waiting_on.entry(&a).or_insert(Vec::new()).push(id);
        waiting_on.entry(&b).or_insert(Vec::new()).push(id);
    }

    for (name, val) in input.0.iter() {
        vals.insert(&name, *val);
        update_waiting(&input.1, &mut waiting_on, &mut vals, &name);
    }

    let mut cur = 0;
    let mut cur_val = 0;

    let mut it = (0..).map(|a| vals.get(format!("z{:0>2}", a).as_str()));

    while let Some(val) = it.next().unwrap() {
        cur_val += (*val as u64) << cur;
        cur += 1;
    }

    Some(cur_val.to_string())
}

fn update_waiting<'a>(
    input: &'a [(String, Op, String, String)],
    waiting_on: &HashMap<&'a str, Vec<usize>>,
    vals: &mut HashMap<&'a str, bool>,
    updated: &'a str,
) {
    let Some(a) = waiting_on.get(updated) else {
        return;
    };

    for rule in a {
        let (a, op, b, c) = &input[*rule];
        let Some(a) = vals.get(a.as_str()) else {
            continue;
        };
        let Some(b) = vals.get(b.as_str()) else {
            continue;
        };

        let c_val = match op {
            Op::And => *a && *b,
            Op::Or => *a || *b,
            Op::XOr => *a ^ *b,
        };

        vals.insert(&c, c_val);
        update_waiting(input, waiting_on, vals, &c);
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
enum CircutLoc {
    HalfAnd,
    HalfXOr,
    AND1(usize),
    AND2(usize),
    Or(usize),
    XOr1(usize),
    XOr2(usize),
}

#[derive(Eq, PartialEq, Hash, Debug)]
enum OutputLoc {
    Circut(CircutLoc),
    Output(usize),
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
enum Origin {
    CircutLoc(CircutLoc),
    Input(usize),
}

fn part_b(input: &Input) -> Option<String> {
    let mut vals: HashMap<&str, Origin> = HashMap::new();
    let mut waiting_on: HashMap<&str, Vec<usize>> = HashMap::new();
    let mut to_update: Vec<&str> = Vec::new();
    let mut bad_wires: HashSet<&str> = HashSet::new();

    for (id, (a, _, b, c)) in input.1.iter().enumerate() {
        waiting_on.entry(&a).or_insert(Vec::new()).push(id);
        waiting_on.entry(&b).or_insert(Vec::new()).push(id);
        waiting_on.entry(&c).or_insert(Vec::new()).push(id);

        if c == "z45" {
            vals.insert(c, Origin::CircutLoc(CircutLoc::Or(44)));
            to_update.push(c);
        } else if c == "z00" {
            vals.insert(c, Origin::CircutLoc(CircutLoc::HalfXOr));
            to_update.push(c);
        } else if c.chars().next().unwrap() == 'z' {
            let l = c[1..].parse::<usize>().unwrap();
            vals.insert(c, Origin::CircutLoc(CircutLoc::XOr2(l)));
            to_update.push(c);
        }
    }

    for (id, _) in input.0.iter() {
        vals.insert(id, Origin::Input(id[1..].parse::<usize>().unwrap()));
        update_waitingb(&input.1, &waiting_on, &mut vals, id, &mut bad_wires);
    }

    for u in to_update.into_iter() {
        update_waitingb(&input.1, &waiting_on, &mut vals, u, &mut bad_wires);
    }

    let mut bad_wires = bad_wires.into_iter().collect_vec();

    bad_wires.sort_unstable();

    Some(bad_wires.join(","))
}

fn update_waitingb<'a>(
    input: &'a [(String, Op, String, String)],
    waiting_on: &HashMap<&'a str, Vec<usize>>,
    vals: &mut HashMap<&'a str, Origin>,
    updated: &'a str,
    bad_wires: &mut HashSet<&'a str>,
) {
    let Some(a) = waiting_on.get(updated) else {
        return;
    };

    for rule in a {
        let (an, op, bn, cn) = &input[*rule];

        match (
            vals.get(an.as_str()),
            vals.get(bn.as_str()),
            vals.get(cn.as_str()),
        ) {
            // propogation rules
            (Some(a), Some(b), None) => {
                let ea = expected_circut_loc_i(*a);
                let ea = ea
                    .iter()
                    .filter(|g| gate_type(**g) == *op)
                    .collect::<HashSet<_>>();
                let eb = expected_circut_loc_i(*b);
                let eb = eb
                    .iter()
                    .filter(|g| gate_type(**g) == *op)
                    .collect::<HashSet<_>>();

                let locs = ea.intersection(&eb).collect_vec();

                if locs.len() == 1 {
                    vals.insert(cn, Origin::CircutLoc(**locs[0]));
                    update_waitingb(input, waiting_on, vals, cn, bad_wires);
                }
            }
            (Some(a), None, Some(c)) => {
                let ea = expected_circut_loc_i(*a);
                let ea = ea
                    .iter()
                    .filter(|g| gate_type(**g) == *op)
                    .collect::<HashSet<_>>();
                let ec = expected_circut_loc_o(*c);

                let expected_type = gate_type(ec);
                let possible_locs_a = ea.iter().collect_vec();

                if expected_type != gate_type(ec) {
                    bad_wires.insert(cn.as_str());
                    if possible_locs_a.len() == 1 {
                        vals.insert(bn, other_input(**possible_locs_a[0], *a));
                        update_waitingb(input, waiting_on, vals, bn, bad_wires);
                    } else if possible_locs_a.len() == 0 {
                        panic!("both bad not allowed :(")
                    }
                } else {
                    if ea.contains(&ec) {
                        vals.insert(bn, other_input(ec, *a));
                        update_waitingb(input, waiting_on, vals, bn, bad_wires);
                    }
                }
            }

            (None, Some(b), Some(c)) => {
                let eb = expected_circut_loc_i(*b);
                let eb = eb
                    .iter()
                    .filter(|g| gate_type(**g) == *op)
                    .collect::<HashSet<_>>();
                let ec = expected_circut_loc_o(*c);

                let expected_type = gate_type(ec);
                let possible_locs_b = eb.iter().collect_vec();

                if expected_type != gate_type(ec) {
                    bad_wires.insert(cn.as_str());
                    if possible_locs_b.len() == 1 {
                        vals.insert(an, other_input(**possible_locs_b[0], *b));
                        update_waitingb(input, waiting_on, vals, an, bad_wires);
                    } else if possible_locs_b.len() == 0 {
                        panic!("both bad not allowed :(")
                    }
                } else {
                    if eb.contains(&ec) {
                        vals.insert(an, other_input(ec, *b));
                        update_waitingb(input, waiting_on, vals, an, bad_wires);
                    }
                }
            }

            // checking rule
            (Some(a), Some(b), Some(c)) => {
                let ea = expected_circut_loc_i(*a);
                let ea = ea
                    .iter()
                    .filter(|g| gate_type(**g) == *op)
                    .collect::<HashSet<_>>();
                let eb = expected_circut_loc_i(*b);
                let eb = eb
                    .iter()
                    .filter(|g| gate_type(**g) == *op)
                    .collect::<HashSet<_>>();
                let ec = expected_circut_loc_o(*c);

                if ea.contains(&ec) {
                    if eb.contains(&ec) {
                        continue; // don't need to do anything
                    } else {
                        bad_wires.insert(bn);
                    }
                } else if eb.contains(&ec) {
                    bad_wires.insert(an);
                } else {
                    bad_wires.insert(cn);
                }
            }

            (Some(a), None, None) => {
                let ea = expected_circut_loc_i(*a);
                let posible_locs_a = ea.iter().filter(|g| gate_type(**g) == *op).collect_vec();

                if posible_locs_a.len() == 0 {
                    bad_wires.insert(an);
                }
            }
            (None, Some(b), None) => {
                let eb = expected_circut_loc_i(*b);
                let posible_locs_b = eb.iter().filter(|g| gate_type(**g) == *op).collect_vec();

                if posible_locs_b.len() == 0 {
                    bad_wires.insert(bn);
                }
            }

            (None, None, Some(c)) => {
                let ec = expected_circut_loc_o(*c);

                if *op != gate_type(ec) {
                    bad_wires.insert(cn);
                }
            }

            // not enough info to do anything
            _ => {}
        }
    }
}

fn other_input(loc: CircutLoc, b: Origin) -> Origin {
    match (b, loc) {
        (_, CircutLoc::HalfAnd) | (_, CircutLoc::HalfXOr) => Origin::Input(0),

        (_, CircutLoc::XOr1(n)) | (_, CircutLoc::AND1(n)) => Origin::Input(n),

        (Origin::CircutLoc(CircutLoc::XOr1(_a)), CircutLoc::AND2(n)) if n == 1 => {
            Origin::CircutLoc(CircutLoc::HalfAnd)
        }
        (Origin::CircutLoc(CircutLoc::XOr1(_a)), CircutLoc::AND2(n)) => {
            Origin::CircutLoc(CircutLoc::Or(n - 1))
        }

        (Origin::CircutLoc(CircutLoc::Or(_a)), CircutLoc::AND2(n)) => {
            Origin::CircutLoc(CircutLoc::XOr1(n))
        }
        (Origin::CircutLoc(CircutLoc::HalfAnd), CircutLoc::AND2(n)) if n == 1 => {
            Origin::CircutLoc(CircutLoc::XOr1(n))
        }

        (Origin::CircutLoc(CircutLoc::AND1(_)), CircutLoc::Or(n)) => {
            Origin::CircutLoc(CircutLoc::AND2(n))
        }
        (Origin::CircutLoc(CircutLoc::AND2(_)), CircutLoc::Or(n)) => {
            Origin::CircutLoc(CircutLoc::AND1(n))
        }

        (Origin::CircutLoc(CircutLoc::XOr1(_a)), CircutLoc::XOr2(n)) if n == 1 => {
            Origin::CircutLoc(CircutLoc::HalfAnd)
        }
        (Origin::CircutLoc(CircutLoc::XOr1(_a)), CircutLoc::XOr2(n)) => {
            Origin::CircutLoc(CircutLoc::Or(n - 1))
        }

        (Origin::CircutLoc(CircutLoc::Or(_a)), CircutLoc::XOr2(n)) => {
            Origin::CircutLoc(CircutLoc::XOr1(n))
        }
        (Origin::CircutLoc(CircutLoc::HalfAnd), CircutLoc::XOr2(n)) if n == 1 => {
            Origin::CircutLoc(CircutLoc::XOr1(n))
        }

        // (Origin::CircutLoc(circut_loc), CircutLoc::XOr2(_)) => todo!(),
        _ => panic!("not possible"),
    }
}

fn gate_type(ec: CircutLoc) -> Op {
    match ec {
        CircutLoc::HalfAnd => Op::And,
        CircutLoc::HalfXOr => Op::XOr,
        CircutLoc::AND1(_) => Op::And,
        CircutLoc::AND2(_) => Op::And,
        CircutLoc::Or(_) => Op::Or,
        CircutLoc::XOr1(_) => Op::XOr,
        CircutLoc::XOr2(_) => Op::XOr,
    }
}

fn expected_circut_loc_i(a: Origin) -> Vec<CircutLoc> {
    match a {
        Origin::CircutLoc(circut_loc) => match circut_loc {
            CircutLoc::HalfAnd => vec![CircutLoc::XOr2(1), CircutLoc::AND2(1)],
            CircutLoc::HalfXOr => vec![],
            CircutLoc::AND1(u) => vec![CircutLoc::Or(u)],
            CircutLoc::AND2(u) => vec![CircutLoc::Or(u)],
            CircutLoc::Or(u) => vec![CircutLoc::XOr2(u + 1), CircutLoc::AND2(u + 1)],
            CircutLoc::XOr1(u) => vec![CircutLoc::XOr2(u), CircutLoc::AND2(u)],
            CircutLoc::XOr2(u) => vec![],
        },
        Origin::Input(n) if n == 0 => vec![CircutLoc::HalfAnd, CircutLoc::HalfXOr],
        Origin::Input(n) => vec![CircutLoc::XOr1(n), CircutLoc::AND1(n)],
    }
}

fn expected_circut_loc_o(c: Origin) -> CircutLoc {
    match c {
        Origin::CircutLoc(circut_loc) => circut_loc,
        Origin::Input(_) => panic!(
            "expected input and input's outputs can't be swapped so this should be impossible"
        ),
    }
}

fn exe(input: &Input) {
    print!("digraph {{ ");

    for (a, o, b, c) in input.1.iter() {
        let sh = match o {
            Op::And => "pentagon",
            Op::XOr => "hexagon",
            Op::Or => "ellipse",
        };

        let col = match o {
            Op::And => "red",
            Op::XOr => "blue",
            Op::Or => "green",
        };
        print!("{c} [shape=\"{sh}\" color=\"{col}\"]; ");
        print!("{a} -> {c}; ");
        print!("{b} -> {c}; ");
    }

    println!("}}")
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
        utils::golden("day24-1", &DAY, Some("4"), None, false);
        utils::golden("day24-2", &DAY, Some("2024"), None, false);
    }

    #[test]
    fn finalanswer() {
        utils::finalanswer(24, &DAY, Some("57344080719736"), Some("cgq,fnr,kqk,nbc,svm,z15,z23,z39"), false);
    }
}
