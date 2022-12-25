use anyhow::Result;
use std::collections::{HashMap, HashSet};

pub fn run(input: &str) -> Result<String> {
    let p1 = sim_step_area(input, 10);
    let p2 = find_stop_round(input);
    Ok(format!("{} {}", p1, p2))
}

const SHOW_STEPS: bool = false;
fn sim_step_area(input: &str, n: usize) -> usize {
    let mut s = Sim::from(input);
    for _ in 0..n {
        s.step();
        if crate::verbose() {
            println!("{}", s.to_string_lines());
        }
    }
    s.count_free()
}

fn find_stop_round(input: &str) -> usize {
    let mut s = Sim::from(input);
    while s.step() {}
    s.n
}

struct Sim {
    m: HashSet<Vec2>,
    n: usize,
}

impl Sim {
    fn from(input: &str) -> Sim {
        Sim {
            m: poss(input).collect(),
            n: 0,
        }
    }

    fn step(&mut self) -> bool {
        let mut goal = HashMap::new();
        for (_, p) in self.nexts() {
            if let Some(x) = p {
                goal.entry(x).and_modify(|n| *n += 1).or_insert(1);
            }
        }

        let mut moved = false;
        let m2 = self
            .nexts()
            .map(|(e, p)| {
                if let Some(x) = p {
                    if goal.get(&x) == Some(&1) {
                        moved = true;
                        return x;
                    }
                }
                e
            })
            .collect();

        self.m = m2;
        self.n += 1;

        moved
    }

    fn count_free(&self) -> usize {
        let n = self.m.len();
        if n == 0 {
            return 0;
        }
        let x0 = self.m.iter().map(|p| p.0).min().unwrap();
        let x1 = self.m.iter().map(|p| p.0).max().unwrap();
        let y0 = self.m.iter().map(|p| p.1).min().unwrap();
        let y1 = self.m.iter().map(|p| p.1).max().unwrap();
        let a = ((x1 + 1 - x0) as usize) * ((y1 + 1 - y0) as usize);
        a - n
    }

    fn nexts(&self) -> impl Iterator<Item = (Vec2, Option<Vec2>)> + '_ {
        let l = PROPOSALS.len();
        self.m.iter().map(move |&e| {
            if self.stay(e) {
                return (e, None);
            }
            let p = (0..l)
                .map(|i| PROPOSALS[(i + self.n) % l])
                .find_map(|p| self.proposal(e, &p));
            (e, p)
        })
    }

    fn stay(&self, e: Vec2) -> bool {
        NBORS
            .iter()
            .all(|d| !self.m.contains(&(e.0 + d.0, e.1 + d.1)))
    }

    fn proposal(&self, e: Vec2, p: &Proposal) -> Option<Vec2> {
        (!p.check
            .iter()
            .any(|x| self.m.contains(&(e.0 + x.0, e.1 + x.1))))
        .then_some((e.0 + p.step.0, e.1 + p.step.1))
    }

    fn to_string_lines(&self) -> String {
        if self.m.is_empty() {
            return String::new();
        }
        let x0 = self.m.iter().map(|p| p.0).min().unwrap();
        let x1 = self.m.iter().map(|p| p.0).max().unwrap();
        let y0 = self.m.iter().map(|p| p.1).min().unwrap();
        let y1 = self.m.iter().map(|p| p.1).max().unwrap();

        let mut s = String::new();
        for y in y0..=y1 {
            for x in x0..=x1 {
                s.push(if self.m.contains(&(x, y)) { '#' } else { '.' });
            }
            s.push('\n');
        }
        s
    }
}

fn poss(input: &str) -> impl Iterator<Item = Vec2> + '_ {
    input.lines().enumerate().flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .filter_map(move |(x, ch)| (ch == '#').then_some((x as i32, y as i32)))
    })
}

type Vec2 = (i32, i32);

const NW: Vec2 = (-1, -1);
const N: Vec2 = (0, -1);
const NE: Vec2 = (1, -1);
const W: Vec2 = (-1, 0);
const E: Vec2 = (1, 0);
const SW: Vec2 = (-1, 1);
const S: Vec2 = (0, 1);
const SE: Vec2 = (1, 1);

static NBORS: &[Vec2] = &[NW, N, NE, W, E, SW, S, SE];

static PROPOSALS: &[Proposal] = &[
    Proposal {
        check: &[N, NW, NE],
        step: N,
    },
    Proposal {
        check: &[S, SE, SW],
        step: S,
    },
    Proposal {
        check: &[W, SW, NW],
        step: W,
    },
    Proposal {
        check: &[E, SE, NE],
        step: E,
    },
];

#[derive(Debug, Copy, Clone)]
struct Proposal<'a> {
    check: &'a [Vec2],
    step: Vec2,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day23_works() {
        let sample = "\
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";
        assert_eq!(sim_step_area(sample, 10), 110);
        assert_eq!(find_stop_round(sample), 20);
    }
}
