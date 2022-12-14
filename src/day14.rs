use anyhow::Result;

pub fn run(input: &str, _: &crate::Options) -> Result<String> {
    let (p1, _) = sim_drops(input, false)?;
    let (p2, _) = sim_drops(input, true)?;
    Ok(format!("{} {}", p1, p2))
}

type Map = crate::quadmap::Map<u8>;

fn sim_drops(input: &str, floor: bool) -> Result<(usize, Map)> {
    let mut m = parse(input);
    let floor = floor.then_some(m.bounds().max.1 + 1);
    let mut n = 0;
    while drop(&mut m, (SX, 0), floor) {
        n += 1;
    }
    Ok((n, m))
}

// drop sand, returns true if it stays in the area
fn drop(m: &mut Map, p: (i32, i32), floor: Option<i32>) -> bool {
    let mut p = p;

    if m.at((p.0, p.1)) != &EMPTY {
        return false; // no more space
    }

    let ystop = match floor {
        Some(y) => y - 1,
        None => m.bounds().max.1,
    };

    loop {
        if p.1 == ystop {
            if floor.is_some() {
                *m.at_mut(p) = SAND;
                return true;
            } else {
                return false; // fallen outside
            }
        }

        let y = p.1 + 1;
        if m.at((p.0, y)) == &EMPTY {
            p.1 = y;
        } else if m.at((p.0 - 1, y)) == &EMPTY {
            p = (p.0 - 1, y);
        } else if m.at((p.0 + 1, y)) == &EMPTY {
            p = (p.0 + 1, y);
        } else {
            *m.at_mut(p) = SAND;
            return true;
        }
    }
}

const SX: i32 = 500;
const EMPTY: u8 = 0;
const WALL: u8 = 1;
const SAND: u8 = 2;

fn parse(input: &str) -> Map {
    let mut m = Map::new(EMPTY);
    segments(input).for_each(|seg| add_segment(&mut m, &seg));
    m
}

fn add_segment(map: &mut Map, seg: &Segment) {
    let a = seg.a;
    let b = seg.b;
    if a.1 == b.1 {
        map.hline(a.0, b.0, a.1, &WALL);
    } else if a.0 == b.0 {
        map.vline(a.0, a.1, b.1, &WALL);
    } else {
        panic!("invalid segment")
    }
}

fn segments(input: &str) -> impl Iterator<Item = Segment> + '_ {
    input.lines().flat_map(segment)
}

fn segment(line: &str) -> impl Iterator<Item = Segment> + '_ {
    let mut acc = None;
    line.split(" -> ")
        .filter_map(|s| {
            let (l, r) = s.split_once(',')?;
            Some((l.parse().ok()?, r.parse().ok()?))
        })
        .filter_map(move |q| {
            let p = acc;
            acc = Some(q);
            let p = p?;
            Some(Segment { a: p, b: q })
        })
}

#[derive(Debug, Copy, Clone)]
struct Segment {
    a: (i32, i32),
    b: (i32, i32),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day14_works() {
        let sample = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";
        let t = |floor| {
            if let Ok(r) = sim_drops(sample, floor) {
                show(&r.1);
                r.0
            } else {
                0
            }
        };

        assert_eq!(t(false), 24);
        assert_eq!(t(true), 93);
    }

    fn show(_map: &Map) {
        /*
        map.v.chunks(map.stride as usize).for_each(|row| {
            println!("{}", String::from_utf8_lossy(row));
        });
        */
    }
}
