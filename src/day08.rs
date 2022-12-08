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
    let t = v[y][x];
    rays(v, x, y).iter().any(|v| v.iter().all(|h| *h < t))
}

fn scenic(v: &[Vec<u8>], x: usize, y: usize) -> usize {
    let t = v[y][x];
    rays(v, x, y)
        .iter()
        .map(|v| {
            let mut n = 0;
            for h in v {
                n += 1;
                if *h >= t {
                    break;
                }
            }
            n
        })
        .product()
}

fn rays(v: &[Vec<u8>], x: usize, y: usize) -> Vec<Vec<u8>> {
    let row = &v[y];
    vec![
        row[x + 1..].to_vec(),
        row[..x].iter().rev().copied().collect(),
        v[y + 1..].iter().map(|r| r[x]).collect(),
        v[..y].iter().rev().map(|r| r[x]).collect(),
    ]
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
