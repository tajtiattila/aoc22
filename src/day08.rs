use crate::{day_ok, DayResult, Options};

pub fn run(input: &str, _: &Options) -> DayResult {
    let (p1, p2) = trees(input);
    day_ok(p1, p2)
}

fn trees(input: &str) -> (usize, usize) {
    let v = input
        .lines()
        .map(|x| Vec::from(x.as_bytes()))
        .collect::<Vec<_>>();

    let mut nvis = 0;
    let mut smax = 0;
    for y in 0..v.len() {
        for x in 0..v[y].len() {
            if vis(&v, x, y) {
                nvis += 1;
            }

            smax = std::cmp::max(smax, scenic(&v, x, y));
        }
    }
    (nvis, smax)
}

fn vis(v: &[Vec<u8>], x: usize, y: usize) -> bool {
    let row = &v[y];
    let t = row[x];
    if x == 0 || y == 0 || x == row.len() - 1 || y == v.len() - 1 {
        return true;
    }
    return row[x + 1..].iter().all(|z| *z < t)
        || row[..x].iter().all(|z| *z < t)
        || v[y + 1..].iter().all(|r| r[x] < t)
        || v[..y].iter().all(|r| r[x] < t);
}

fn scenic(v: &[Vec<u8>], x: usize, y: usize) -> usize {
    let row = &v[y];
    let t = row[x];
    if x == 0 || y == 0 || x == row.len() - 1 || y == v.len() - 1 {
        return 0;
    }
    let s0 = svis(t, row[x + 1..].iter().copied());
    let s1 = svis(t, row[..x].iter().rev().copied());
    let s2 = svis(t, v[y + 1..].iter().map(|r| r[x]));
    let s3 = svis(t, v[..y].iter().rev().map(|r| r[x]));
    s0 * s1 * s2 * s3
}

fn svis<It>(t: u8, it: It) -> usize
where
    It: Iterator<Item = u8>,
{
    it.scan(true, |go, h| {
        if *go {
            *go = h < t;
            Some(())
        } else {
            None
        }
    })
    .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day08_works() {
        let sample = "\
30373
25512
65332
33549
35390
";
        assert_eq!(trees(sample), (21, 8));
    }
}
