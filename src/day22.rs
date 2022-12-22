use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet, VecDeque};

pub fn run(input: &str) -> Result<String> {
    let (m, instr) = parse(input).ok_or_else(|| anyhow!("parse error"))?;
    let p1 = walk(&m, &instr);
    let p2 = cube_walk(&m, 50, &instr)?;
    Ok(format!("{} {}", p1, p2))
}

fn walk(m: &Map, instr: &[(i32, i32)]) -> i32 {
    let mut p = m.start().unwrap();
    let mut h = 0;
    for (b, n) in instr {
        h = (h + b).rem_euclid(4);
        p = m.walk(p, h, *n);
    }
    pos_hdg_value(p, h)
}

fn cube_walk(m: &Map, dim: Coord, instr: &[(i32, i32)]) -> Result<i32> {
    let mut c = Cube::from(m, dim).ok_or_else(|| anyhow!("cube fold failed"))?;

    let mut p = c.start();
    for &(b, n) in instr {
        p = c.walk(p, b, n);
    }

    if crate::verbose() {
        println!("{}", c.folded.to_str_lines());
    }

    let (p, h) = c
        .folded(p)
        .ok_or_else(|| anyhow!("pos unfold failed for {:?}", p))?;

    Ok(pos_hdg_value(p, h))
}

fn pos_hdg_value(p: Vec2, h: i32) -> i32 {
    let row = (p.1 + 1) as i32;
    let col = (p.0 + 1) as i32;
    1000 * row + 4 * col + h
}

struct Cube {
    dim: Coord,

    // Facelets are 2x2 units in size.
    // A valid position have exactly one coordinate z = |dim|,
    // with other coordinates being odd numbers x,y < |dim|.
    m: HashMap<Vec3, (Vec2, char)>,

    folded: Map,
}

impl Cube {
    fn from(folded: &Map, dim: Coord) -> Option<Cube> {
        let faces = Self::faces(folded, dim)?;
        let m = Self::places(&faces, dim)
            .map(|(mp, fp)| (mp, (fp, folded.at(fp))))
            .collect();
        Some(Cube {
            dim,
            m,
            folded: folded.clone(),
        })
    }

    fn start(&self) -> CubePos {
        let d = self.dim;
        CubePos {
            pos: Vec3(-d + 1, d - 1, -d),
            hdg: Vec3(1, 0, 0),
            nrm: Vec3(0, 0, -1),
        }
    }
    fn walk(&mut self, start: CubePos, turn: i32, nstep: i32) -> CubePos {
        let mut p = start.turn(turn);
        for _ in 0..nstep {
            let q = self.step(p);
            if self.m.get(&q.pos).unwrap().1 == '#' {
                return p; // bumped into a wall
            }
            p = q;

            let (pf, dir) = self.folded(p).unwrap();
            self.folded.set_at(pf, ARROWS[dir as usize]);
        }
        p
    }

    fn folded(&self, pos: CubePos) -> Option<(Vec2, i32)> {
        let p = self
            .step(CubePos {
                hdg: -pos.hdg,
                ..pos
            })
            .pos;
        let q = pos.pos;
        let r = self.step(pos).pos;

        let pf = self.m.get(&p)?.0;
        let qf = self.m.get(&q)?.0;
        let rf = self.m.get(&r)?.0;
        for (i, d) in DIRS.iter().enumerate() {
            if (qf.0 + d.0, qf.1 + d.1) == rf || (qf.0 - d.0, qf.1 - d.1) == pf {
                return Some((qf, i as i32));
            }
        }

        None
    }

    fn places(faces: &[Face], dim: Coord) -> impl Iterator<Item = (Vec3, Vec2)> + '_ {
        faces.iter().flat_map(move |face| face.places(dim))
    }

    fn step(&self, start: CubePos) -> CubePos {
        let pos = start.pos + 2 * start.hdg;
        if self.inside(pos) {
            return CubePos { pos, ..start };
        }
        CubePos {
            pos: start.pos + start.hdg - start.nrm,
            hdg: -start.nrm,
            nrm: start.hdg,
        }
    }

    fn inside(&self, p: Vec3) -> bool {
        let d = self.dim;
        (-d..=d).contains(&p.0) && (-d..=d).contains(&p.1) && (-d..=d).contains(&p.2)
    }

    // Detect cube fold based on folded map, and create faces.
    fn faces(folded: &Map, dim: Coord) -> Option<Vec<Face>> {
        let p = folded.start()?;
        let mut seen = HashSet::new();
        let mut faces = Vec::new();
        let mut w = VecDeque::from(vec![Face {
            p,
            m: (Vec3(1, 0, 0), Vec3(0, -1, 0), Vec3(0, 0, -1)),
        }]);
        while let Some(f) = w.pop_front() {
            faces.push(f);
            seen.insert(f.p);

            let shift = |dx, dy| (f.p.0 + dx, f.p.1 + dy);
            let mut try_add = |p, m| {
                if !seen.contains(&p) && folded.at(p) != ' ' {
                    w.push_back(Face { p, m });
                }
            };
            let (x, y, z) = f.m;
            try_add(shift(-dim, 0), (z, y, -x));
            try_add(shift(dim, 0), (-z, y, x));
            try_add(shift(0, dim), (x, -z, y));
        }
        Some(faces)
    }
}

#[derive(Debug, Copy, Clone)]
struct CubePos {
    pos: Vec3,
    hdg: Vec3,
    nrm: Vec3,
}

impl CubePos {
    fn turn(self, dir: i32) -> Self {
        if dir == 0 {
            self
        } else {
            let hdg = if dir > 0 {
                -cross(self.hdg, self.nrm)
            } else {
                cross(self.hdg, self.nrm)
            };
            Self { hdg, ..self }
        }
    }
}

fn cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3(
        a.1 * b.2 - a.2 * b.1,
        a.2 * b.0 - a.0 * b.2,
        a.0 * b.1 - a.1 * b.0,
    )
}

// Face describes a side of the cube.
#[derive(Debug, Copy, Clone)]
struct Face {
    // face "top left" position on folded source map
    p: Vec2,

    // matrix specifying space
    //  x: right in folded map
    //  y: down in folded map
    //  z: face normal vector (pointing outside)
    m: Mat3,
}

impl Face {
    fn places(&self, dim: Coord) -> impl Iterator<Item = (Vec3, Vec2)> {
        let (xf0, yf0) = self.p;
        let (x, y, z) = self.m;
        let mut py = dim * (z - (x + y)) + x + y;
        (yf0..yf0 + dim).flat_map(move |yf| {
            let mut px = py;
            let r = (xf0..xf0 + dim).map(move |xf| {
                let r = (px, (xf, yf));
                px += 2 * x;
                r
            });
            py += 2 * y;
            r
        })
    }
}

type Coord = i16;
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Vec3(Coord, Coord, Coord);

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl std::ops::Mul<Coord> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Coord) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl std::ops::Mul<Vec3> for Coord {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

type Mat3 = (Vec3, Vec3, Vec3);

fn parse(input: &str) -> Option<(Map, Vec<(i32, i32)>)> {
    let (ms, mut ins) = input.split_once("\n\n")?;

    let mut v = Vec::new();
    let (n, ns) = parse_front_num(ins)?;
    v.push((0, n));
    ins = ns.trim();

    while !ins.is_empty() {
        let d = match ins.as_bytes()[0] {
            b'L' => -1,
            b'R' => 1,
            _ => return None,
        };
        let (n, ns) = parse_front_num(&ins[1..])?;
        v.push((d, n));
        ins = ns;
    }

    Some((Map::parse(ms), v))
}

fn parse_front_num(s: &str) -> Option<(i32, &str)> {
    let i = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    Some((s[..i].parse().ok()?, &s[i..]))
}

type Vec2 = (Coord, Coord);

#[derive(Clone)]
struct Map {
    dx: Coord,
    dy: Coord,
    v: Vec<char>,
}

static ARROWS: &[char; 4] = &['→', '↓', '←', '↑'];
static DIRS: &[Vec2; 4] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];

impl Map {
    fn parse(input: &str) -> Map {
        let dx = input.lines().map(|x| x.len()).max().unwrap_or(0);
        let dy = input.lines().count();

        let mut v = vec![' '; dx * dy];
        for (y, line) in input.lines().enumerate() {
            let row = y * dx;
            for (x, c) in line.chars().enumerate() {
                v[row + x] = c;
            }
        }

        Map {
            dx: dx as Coord,
            dy: dy as Coord,
            v,
        }
    }

    fn start(&self) -> Option<Vec2> {
        self.v
            .iter()
            .enumerate()
            .find_map(|(i, c)| (*c == '.').then_some(i as Coord))
            .map(|i| (i % self.dx, i / self.dx))
    }

    fn walk(&self, p: Vec2, dir: i32, n: i32) -> Vec2 {
        let mut p = p;
        let d = DIRS[dir as usize];
        for _ in 0..n {
            if let Some(q) = self.next_nonwall_wrap(p, d) {
                p = q;
            } else {
                return p;
            }
        }
        p
    }

    fn to_str_lines(&self) -> String {
        self.v
            .chunks(self.dx as usize)
            .map(|chunk| chunk.iter().collect::<String>())
            .map(|s| s.trim_end().to_string() + "\n")
            .collect()
    }

    fn next_nonwall_wrap(&self, p: Vec2, d: Vec2) -> Option<Vec2> {
        let mut q = p;
        loop {
            q.0 = (q.0 + d.0).rem_euclid(self.dx);
            q.1 = (q.1 + d.1).rem_euclid(self.dy);
            if p == q {
                return None;
            }

            let c = self.at(q);
            if c != ' ' {
                return (c == '.').then_some(q);
            }
        }
    }

    fn at(&self, p: Vec2) -> char {
        if self.inside(p) {
            self.v[(p.0 + p.1 * self.dx) as usize]
        } else {
            ' '
        }
    }

    fn set_at(&mut self, p: Vec2, c: char) {
        if self.inside(p) {
            self.v[(p.0 + p.1 * self.dx) as usize] = c;
        }
    }

    fn inside(&self, p: Vec2) -> bool {
        (0..self.dx).contains(&p.0) && (0..self.dy).contains(&p.1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day22_works() {
        let sample = r"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";

        let (m, instr) = parse(sample).unwrap();
        assert_eq!(walk(&m, &instr), 6032);
        assert_eq!(cube_walk(&m, 4, &instr).ok(), Some(5031));
    }
}
