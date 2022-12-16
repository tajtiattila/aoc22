use anyhow::{anyhow, bail, Result};
use pathfinding::prelude::bfs_reach;
use std::collections::HashMap;

pub fn run(input: &str, o: &crate::Options) -> Result<String> {
    let rdg = parse(input)?;
    let working = working_valves(&rdg);
    if o.verbose {
        for v in &working {
            print!(" {} rate={:2}   ", v.label, v.rate);
            for (i, (j, d)) in v.next.iter().enumerate() {
                print!(
                    "{}{}:{}",
                    (i != 0).then_some(", ").unwrap_or_default(),
                    working[*j].label,
                    d
                );
            }
            println!();
        }
    }
    let p1 = pressure_release_1(&working, o.verbose)?;
    let p2 = pressure_release_2(&working, o.verbose)?;
    Ok(format!("{} {}", p1, p2))
}

fn pressure_release_1(wv: &[WorkValve], _verbose: bool) -> Result<i32> {
    const TIME: i32 = 30;
    pressure_release_impl(wv, TIME, 0)
        .map(|n| n.released)
        .max()
        .ok_or_else(|| anyhow!("pressure release failed"))
}

fn pressure_release_2(wv: &[WorkValve], verbose: bool) -> Result<i32> {
    const TIME: i32 = 26;
    let max_rate: i32 = wv.iter().map(|v| v.rate).sum();
    if verbose {
        println!("max. rate: {}", max_rate);
    }
    let mut fst = pressure_release_impl(wv, TIME, 0)
        .map(|n| (n.open, n.released))
        .collect::<Vec<_>>();
    fst.sort_by_key(|x| std::cmp::Reverse(x.1));
    if verbose {
        println!("result size: {}", fst.len());
    }

    let mut bestr = 0;
    for (i, x) in fst.iter().enumerate() {
        let (xo, xr) = *x;
        if xr < bestr / 2 {
            break;
        }

        for y in fst.iter().skip(i + 1) {
            let (yo, yr) = *y;
            let r = xr + yr;
            if r <= bestr {
                break;
            }

            if (xo & yo) == 0 {
                bestr = r;
            }
        }
    }
    if bestr == 0 {
        bail!("pressure release failed");
    }

    Ok(bestr)
}

fn pressure_release_impl(
    wv: &[WorkValve],
    time: i32,
    start_open: u32,
) -> impl Iterator<Item = Node> + '_ {
    bfs_reach(Node::new(time, start_open), move |&from| {
        let node = &wv[from.index];
        node.next
            .iter()
            .filter_map(move |&(i, dist)| {
                (!from.is_open(i)).then_some(from.goto(i, dist, wv[i].rate))
            })
            .filter(|next| next.ttg > 0)
    })
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Node {
    index: usize,
    ttg: i32,  // minutes remaining
    open: u32, // valves opened so far

    rate: i32,     // current release rate
    released: i32, // pressure released so far
}

impl Node {
    fn new(ttg: i32, open: u32) -> Node {
        Node {
            index: 0,
            ttg,
            open,
            rate: 0,
            released: 0,
        }
    }

    fn goto(&self, index: usize, dist: i32, rate: i32) -> Node {
        let time = dist + 1; // time to move and open valve
        Node {
            index,
            ttg: self.ttg - time,
            open: self.open | (1 << index),
            rate: self.rate + rate,
            released: self.released + (self.ttg - time) * rate,
        }
    }

    fn is_open(&self, index: usize) -> bool {
        (self.open & (1 << index)) != 0
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Elef {
    a: Agent,
    b: Agent,

    time: i32, // minutes elapsed
    open: u64, // valves opened so far

    rate: i32,     // current release rate
    released: i32, // pressure released so far
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Agent {
    loc: usize,
    ttg: i32, // time to go
}

// a working valve
struct WorkValve {
    label: String,
    rate: i32,
    next: Vec<(usize, i32)>, // index and dist. of rooms with wotking valves
}

fn working_valves(rdg: &[Valve]) -> Vec<WorkValve> {
    let nsteps = &map_nsteps(rdg);

    let mut vidx = Vec::new();
    vidx.resize(rdg.len(), usize::max_value());
    rdg.iter()
        .enumerate()
        .filter_map(|(i, v)| (i == 0 || v.rate != 0).then_some(i))
        .enumerate()
        .for_each(|(new_idx, i)| vidx[i] = new_idx);

    rdg.iter()
        .enumerate()
        .filter(|(i, v)| i == &0 || v.rate != 0)
        .map(|(i, v)| {
            let mut next: Vec<_> = nsteps[i]
                .iter()
                .enumerate()
                .filter_map(|(old_idx, dist)| {
                    (i != old_idx && rdg[old_idx].rate != 0).then_some((vidx[old_idx], *dist))
                })
                .collect();
            next.sort_by_key(|(_, d)| *d);
            WorkValve {
                label: v.label.clone(),
                rate: v.rate,
                next,
            }
        })
        .collect()
}

fn map_nsteps(rdg: &[Valve]) -> Vec<Vec<i32>> {
    (0..rdg.len()).map(|i| calc_nsteps(rdg, i)).collect()
}

fn calc_nsteps(rdg: &[Valve], i: usize) -> Vec<i32> {
    let mut v = Vec::new();
    v.resize(rdg.len(), i32::max_value());
    v[i] = 0;
    let mut acc = vec![i];
    let mut dist = 1;
    while !acc.is_empty() {
        let w = acc;
        acc = Vec::new();
        for i in w {
            for &j in &rdg[i].next {
                if v[j] == i32::max_value() {
                    v[j] = dist;
                    acc.push(j);
                }
            }
        }
        dist += 1;
    }
    v
}

fn parse(input: &str) -> Result<Vec<Valve>> {
    let mut m = HashMap::new();
    m.insert("AA", 0);
    for (i, s) in input.lines().enumerate() {
        let (label, _, _) =
            parse_valve(s).ok_or_else(|| anyhow!("parse error on line {}", i + 1))?;
        let index = m.len();
        m.entry(label).or_insert(index);
    }

    let mut v: Vec<(usize, Valve)> = input
        .lines()
        .map(|s| {
            let (label, rate, next) = parse_valve(s).unwrap();

            let next: Vec<usize> = next
                .map(|s| {
                    m.get(s)
                        .copied()
                        .ok_or_else(|| anyhow!("invalid valve {}", s))
                })
                .collect::<Result<Vec<usize>>>()?;
            Ok((
                *m.get(label).unwrap(),
                Valve {
                    label: String::from(label),
                    rate,
                    next,
                },
            ))
        })
        .collect::<Result<Vec<(usize, Valve)>>>()?;
    v.sort_by_key(|x| x.0);

    Ok(v.into_iter().map(|x| x.1).collect())
}

struct Valve {
    label: String,
    rate: i32,
    next: Vec<usize>, // index of adjacent rooms
}

fn parse_valve(s: &str) -> Option<(&str, i32, impl Iterator<Item = &str>)> {
    let (ls, rs) = s.split_once("; ")?;
    let mut it = ls.split(' ').enumerate().filter_map(|(i, z)| {
        if i == 1 {
            Some(z)
        } else {
            z.strip_prefix("rate=")
        }
    });
    let rs = rs
        .strip_prefix("tunnels lead to valves ")
        .or_else(|| rs.strip_prefix("tunnel leads to valve "))?;

    Some((it.next()?, it.next()?.parse().ok()?, rs.split(", ")))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day16_works() {
        let sample = "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

        let rdg = parse(sample).unwrap();

        for (i, v) in rdg.iter().enumerate() {
            println!(" {:2}:  {} {:2}  {:?}", i, v.label, v.rate, v.next);
        }

        for r in map_nsteps(&rdg) {
            for c in r {
                print!(" {:3}", c)
            }
            println!();
        }

        let working = working_valves(&rdg);

        assert_eq!(pressure_release_1(&working, true).ok(), Some(1651));
        assert_eq!(pressure_release_2(&working, true).ok(), Some(1707));
    }
}
