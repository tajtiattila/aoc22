use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::collections::HashMap;

pub fn run(input: &str, o: &crate::Options) -> Result<String> {
    let rdg = parse(input);
    if o.verbose {
        for s in &rdg {
            println!("({},{}) range {}", s.p.0, s.p.1, s.range);
        }
    }
    let p1 = count_no_beacon(&rdg, 2000000);
    let p2 = scan_beacon(&rdg, 4000000).ok_or_else(|| anyhow!("not found"))?;
    Ok(format!("{} {}", p1, p2))
}

fn count_no_beacon(rdg: &[Sensor], yline: i32) -> usize {
    rdg_spans(rdg, yline)
        .map(|(lo, hi)| (hi - 1 - lo) as usize)
        .sum()
}

fn scan_beacon(rdg: &[Sensor], max: i32) -> Option<usize> {
    (0..=max).find_map(|y| scan_line(rdg, y, max))
}

fn scan_line(rdg: &[Sensor], y: i32, max: i32) -> Option<usize> {
    rdg_spans(rdg, y)
        .map(|s| s.1)
        .find(|x| (0..=max).contains(x))
        .map(|x| (x as usize) * 4000000 + (y as usize))
}

fn rdg_spans(rdg: &[Sensor], yline: i32) -> impl Iterator<Item = (i32, i32)> {
    let mut v = Vec::new();
    for s in rdg {
        let width = s.range - (s.p.1 - yline).abs();
        if width > 0 {
            v.push((s.p.0 - width, s.p.0 + width));
        }
    }

    spans_merged(&v)
}

fn spans_merged(spans: &[(i32, i32)]) -> impl Iterator<Item = (i32, i32)> {
    let mut m = HashMap::new();
    for (lo, hi) in spans {
        m.entry(*lo).and_modify(|v| *v += 1).or_insert(1);
        m.entry(*hi + 1).and_modify(|v| *v -= 1).or_insert(-1);
    }
    let mut lvl = 0;
    let mut acc = 0;
    m.into_iter()
        .sorted()
        .filter_map(move |(x, l)| {
            let nlvl = lvl + l;
            let end = (lvl == 0) != (nlvl == 0);
            lvl = nlvl;
            end.then_some(x)
        })
        .enumerate()
        .filter_map(move |(i, x)| {
            if i % 2 == 0 {
                acc = x;
                None
            } else {
                Some((acc, x))
            }
        })
}

//Sensor at x=1054910, y=811769: closest beacon is at x=2348729, y=1239977
fn parse(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .filter_map(parse_line)
        .map(|(s, b)| Sensor {
            p: s,
            range: manhattan(s, b),
        })
        .collect()
}

fn parse_line(line: &str) -> Option<((i32, i32), (i32, i32))> {
    let mut it = line
        .split(' ')
        .filter_map(|s| s.strip_prefix("x=").or_else(|| s.strip_prefix("y=")))
        .map(|s| s.trim_end_matches(|c| c == ',' || c == ':'))
        .filter_map(|s| s.parse::<i32>().ok());
    Some(((it.next()?, it.next()?), (it.next()?, it.next()?)))
}

#[derive(Debug)]
struct Sensor {
    p: (i32, i32),
    range: i32,
}

fn manhattan(p: (i32, i32), q: (i32, i32)) -> i32 {
    (p.0 - q.0).abs() + (p.1 - q.1).abs()
}

#[cfg(test)]
mod test {
    use super::*;
    /*
                     1    1    2    2
           0    5    0    5    0    5
     9 ...#########################...
    10 ..####B######################..
    11 .###S#############.###########.

       ...aaaaaaaaaaaaaaaaaaaaaaaaaaa.
    */
    #[test]
    fn day15_works() {
        let sample = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";
        let rdg = parse(sample);
        assert_eq!(count_no_beacon(&rdg, 10), 26);
        assert_eq!(scan_beacon(&rdg, 20), Some(56000011));
    }
}
