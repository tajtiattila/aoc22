use anyhow::{anyhow, Result};

pub fn run(input: &str) -> Result<String> {
    let p1 = coord_sum(input, 1, 1)?;
    let p2 = coord_sum(input, DECR_KEY, 10)?;
    Ok(format!("{} {}", p1, p2))
}

type Num = i64;
const DECR_KEY: Num = 811589153;

fn coord_sum(input: &str, key: Num, nmix: usize) -> Result<Num> {
    let mut m = Mixer::from(input, key)?;
    let verbose = crate::verbose();
    if verbose {
        println!("     {:?}", m.to_vec());
    }
    for i in 0..nmix {
        m.mix();
        if verbose {
            println!(" {:2}: {:?}", i + 1, m.to_vec());
        }
    }
    let v = m.to_vec();
    let iz = v
        .iter()
        .enumerate()
        .find_map(|(i, x)| (*x == 0).then_some(i))
        .ok_or_else(|| anyhow!("zero missing"))?;
    Ok([1000, 2000, 3000]
        .iter()
        .map(|n| v[(iz + n) % v.len()])
        .sum())
}

struct Mixer {
    head: usize,
    vec: Vec<Node>,
}

impl Mixer {
    fn from(input: &str, key: Num) -> Result<Mixer> {
        let v = input
            .lines()
            .map(|s| s.parse::<Num>())
            .collect::<std::result::Result<Vec<Num>, _>>()?;
        let vl = v.len();
        Ok(Mixer {
            head: 0,
            vec: v
                .iter()
                .enumerate()
                .map(|(i, x)| Node {
                    pred: (i + vl - 1) % vl,
                    succ: (i + 1) % vl,
                    value: *x * key,
                })
                .collect(),
        })
    }

    fn mix(&mut self) {
        for i in 0..self.vec.len() {
            self.shift(i);
        }
    }

    fn to_vec(&self) -> Vec<Num> {
        let mut v = Vec::new();
        v.reserve(self.vec.len());

        let mut i = self.head;
        loop {
            let n = &self.vec[i];
            v.push(n.value);
            i = n.succ;
            if i == self.head {
                return v;
            }
        }
    }

    fn shift(&mut self, i: usize) {
        let v = self.vec[i].value;
        if v == 0 {
            return;
        }
        if i == self.head {
            self.head = self.vec[self.head].succ;
        }
        if v < 0 {
            self.shift_left(i, self.nshift(-v));
        } else {
            self.shift_right(i, self.nshift(v));
        }
    }

    fn nshift(&self, n: Num) -> usize {
        // wrapping ignores the element being moved
        let wrap = self.vec.len() - 1;
        ((n as usize) + wrap - 1) % wrap
    }

    fn shift_left(&mut self, i: usize, n: usize) {
        let mut j = self.unlink(i).0;
        for _ in 0..n {
            j = self.vec[j].pred;
        }
        self.link(self.vec[j].pred, i, j);
    }

    fn shift_right(&mut self, i: usize, n: usize) {
        let mut j = self.unlink(i).1;
        for _ in 0..n {
            j = self.vec[j].succ;
        }
        self.link(j, i, self.vec[j].succ);
    }

    fn unlink(&mut self, i: usize) -> (usize, usize) {
        let ix = &self.vec[i];
        let p = ix.pred;
        let s = ix.succ;
        let mut px = &mut self.vec[p];
        px.succ = s;
        let mut sx = &mut self.vec[s];
        sx.pred = p;
        (p, s)
    }

    fn link(&mut self, p: usize, i: usize, s: usize) {
        let mut px = &mut self.vec[p];
        assert_eq!(px.succ, s);
        px.succ = i;

        let mut ix = &mut self.vec[i];
        ix.pred = p;
        ix.succ = s;

        let mut sx = &mut self.vec[s];
        assert_eq!(p, sx.pred);
        sx.pred = i;
    }
}

struct Node {
    pred: usize,
    succ: usize,

    value: Num,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day20_works() {
        let sample = "1\n2\n-3\n3\n-2\n0\n4\n";
        assert_eq!(coord_sum(sample, 1, 1).ok(), Some(3));
        assert_eq!(coord_sum(sample, DECR_KEY, 10).ok(), Some(1623178306));
    }
}
