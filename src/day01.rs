use crate::{day_ok, DayResult, Options};
use std::cmp::Reverse;

pub fn run(input: &str, _: &Options) -> DayResult {
    let mut cals = calories(input);
    cals.sort_by_key(|&x| Reverse(x));

    let pr1 = cals[0];
    let pr2 = cals.iter().take(3).sum::<usize>();

    day_ok(pr1, pr2)
}

fn calories(input: &str) -> Vec<usize> {
    let mut v = Vec::new();
    let mut acc = 0;
    for x in input.lines().map(|s| s.parse::<usize>()) {
        match x {
            Ok(v) => {
                acc += v;
            }
            Err(_) => {
                v.push(acc);
                acc = 0;
            }
        }
    }
    v.push(acc);
    v
}
