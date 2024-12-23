use crate::day;
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};

type Input = Vec<(String, String)>;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    Ok(input
        .lines()
        .map(|line| {
            let (bef, aft) = line.split_once('-').unwrap();
            (bef.to_string(), aft.to_string())
        })
        .collect_vec())
}

fn part_a(input: &Input) -> Option<String> {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for (a, b) in input {
        graph.entry(&a).or_insert_with(|| Vec::new()).push(&b);
        graph.entry(&b).or_insert_with(|| Vec::new()).push(&a);
    }

    for v in graph.values_mut() {
        v.sort_unstable()
    }

    let mut found: HashSet<(&str, &str, &str)> = HashSet::new();

    let empty_vec = Vec::new();

    for loc in graph.keys().filter(|l| l.chars().next().unwrap() == 't') {
        let a = graph.get(loc).unwrap_or(&empty_vec);
        for connected in a.iter() {
            let c = graph.get(connected).unwrap_or(&empty_vec);
            add_shared(&mut found, c, connected, a, loc)
        }
    }

    Some(found.len().to_string())
}

// assumes that the &strs in a and c are already sorted
fn add_shared<'a, 'b>(
    found: &'b mut HashSet<(&'a str, &'a str, &'a str)>,
    c: &'a [&'a str],
    cs: &'a str,
    a: &'a [&'a str],
    b: &'a str,
) {
    let mut c = c.iter().peekable();

    for a in a.iter() {
        while let Some(v) = c.peek() {
            if *v < a {
                let _ = c.next();
            } else {
                break;
            }
        }

        if c.peek() == Some(&a) {
            let c = c.next().unwrap();
            let mut s = [cs, b, c];
            s.sort_unstable();

            found.insert((s[0], s[1], s[2]));
        }
    }
}

fn part_b(input: &Input) -> Option<String> {
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (a, b) in input {
        graph.entry(&a).or_insert_with(|| HashSet::new()).insert(&b);
        graph.entry(&b).or_insert_with(|| HashSet::new()).insert(&a);
    }

    let mut max_clique = HashSet::new();

    bronkerbosch(
        &mut HashSet::new(),
        graph.keys().map(|c| *c).collect::<HashSet<&str>>(),
        HashSet::with_capacity(graph.len()),
        &graph,
        &mut max_clique,
    );

    let mut max_clique: Vec<&str> = max_clique.iter().map(|s| *s).collect_vec();

    max_clique.sort_unstable();

    Some(max_clique.join(","))
}

fn bronkerbosch<'a>(
    r: &mut HashSet<&'a str>,
    mut p: HashSet<&'a str>,
    mut x: HashSet<&'a str>,
    graph: &HashMap<&'a str, HashSet<&'a str>>,
    max_clique: &mut HashSet<&'a str>,
) {
    if r.len() + p.len() < max_clique.len() {
        return;
    }

    if p.is_empty() && x.is_empty() {
        if r.len() > max_clique.len() {
            *max_clique = r.clone()
        }
        return;
    }

    let u = p
        .union(&x)
        .max_by(|a, b| graph[**a].len().cmp(&graph[**b].len()))
        .unwrap();

    let verts = p.difference(&graph[*u]).cloned().collect::<Vec<_>>();

    for v in verts.into_iter() {
        r.insert(v);
        bronkerbosch(
            r,
            p.intersection(&graph[v]).cloned().collect::<HashSet<_>>(),
            x.intersection(&graph[v]).cloned().collect::<HashSet<_>>(),
            graph,
            max_clique,
        );
        r.remove(&v);
        p.remove(&v);
        x.insert(v);
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
        utils::golden("day23", &DAY, Some("7"), Some("co,de,ka,ta"), false)
    }

    #[test]
    fn finalanswer() {
        utils::finalanswer(23, &DAY, Some("1184"), Some("hf,hz,lb,lm,ls,my,ps,qu,ra,uc,vi,xz,yv"), false);
    }
}
