use anyhow::{anyhow, bail, Result};
use pathfinding::prelude::{astar, bfs};

pub fn run(input: &str) -> Result<String> {
    let p = parse(input)?;
    let p1 = path_len(&p).ok_or_else(|| anyhow!("path failed"))?;
    // 452 too high
    let p2 = hike_len(&p).ok_or_else(|| anyhow!("hike failed"))?;
    Ok(format!("{} {}", p1, p2))
}

fn parse(input: &str) -> Result<Problem> {
    let mut map = Map::parse(input)?;
    let start = map
        .find_replace_one(b'S', b'a')
        .ok_or_else(|| anyhow!("can't find start"))?;
    let goal = map
        .find_replace_one(b'E', b'z')
        .ok_or_else(|| anyhow!("can't find goal"))?;
    Ok(Problem { map, start, goal })
}

fn path_len(problem: &Problem) -> Option<i32> {
    let m = &problem.map;
    let g = problem.goal;
    astar(
        &problem.start,
        |&p| {
            let max = m.at(p).unwrap() + 1;
            //println!("{};{} {}", p.0, p.1, m.at(p).unwrap() as char);
            NEIGHBORS.iter().filter_map(move |&d| {
                let q = (p.0 + d.0, p.1 + d.1);
                //println!("  {};{} {}", q.0, q.1, m.at(q).unwrap_or(64) as char);
                (m.at(q)? <= max).then_some((q, 1))
            })
        },
        |&p| (g.0 - p.0).abs() + (g.1 - p.1).abs(),
        |&p| p == g,
    )
    .map(|r| r.1)
}

fn hike_len(problem: &Problem) -> Option<i32> {
    let m = &problem.map;
    bfs(
        &problem.goal,
        |&p| {
            let href = m.at(p).unwrap();
            //println!("{};{} {}", p.0, p.1, m.at(p).unwrap() as char);
            NEIGHBORS.iter().filter_map(move |&d| {
                let q = (p.0 + d.0, p.1 + d.1);
                //println!("  {};{} {}", q.0, q.1, m.at(q).unwrap_or(64) as char);
                (m.at(q)? >= href - 1).then_some(q)
            })
        },
        |&p| m.at(p) == Some(b'a'),
    )
    .map(|r| r.len() as i32 - 1)
}

const NEIGHBORS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

struct Problem {
    map: Map,
    start: (i32, i32),
    goal: (i32, i32),
}

#[derive(Debug, Clone)]
pub struct Map {
    dx: i32,
    dy: i32,
    m: Vec<u8>,
}

impl Map {
    pub fn parse(input: &str) -> Result<Map> {
        let dx = input
            .lines()
            .map(|x| x.len())
            .max()
            .ok_or_else(|| anyhow!("empty input"))?;
        let dy = input.lines().count();

        for (i, line) in input.lines().enumerate() {
            if line.len() != dx {
                bail!(
                    "line {} has invalid length {} != {}:\n{}",
                    i + 1,
                    line.len(),
                    dx,
                    line
                );
            }
        }

        let v = input
            .lines()
            .flat_map(|x| x.as_bytes().to_vec())
            .collect::<Vec<_>>();
        Ok(Map {
            dx: dx as i32,
            dy: dy as i32,
            m: v,
        })
    }

    pub fn inside(&self, (x, y): (i32, i32)) -> bool {
        (0..self.dx).contains(&x) && (0..self.dy).contains(&y)
    }

    pub fn pos(&self, (x, y): (i32, i32)) -> Option<usize> {
        self.inside((x, y)).then_some((x + y * self.dx) as usize)
    }

    pub fn at(&self, p: (i32, i32)) -> Option<u8> {
        self.pos(p).map(|n| self.m[n])
    }

    pub fn at_mut(&mut self, p: (i32, i32)) -> Option<&mut u8> {
        self.pos(p).map(|n| &mut self.m[n])
    }

    pub fn places(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        (0..self.dy).flat_map(|y| (0..self.dx).map(move |x| (x, y)))
    }

    pub fn find_replace_one(&mut self, find: u8, repl: u8) -> Option<(i32, i32)> {
        let p = self.places().find(|&p| self.at(p).unwrap() == find)?;
        *self.at_mut(p).unwrap() = repl;
        Some(p)
    }
}
