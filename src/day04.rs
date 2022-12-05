use crate::{day_ok, DayResult, Options};

pub fn run(input: &str, _: &Options) -> DayResult {
    let p1 = count_rp(input, |a, b, c, d| (a <= c && d <= b));
    let p2 = count_rp(input, |a, b, c, d| {
        (a..=b).contains(&c) || (a..=b).contains(&d)
    });
    day_ok(p1, p2)
}

fn count_rp<F>(input: &str, mut f: F) -> usize
where
    F: FnMut(u32, u32, u32, u32) -> bool,
{
    input
        .lines()
        .filter_map(parse_rng_pair)
        .filter(|(a, b, c, d)| f(*a, *b, *c, *d) || f(*c, *d, *a, *b))
        .count()
}

fn parse_rng_pair(line: &str) -> Option<(u32, u32, u32, u32)> {
    let mut it = line
        .split(|c| "-,".contains(c))
        .filter_map(|s| s.parse::<u32>().ok());

    Some((it.next()?, it.next()?, it.next()?, it.next()?))
}
