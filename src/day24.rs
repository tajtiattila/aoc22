use anyhow::{anyhow, Result};
use pathfinding::prelude::astar;

pub fn run(input: &str) -> Result<String> {
    let m = Map::parse(input);
    let p1 = shortest(&m)?;
    let p2 = shortest_2(&m)?;
    Ok(format!("{} {}", p1, p2))
}

fn shortest(m: &Map) -> Result<Coord> {
    let goal = (m.dx - 2, m.dy - 1);
    let ref mut m = TimeMap::from(m);

    let r = astar(
        &(1, 0, 0),
        |&n| m.nextv(n),
        |&n| taxicab(flat(n), goal),
        |&n| flat(n) == goal,
    )
    .map(|x| x.1);
    r.ok_or_else(|| anyhow!("pathfind failed"))
}

fn shortest_2(m: &Map) -> Result<Coord> {
    let ref mut m = TimeMap::from(m);
    let s = m.start;
    let g = m.goal;

    let r = astar(
        &((s.0, s.1, 0), 0),
        |&n| m.nextvs(n),
        |&n| taxicab_est(n, s, g),
        |&n| flat(n.0) == g && n.1 == 2,
    )
    .map(|x| x.1);
    r.ok_or_else(|| anyhow!("pathfind failed"))
}

fn flat(p: Vec3) -> Vec2 {
    (p.0, p.1)
}

fn taxicab(p: Vec2, q: Vec2) -> Coord {
    (p.0 - q.0).abs() + (p.1 - q.1).abs()
}

fn taxicab_est(n: (Vec3, State), start: Vec2, goal: Vec2) -> Coord {
    let p = flat(n.0);
    if n.1 == 0 {
        taxicab(p, goal) + 2 * taxicab(goal, start)
    } else if n.1 == 1 {
        taxicab(p, start) + taxicab(start, goal)
    } else {
        taxicab(p, goal)
    }
}

type State = u8;
type Coord = i16;
type Vec2 = (Coord, Coord);

#[derive(Debug, Clone, Eq, PartialEq)]
struct Map {
    dx: Coord,
    dy: Coord,
    v: Vec<u8>,
}

impl Map {
    fn parse(input: &str) -> Map {
        let dy = input.lines().count();
        let v: Vec<_> = input
            .lines()
            .flat_map(|line| {
                line.chars().map(|c| {
                    ENC.iter()
                        .find_map(|&(ec, ev)| (c == ec).then_some(ev))
                        .unwrap_or(0_u8)
                })
            })
            .collect();
        let dx = v.len() / dy;
        Map {
            dx: dx as Coord,
            dy: dy as Coord,
            v,
        }
    }

    fn blow(&self) -> Map {
        let mut m = Map {
            v: self
                .v
                .iter()
                .map(|&c| if c < 0x10 { c } else { 0 })
                .collect(),
            ..*self
        };
        for x in 1..(self.dx - 1) {
            for y in 1..(self.dy - 1) {
                let p = (x, y);
                let c = self.at(p);
                if c < 0x10 {
                    continue;
                }
                for (i, d) in DIRS.iter().enumerate() {
                    let x = 1 << i;
                    if (c & x) != 0 {
                        *m.at_mut(self.windp(p, *d)) |= 0x10 | x;
                    }
                }
            }
        }
        m
    }

    fn at(&self, p: Vec2) -> u8 {
        self.v[self.pos(p)]
    }

    fn at_mut(&mut self, p: Vec2) -> &mut u8 {
        let p = self.pos(p);
        &mut self.v[p]
    }

    fn windp(&self, p: Vec2, d: Vec2) -> Vec2 {
        (cfix(p.0 + d.0, self.dx), cfix(p.1 + d.1, self.dy))
    }

    fn pos(&self, p: Vec2) -> usize {
        (p.0 + p.1 * self.dx) as usize
    }

    fn to_string_lines(&self) -> String {
        self.v
            .chunks(self.dx as usize)
            .flat_map(|row| {
                row.iter()
                    .copied()
                    .map(wind_char)
                    .chain(std::iter::once('\n'))
            })
            .collect()
    }
}

type Vec3 = (Coord, Coord, Coord);

struct TimeMap {
    dy: Coord,
    start: Vec2,
    goal: Vec2,
    tv: Vec<Map>,
}

impl TimeMap {
    fn from(m0: &Map) -> TimeMap {
        TimeMap {
            dy: m0.dy,
            start: (1, 0),
            goal: (m0.dx - 2, m0.dy - 1),
            tv: vec![m0.clone()],
        }
    }

    fn at(&mut self, p: Vec3) -> u8 {
        let p2 = (p.0, p.1);
        let h = p.2 as usize;
        while self.tv.len() <= h {
            let m = &self.tv.last().unwrap();
            self.tv.push(m.blow());
        }
        self.tv[h].at(p2)
    }

    fn nextv(&mut self, n: Vec3) -> Vec<(Vec3, Coord)> {
        self.nexts(n)
            .map(|p| (p, 1))
            .collect::<Vec<(Vec3, Coord)>>()
    }

    fn nextvs(&mut self, n: (Vec3, State)) -> Vec<((Vec3, State), Coord)> {
        let p = n.0;
        let mut s = n.1;
        if s == 0 && flat(p) == self.goal {
            s = 1;
        } else if s == 1 && flat(p) == self.start {
            s = 2;
        }
        self.nexts(n.0)
            .map(|p| ((p, s), 1))
            .collect::<Vec<((Vec3, State), Coord)>>()
    }

    fn nexts(&mut self, n: Vec3) -> impl Iterator<Item = Vec3> + '_ {
        XDIRS
            .iter()
            .map(move |d| (n.0 + d.0, n.1 + d.1, n.2 + 1))
            .filter(|p| (0..self.dy).contains(&p.1) && self.at(*p) == 0)
    }
}

const DIRC: &[char] = &['^', '>', 'v', '<'];
const DIRS: &[Vec2] = &[(0, -1), (1, 0), (0, 1), (-1, 0)];
const XDIRS: &[Vec2] = &[(0, -1), (1, 0), (0, 1), (-1, 0), (0, 0)];

const ENC: &[(char, u8)] = &[
    ('#', 0x01),
    ('^', 0x11),
    ('>', 0x12),
    ('v', 0x14),
    ('<', 0x18),
];

fn wind_char(b: u8) -> char {
    if b < 0x10 {
        return if b == 0 { '.' } else { '#' };
    }
    let (i, n) = (0..4).fold((0, 0), |(i, n), bit| {
        if b & (1 << bit) != 0 {
            (bit, n + 1)
        } else {
            (i, n)
        }
    });
    if n == 1 {
        DIRC[i]
    } else {
        char::from_digit(n, 10).unwrap()
    }
}

fn cfix(p: Coord, d: Coord) -> Coord {
    if p == 0 {
        d - 2
    } else if p == d - 1 {
        1
    } else {
        p
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day24_wind() {
        let start = "\
#.#####
#...v.#
#..>..#
#.....#
#<....#
#..v..#
#####.#
";
        let t1 = "\
#.#####
#..v..#
#...2.#
#.....#
#....<#
#.....#
#####.#
";
        let t2 = "\
#.#####
#.....#
#..v.>#
#...v.#
#...<.#
#.....#
#####.#
";

        let m = Map::parse(start);
        let sim = |n| {
            let mut m = m.clone();
            for _ in 0..n {
                m = m.blow();
            }
            m.to_string_lines()
        };
        println!("{}", m.blow().to_string_lines());
        assert_eq!(sim(1), t1);
        println!("{}", m.blow().blow().to_string_lines());
        assert_eq!(sim(2), t2);
    }
}
