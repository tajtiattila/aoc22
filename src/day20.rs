use anyhow::{anyhow, Result};

pub fn run(input: &str) -> Result<String> {
    let p1 = coord_sum(input)?;
    let p2 = "";
    Ok(format!("{} {}", p1, p2))
}

fn coord_sum(input: &str) -> Result<i32> {
    let mut m = Mixer::from(input)?;
    m.mix();
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
    fn from(input: &str) -> Result<Mixer> {
        let v = input
            .lines()
            .map(|s| s.parse::<i32>())
            .collect::<std::result::Result<Vec<i32>, _>>()?;
        let vl = v.len();
        Ok(Mixer {
            head: 0,
            vec: v
                .iter()
                .enumerate()
                .map(|(i, x)| Node {
                    pred: (i + vl - 1) % vl,
                    succ: (i + 1) % vl,
                    value: *x,
                })
                .collect(),
        })
    }

    fn mix(&mut self) {
        for i in 0..self.vec.len() {
            self.shift(i);
            //println!("{:2}: {:?}", i, self.to_vec());
        }
    }

    fn to_vec(&self) -> Vec<i32> {
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
            self.shift_left(i, -v);
        } else {
            self.shift_right(i, v);
        }
    }

    fn shift_left(&mut self, i: usize, n: i32) {
        let mut j = self.unlink(i).0;
        for _ in 1..n {
            j = self.vec[j].pred;
        }
        self.link(self.vec[j].pred, i, j);
    }

    fn shift_right(&mut self, i: usize, n: i32) {
        let mut j = self.unlink(i).1;
        for _ in 1..n {
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

    value: i32,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day20_works() {
        let sample = "1\n2\n-3\n3\n-2\n0\n4\n";
        assert_eq!(coord_sum(sample).ok(), Some(3));
    }
}
