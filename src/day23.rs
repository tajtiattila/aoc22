use anyhow::Result;
use std::collections::HashSet;

pub fn run(input: &str) -> Result<String> {
    let p1 = input.len();
    let p2 = "";
    Ok(format!("{} {}", p1, p2))
}

struct Sim<'a> {
    m: HashSet<Vec2>,
    proposal: [Proposal<'a>; 4],
}

impl Sim<'_> {
    fn from(input: &str) -> Sim {
        Sim {
            m: poss(input).collect(),
            proposal: PROPOSALS,
        }
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

static PROPOSALS: [Proposal; 4] = [
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
