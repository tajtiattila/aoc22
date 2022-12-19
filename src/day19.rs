use pathfinding::prelude::bfs_reach;

pub fn run(input: &str) -> anyhow::Result<String> {
    let bps = parse(input);
    let p1 = sim_all(&bps, 24);
    let p2 = "";
    Ok(format!("{} {}", p1, p2))
}

fn parse(input: &str) -> Vec<Blueprint> {
    input.lines().filter_map(Blueprint::parse).collect()
}

fn sim_all(bps: &[Blueprint], time: u8) -> usize {
    bps.iter()
        .filter_map(|bp| sim(bp, time).map(|n| (bp.num as usize) * n))
        .sum()
}

fn sim(bp: &Blueprint, time: u8) -> Option<usize> {
    println!("Blueprint: {:?}", bp);
    #[allow(clippy::unnecessary_lazy_evaluations)]
    bfs_reach((State::new(), time), |&(n, ttg)| {
        n.nexts(bp)
            .filter_map(move |m| (ttg > 0).then(|| (m, ttg - 1)))
    })
    .filter(|&(_, ttg)| (ttg == 0))
    .map(|(n, _)| n.res.gde as usize)
    .max()
}

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

type Count = u8;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Counts {
    ore: Count,
    cly: Count,
    obs: Count,
    gde: Count,
}
