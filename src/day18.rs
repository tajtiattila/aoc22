use anyhow::Result;
use pathfinding::prelude::bfs_reach;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

pub fn run(input: &str, _: &crate::Options) -> Result<String> {
    // 2654 too low
    let p1 = cuboids_surface(input);
    let p2 = outer_surface(input);
    Ok(format!("{} {}", p1, p2))
}

fn cuboids_surface(input: &str) -> usize {
    iter_surface(parse(input))
}

fn outer_surface(input: &str) -> usize {
    let mut reading = HashSet::new();

    let mut bb = Box::new();
    for p in parse(input) {
        reading.insert(p);
        bb.add(p);
    }

    let nbors = &neighbors();
    let hull = bfs_reach(bb.corner(), |&p| {
        nbors
            .iter()
            .map(move |&n| (p.0 + n.0, p.1 + n.1, p.2 + n.2))
            .filter(|q| bb.inside_or_boundary(*q) && !reading.contains(q))
    })
    .collect::<HashSet<_>>();

    iter_surface(bb.points().filter(move |p| !hull.contains(p)))
}

type Coord = i8;
type Vec3 = (Coord, Coord, Coord);

fn iter_surface<IT: Iterator<Item = Vec3>>(it: IT) -> usize {
    let mut m = HashMap::new();
    let nbors = &neighbors();
    for p in it {
        for n in nbors {
            let q = (2 * p.0 + n.0, 2 * p.1 + n.1, 2 * p.2 + n.2);
            flip(m.entry(q).or_insert(false));
        }
    }
    m.into_iter().filter_map(|(i, x)| x.then_some(i)).count()
}

fn parse(input: &str) -> impl Iterator<Item = Vec3> + '_ {
    input.lines().filter_map(|line| {
        let mut it = line.split(',').filter_map(|s| s.parse::<Coord>().ok());
        Some((it.next()?, it.next()?, it.next()?))
    })
}

fn neighbors() -> Vec<Vec3> {
    vec![
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ]
}

fn flip(f: &mut bool) {
    *f = !*f;
}

struct Box {
    x: (Coord, Coord),
    y: (Coord, Coord),
    z: (Coord, Coord),
}

impl Box {
    fn new() -> Box {
        Box {
            x: (0, 0),
            y: (0, 0),
            z: (0, 0),
        }
    }

    fn add(&mut self, p: Vec3) {
        Self::up_rng(&mut self.x, p.0);
        Self::up_rng(&mut self.y, p.1);
        Self::up_rng(&mut self.z, p.2);
    }

    fn corner(&self) -> Vec3 {
        (self.x.1, self.y.1, self.z.1)
    }

    // Check if p is inside the box or is on the boundary.
    // The boundary is the 1 element thick layer outside the box.
    fn inside_or_boundary(&self, p: Vec3) -> bool {
        (self.x.0 - 1..self.x.1 + 1).contains(&p.0)
            && (self.y.0 - 1..self.y.1 + 1).contains(&p.1)
            && (self.z.0 - 1..self.z.1 + 1).contains(&p.2)
    }

    fn points(&self) -> impl Iterator<Item = Vec3> + '_ {
        (self.x.0..self.x.1).flat_map(move |x| {
            (self.y.0..self.y.1).flat_map(move |y| (self.z.0..self.z.1).map(move |z| (x, y, z)))
        })
    }

    fn up_rng(r: &mut (Coord, Coord), v: Coord) {
        if r.0 == r.1 {
            *r = (v, v + 1);
        } else {
            r.0 = min(r.0, v);
            r.1 = max(r.1, v + 1);
        }
    }
}
