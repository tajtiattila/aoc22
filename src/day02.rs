use crate::Options;
use std::cmp::Ordering;

pub fn run(input: &str, _: &Options) -> anyhow::Result<String> {
    let p1 = sim(input, |_, p| p);
    let p2 = sim(input, |o, z| (o + z + 2) % 3);
    Ok(format!("{} {}", p1, p2))
}

fn sim<F: FnMut(u8, u8) -> u8>(input: &str, mut f: F) -> usize {
    input
        .lines()
        .filter_map(play_chars)
        .map(|(o, p)| {
            let o: u8 = (o as u8) - b'A';
            let p: u8 = (p as u8) - b'X';
            let p = f(o, p);
            p as usize
                + 1
                + match cmp_play(p, o) {
                    Ordering::Greater => 6,
                    Ordering::Equal => 3,
                    Ordering::Less => 0,
                }
        })
        .sum()
}

fn play_chars(line: &str) -> Option<(char, char)> {
    let mut cs = line.chars();
    let c0 = cs.next()?;
    cs.next()?;
    let c1 = cs.next()?;

    Some((c0, c1))
}

fn cmp_play(a: u8, b: u8) -> Ordering {
    if a == b {
        Ordering::Equal
    } else if (b + 1) % 3 == a {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}
