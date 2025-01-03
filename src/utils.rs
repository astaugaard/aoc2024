#[cfg(test)]
use std::fs;

#[cfg(test)]
use once_cell::sync::Lazy;

#[cfg(test)]
use crate::day::Day;

#[cfg(test)]
pub fn golden<'a, Input>(
    file: &'a str,
    parent: &'a Lazy<Day<Input>>,
    expected_a: Option<&'a str>,
    expected_b: Option<&'a str>,
    verbose: bool,
) {
    let input = match fs::read_to_string(format!("goldens/{}", file)) {
        Ok(a) => a,
        Err(_) => panic!("golden {} failed: could not open file", file),
    };

    let parent = Lazy::force(parent);

    let input = match (*parent.parser)(input, verbose) {
        Ok(a) => a,
        Err(err) => panic!("golden {} failed to parse: {}", file, err),
    };

    if let Some(_a) = expected_a {
        let part_a = (*parent.part_a)(&input);
        if expected_a != part_a.as_deref() {
            panic!("golden {} expected {:?} got {:?}", file, expected_a, part_a);
        }
    }

    if let Some(_a) = expected_b {
        let part_b = (*parent.part_b)(&input);
        if expected_b != part_b.as_deref() {
            panic!("golden {} expected {:?} got {:?}", file, expected_b, part_b);
        }
    }
}

#[cfg(test)]
pub fn finalanswer<'a, Input>(
    daynum: usize,
    parent: &'a Lazy<Day<Input>>,
    expected_a: Option<&'a str>,
    expected_b: Option<&'a str>,
    verbose: bool,
) {
    let input = match fs::read_to_string(format!("inputs/day{}", daynum)) {
        Ok(a) => a,
        Err(_) => panic!(
            "regression test for day: {} failed: could not open file",
            daynum
        ),
    };

    let parent = Lazy::force(parent);

    let input = match (*parent.parser)(input, verbose) {
        Ok(a) => a,
        Err(err) => panic!(
            "regression test for day: {} failed to parse: {}",
            daynum, err
        ),
    };

    let part_a = (*parent.part_a)(&input);
    let part_b = (*parent.part_b)(&input);

    if let Some(_a) = expected_a {
        if expected_a != part_a.as_deref() {
            panic!(
                "regression test for day: {} expected {:?} got {:?}",
                daynum, expected_a, part_a
            );
        }
    }

    if let Some(_a) = expected_b {
        if expected_b != part_b.as_deref() {
            panic!(
                "regression test for day: {} expected {:?} got {:?}",
                daynum, expected_b, part_b
            );
        }
    }
}

#[cfg(test)]
pub fn set_function<'a, Input>(
    file: &'a str,
    parser: &dyn Fn(String, bool) -> Result<Input, String>,
    function: &dyn Fn(&Input) -> String,
    expected: &'a str,
    verbose: bool,
) {
    let input = match fs::read_to_string(format!("goldens/{}", file)) {
        Ok(a) => a,
        Err(_) => panic!("golden {} failed: could not open file", file),
    };

    let input = match (*parser)(input, verbose) {
        Ok(a) => a,
        Err(err) => panic!("golden {} failed to parse: {}", file, err),
    };

    let res = (*function)(&input);

    if expected != &res {
        panic!("golden {} expected {:?} got {:?}", file, expected, res);
    }
}
