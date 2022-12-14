use anyhow::{anyhow, Result};
use std::cmp::{max, min};

pub fn run(input: &str, _: &crate::Options) -> Result<String> {
    let m = Map::parse(input).ok_or_else(|| anyhow!("invalid input"))?;
    let p1 = count_drops(&m);
    let p2 = "";
    Ok(format!("{} {}", p1, p2))
}

fn count_drops(m: &Map) -> usize {
    let mut m = m.clone();
    let mut n = 0;
    while m.drop((500, 0)) {
        n += 1;
    }
    n
}

#[derive(Debug, Clone)]
struct Map {
    bounds: Rect,
    stride: i32,
    v: Vec<u8>,
}

impl Map {
    fn parse(input: &str) -> Option<Self> {
        let mut bounds = Self::src_dim(input)?;
        bounds.min.1 = 0;
        let dx = bounds.max.0 - bounds.min.0 + 1;
        let dy = bounds.max.1 - bounds.min.1 + 1;
        let v = vec![b'.'; (dx * dy) as usize];
        let mut r = Self {
            bounds,
            stride: dx,
            v,
        };
        segments(input).for_each(|seg| r.add_seg(&seg));
        Some(r)
    }

    // drop sand, returns true if it stays in the area
    fn drop(&mut self, p: (i32, i32)) -> bool {
        let mut p = p;

        if self.at((p.0, p.1)) != b'.' {
            return false; // no more space
        }

        loop {
            if p.1 > self.bounds.max.1 {
                return false; // fallen outside
            }

            let y = p.1 + 1;
            if self.at((p.0, y)) == b'.' {
                p.1 = y;
            } else if self.at((p.0 - 1, y)) == b'.' {
                p = (p.0 - 1, y);
            } else if self.at((p.0 + 1, y)) == b'.' {
                p = (p.0 + 1, y);
            } else {
                if let Some(i) = self.pos(p) {
                    self.v[i] = b'o';
                }
                return true;
            }
        }
    }

    fn at(&self, p: (i32, i32)) -> u8 {
        self.pos(p).map_or(b'.', |i| self.v[i])
    }

    fn show(&self) {
        println!("{}", self.to_str());
    }

    fn to_str(&self) -> String {
        let mut acc = String::new();
        self.v.chunks(self.stride as usize).for_each(|row| {
            acc.push_str(&format!("{}\n", String::from_utf8_lossy(row)));
        });
        acc
    }

    fn add_seg(&mut self, seg: &Segment) {
        let a = seg.a;
        let b = seg.b;
        if a.1 == b.1 {
            let lo = self.pos((min(a.0, b.0), a.1)).unwrap();
            let hi = self.pos((max(a.0, b.0), a.1)).unwrap();
            for p in lo..=hi {
                self.v[p] = b'#';
            }
        } else if a.0 == b.0 {
            let lo = self.pos((a.0, min(a.1, b.1))).unwrap();
            let hi = self.pos((a.0, max(a.1, b.1))).unwrap();
            for p in (lo..=hi).step_by(self.stride as usize) {
                self.v[p] = b'#';
            }
        } else {
            panic!("invalid segment")
        }
    }

    fn pos(&self, p: (i32, i32)) -> Option<usize> {
        let b = &self.bounds;
        b.contains(p)
            .then_some((p.0 - b.min.0 + (p.1 - b.min.1) * self.stride) as usize)
    }

    fn src_dim(input: &str) -> Option<Rect> {
        let mut dim: Option<Rect> = None;
        for seg in segments(input) {
            Self::extend(&mut dim, seg.a);
            Self::extend(&mut dim, seg.b);
        }
        dim
    }

    fn extend(ob: &mut Option<Rect>, p: (i32, i32)) {
        *ob = Some(match ob {
            Some(r) => Rect {
                min: (min(r.min.0, p.0), min(r.min.1, p.1)),
                max: (max(r.max.0, p.0), max(r.max.1, p.1)),
            },
            None => Rect {
                min: (p.0, p.1),
                max: (p.0, p.1),
            },
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct Rect {
    min: (i32, i32),
    max: (i32, i32),
}

impl Rect {
    fn contains(&self, p: (i32, i32)) -> bool {
        (self.min.0..=self.max.0).contains(&p.0) && (self.min.1..=self.max.1).contains(&p.1)
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
        let m = Map::parse(sample).unwrap();
        m.show();
        assert_eq!(count_drops(&m), 24);
    }
}
