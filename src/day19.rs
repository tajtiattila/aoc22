use std::cmp::max;
use std::collections::{HashSet, VecDeque};

pub fn run(input: &str) -> anyhow::Result<String> {
    let bps = &parse(input);
    let verbose = crate::verbose();
    if verbose {
        println!("Problem 1");
    }
    let p1 = sim1(bps);
    if verbose {
        println!("Problem 2");
    }
    let p2 = sim2(bps);
    Ok(format!("{} {}", p1, p2))
}

fn parse(input: &str) -> Vec<Blueprint> {
    input.lines().filter_map(Blueprint::parse).collect()
}

fn sim1(bps: &[Blueprint]) -> usize {
    bps.iter()
        .map(|bp| (bp.num as usize) * bfs_sim(bp, 24))
        .sum()
}

fn sim2(bps: &[Blueprint]) -> usize {
    bps.iter().take(3).map(|bp| bfs_sim(bp, 32)).product()
}

fn bfs_sim(bp: &Blueprint, time: usize) -> usize {
    const MAX_SCORE_DIFF: usize = 1;
    let mut scores = Vec::new();
    scores.resize(time + 1, 0);
    let mut seen = HashSet::new();
    let mut queue = VecDeque::from([(State::new(), time)]);
    let mut bestg = 0;
    while let Some((s, t)) = queue.pop_front() {
        if t <= 1 {
            bestg = max(bestg, s.res.gde + s.robot.gde);
            continue;
        }
        if s.score(t) + MAX_SCORE_DIFF < scores[t] {
            continue;
        }
        for r in s.nexts(bp) {
            let t = t - 1;
            let rscore = r.score(t);
            if rscore + MAX_SCORE_DIFF >= scores[t] && seen.insert((r, t)) {
                scores[t] = max(scores[t], rscore);
                queue.push_back((r, t));
            }
        }
    }

    if crate::verbose() {
        let msg;
        println!(
            "  Blueprint {:2}: {}",
            bp.num,
            match bestg {
                0 => "no geode",
                1 => " 1 geode",
                _ => {
                    msg = format!("{:2} geodes", bestg);
                    msg.as_str()
                }
            }
        );
    }

    bestg.into()
}

type Count = u8;

#[derive(Debug, Copy, Clone)]
struct Blueprint {
    num: Count,

    ore_ore: Count,

    cly_ore: Count,

    obs_ore: Count,
    obs_cly: Count,

    gde_ore: Count,
    gde_obs: Count,
}

impl Blueprint {
    fn parse(line: &str) -> Option<Blueprint> {
        let mut it = line
            .split(' ')
            .map(|s| s.trim_end_matches(':'))
            .filter_map(|s| s.parse().ok());
        Some(Blueprint {
            num: it.next()?,
            ore_ore: it.next()?,
            cly_ore: it.next()?,
            obs_ore: it.next()?,
            obs_cly: it.next()?,
            gde_ore: it.next()?,
            gde_obs: it.next()?,
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    robot: Counts, // no. of robots for each resource
    res: Counts,   // available resources
}

impl State {
    fn new() -> State {
        State {
            robot: Counts {
                ore: 1,
                cly: 0,
                obs: 0,
                gde: 0,
            },
            res: Counts {
                ore: 0,
                cly: 0,
                obs: 0,
                gde: 0,
            },
        }
    }

    fn score(&self, time_left: usize) -> usize {
        (self.res.gde as usize) + time_left + (self.robot.gde as usize)
    }

    #[allow(clippy::unnecessary_lazy_evaluations)]
    fn nexts(&self, bp: &Blueprint) -> impl Iterator<Item = State> {
        let nres = Counts {
            ore: self.res.ore + self.robot.ore,
            cly: self.res.cly + self.robot.cly,
            obs: self.res.obs + self.robot.obs,
            gde: self.res.gde + self.robot.gde,
        };

        //println!("{:?} {:?}", self.res, bp);
        let m_ore = (self.res.ore >= bp.ore_ore).then(|| State {
            robot: Counts {
                ore: self.robot.ore + 1,
                ..self.robot
            },
            res: Counts {
                ore: nres.ore - bp.ore_ore,
                ..nres
            },
        });

        let m_cly = (self.res.ore >= bp.cly_ore).then(|| State {
            robot: Counts {
                cly: self.robot.cly + 1,
                ..self.robot
            },
            res: Counts {
                ore: nres.ore - bp.cly_ore,
                ..nres
            },
        });

        let m_obs = (self.res.ore >= bp.obs_ore && self.res.cly >= bp.obs_cly).then(|| State {
            robot: Counts {
                obs: self.robot.obs + 1,
                ..self.robot
            },
            res: Counts {
                ore: nres.ore - bp.obs_ore,
                cly: nres.cly - bp.obs_cly,
                ..nres
            },
        });

        let m_gde = (self.res.ore >= bp.gde_ore && self.res.obs >= bp.gde_obs).then(|| State {
            robot: Counts {
                gde: self.robot.gde + 1,
                ..self.robot
            },
            res: Counts {
                ore: nres.ore - bp.gde_ore,
                obs: nres.obs - bp.gde_obs,
                ..nres
            },
        });

        [
            Some(State {
                robot: self.robot,
                res: nres,
            }),
            m_ore,
            m_cly,
            m_obs,
            m_gde,
        ]
        .into_iter()
        .flatten()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Counts {
    ore: Count,
    cly: Count,
    obs: Count,
    gde: Count,
}
