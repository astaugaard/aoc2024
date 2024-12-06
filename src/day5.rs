use crate::day;
use crate::utils;
use inttable::IntTable;
use itertools::Itertools;
use once_cell::sync::Lazy;

type Input = (Vec<(u64, u64)>, Vec<Vec<u64>>);

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    let (b, a) = input.split_once("\n\n").unwrap();

    let beforeConds = b
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('|').unwrap();
            (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap())
        })
        .collect_vec();
    let afterRows = a
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<u64>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    Ok((beforeConds, afterRows))
}

fn valid_edit(edit: &Vec<u64>, after: &IntTable<Vec<u64>>) -> bool {
    let mut used: IntTable<()> = IntTable::with_capacity(edit.len() * 2);

    for page in edit {
        for needAfter in after.get(*page).unwrap_or(&Vec::new()) {
            if used.contains_key(*needAfter) {
                return false;
            }
        }
        let _ = used.insert(*page, ());
    }

    true
}

fn part_a(input: &Input) -> Option<String> {
    let mut after = IntTable::new();

    for (bef, aft) in input.0.iter() {
        after.entry(*bef).or_insert(Vec::new()).push(*aft)
    }

    let res: u64 = input
        .1
        .iter()
        .filter(|line| valid_edit(&line, &after))
        .map(|line| line[line.len() / 2])
        .sum();

    Some(res.to_string())
}

fn fix_edit(edit: &Vec<u64>, after: &IntTable<Vec<u64>>) -> Vec<u64> {
    let mut used: IntTable<()> = IntTable::with_capacity(edit.len() * 2);
    let mut res = Vec::with_capacity(edit.len());
    let mut contains = IntTable::with_capacity(edit.len() * 2);

    for i in edit {
        let _ = contains.insert(*i, ());
    }

    for i in edit {
        dfs(&mut res, &mut used, after, &contains, *i);
    }

    println!("{:?}", res);

    res
}

fn dfs(res: &mut Vec<u64>, used: &mut IntTable<()>, after: &IntTable<Vec<u64>>, contains: &IntTable<()>, i: u64) {
    if used.contains_key(i) {
        return;
    }

    if !contains.contains_key(i) {
	return;
    }

    used.insert(i, ());

    for loc in after.get(i).unwrap_or(&Vec::new()) {
        dfs(res, used, after, contains, *loc)
    }

    res.push(i)
}

fn part_b(input: &Input) -> Option<String> {

    println!("{:#?}", input);
    let mut after = IntTable::new();

    for (bef, aft) in input.0.iter() {
        after.entry(*bef).or_insert(Vec::new()).push(*aft)
    }

    let res: u64 = input
        .1
        .iter()
        .filter(|line| !valid_edit(line, &after))
        .map(|line| fix_edit(line, &after))
        .map(|line| line[line.len() / 2])
        .sum();

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
    #[test]
    fn goldens() {
        utils::golden("day5", &DAY, Some("143"), Some("123"), false)
    }
    #[test]
    fn finalanswer() {
        utils::finalanswer(5, &DAY, Some("5713"), Some("5180"), false);
    }
}
