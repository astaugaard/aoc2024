use crate::day;
use crate::utils;
use itertools::Itertools;
use itertools::*;
use once_cell::sync::Lazy;

type Input = Vec<Vec<i32>>;

fn parser(input: String, _verbose: bool) -> Result<Input, String> {
    input
        .lines()
        .map(|row| {
            row.split_whitespace()
                .map(|a| a.parse::<i32>().map_err(|err| err.to_string()))
                .process_results(|row| row.collect_vec())
        })
        .process_results(|colm| colm.collect_vec())
}

fn part_a(input: &Input) -> Option<String> {
    Some(input.iter().filter(|row| is_safe(row)).count().to_string())
}

fn is_safe(row: &[i32]) -> bool {
    let mut all_pos = true;
    let mut all_neg = true;
    for diff in row.iter().tuple_windows().map(|(a, b)| b - a) {
        if diff < 0 {
            all_pos = false;
            if !all_neg {
                return false;
            }
        } else if diff > 0 {
            all_neg = false;
            if !all_pos {
                return false;
            }
        } else {
            return false;
        }

        if diff.abs() > 3 {
            return false;
        }
    }

    return true;
}

fn part_b(input: &Input) -> Option<String> {
    Some(input.iter().filter(|row| is_safe2(row)).count().to_string())
}

fn is_safe_from(rowfrom: impl IntoIterator<Item = i32>, pos: bool, neg: bool) -> bool {
    let mut all_pos = pos;
    let mut all_neg = neg;
    for diff in rowfrom.into_iter().tuple_windows().map(|(a, b)| b - a) {
        if diff < 0 {
            all_pos = false;
            if !all_neg {
                return false;
            }
        } else if diff > 0 {
            all_neg = false;
            if !all_pos {
                return false;
            }
        } else {
            return false;
        }

        if diff.abs() > 3 {
            return false;
        }
    }

    return true;
}

fn is_safe2(row: &[i32]) -> bool {
    // all_pos or all_neg
    //   fixed by removing one i.e. if we add the next diff it is safe still

    let mut all_pos = true;
    let mut pos_removed_at = 0;

    let mut all_neg = true;
    let mut neg_removed_at = 0;

    for i in 1..row.len() {
        let diff = row[i] - row[i - 1];
        let issue = (if diff < 0 {
            if all_pos {
                pos_removed_at = i;
            }
            all_pos = false;
            !all_neg
        } else if diff > 0 {
            if all_neg {
                neg_removed_at = i;
            }
            all_neg = false;
            !all_pos
        } else {
            true
        }) | (diff.abs() > 3);

        if issue {
            let tryremovenbefore = |n: usize| {
                if i >= n {
                    let mut all_pos = all_pos;
                    if pos_removed_at >= i - n {
                        all_pos = true;
                    }

                    let mut all_neg = all_neg;
                    if neg_removed_at >= i - n {
                        all_neg = true;
                    }

                    let f = if i >= (n + 1) {
                        row.get(i - (n + 1)).map(|a| *a)
                    } else {
                        None
                    };

                    is_safe_from(
                        chain!(f, row[(i - n + 1)..row.len()].iter().map(|a| *a)),
                        all_pos,
                        all_neg,
                    )
                } else {
                    false
                }
            };
            return tryremovenbefore(0) | tryremovenbefore(1) | tryremovenbefore(2);
        }
    }

    // println!("passed completely");

    return true;
}

pub static DAY: Lazy<day::Day<Input>> = Lazy::new(|| day::Day {
    // do not touch
    parser: Box::new(parser),
    part_a: Box::new(part_a),
    part_b: Box::new(part_b),
});

#[cfg(test)]
mod tests {
    use proptest::arbitrary;
    use proptest::collection;
    use proptest::prelude::*;

    use super::*;

    #[test]
    fn goldens() {
        utils::golden("day2", &DAY, Some("2"), Some("4"), false)
    }

    #[test]
    fn finalanswer() {
        utils::finalanswer(2, &DAY, Some("402"), Some("455"), false)
    }

    prop_compose! {
        fn passinglist()(diffs in (collection::vec(1..=3i32,1..10)), increase in arbitrary::any::<bool>(), start_val in -10..10i32) -> Vec<i32> {
            diffs.into_iter().map(|a| if increase {a} else {-a})
                .scan(
                    start_val, |a,b|
                        {
                            *a += b;
                            Some(*a)
                        }).collect_vec()
        }
    }

    prop_compose! {
        fn almostpassinglist()(passinglist in passinglist())
            (index in 0..=passinglist.len(), diff in -6..6i32, mut passinglist in Just(passinglist)) -> Vec<i32> {

            let value = if index == 0 {
                passinglist[index] + diff
            } else {
                passinglist[index - 1] + diff
            };


            passinglist.insert(index, value);

            passinglist
        }
    }

    proptest! {
        #[test]
        fn test_passinglist(diffs in passinglist()) {
            prop_assert!(is_safe(&diffs));
            prop_assert!(is_safe2(&diffs));
        }
    }
    proptest! {
        #[test]
        fn test_almostpassinglist(list in almostpassinglist()) {
            prop_assert!(is_safe2(&list))
        }
    }
}
