use crate::{day_ok, DayResult, Options};
use itertools::Itertools;
use std::collections::HashSet;

pub fn run(input: &str, _: &Options) -> DayResult {
    day_ok(p1(input), p2(input))
}

fn p1(input: &str) -> u32 {
    let mut scratch = Vec::new();
    input
        .lines()
        .map(|line| in_both_halves(line, &mut scratch))
        .sum()
}

fn p2(input: &str) -> u32 {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .filter_map(|chunks| {
            chunks
                .map(|line| line.chars().collect::<HashSet<char>>())
                .reduce(|acc, item| acc.intersection(&item).copied().collect())
        })
        .map(|chars| chars.into_iter().map(prisum).sum::<u32>())
        .sum()
}

fn in_both_halves(pack: &str, scratch: &mut Vec<u8>) -> u32 {
    let pack = pack.as_bytes();
    let p = pack.len() / 2;

    scratch.clear();
    scratch.extend(&pack[..p]);
    scratch.sort();

    pack[p..]
        .iter()
        .filter_map(|c| {
            if scratch.binary_search(c).is_ok() {
                Some(*c as u8)
            } else {
                None
            }
        })
        .map(|c| prisum(c as char))
        .unique()
        .sum::<u32>()
}

fn prisum(c: char) -> u32 {
    if ('a'..='z').contains(&c) {
        c as u32 - 'a' as u32 + 1
    } else if ('A'..='Z').contains(&c) {
        c as u32 - 'A' as u32 + 27
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d03_p1_works() {
        let sample = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
        assert_eq!(p1(sample), 157);
        assert_eq!(p2(sample), 70);
    }
}
