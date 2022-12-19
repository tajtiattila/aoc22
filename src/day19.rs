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
    let mut seen = HashSet::new();
    let mut queue = VecDeque::from([State::new(time)]);
    let mut best = 0;
    while let Some(s) = queue.pop_front() {
        best = max(best, s.score());
        if s.ttg <= 1 {
            continue;
        }
        for r in s.nexts2(bp) {
            if seen.insert(r) {
                queue.push_back(r);
            }
        }
    }

    if crate::verbose() {
        let msg;
        println!(
            "  Blueprint {:2}: {}",
            bp.num,
            match best {
                0 => "no geode",
                1 => " 1 geode",
                _ => {
                    msg = format!("{:2} geodes", best);
                    msg.as_str()
                }
            }
        );
    }

    best
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

    ore_max: Count,
}

impl Blueprint {
    fn parse(line: &str) -> Option<Blueprint> {
        let mut it = line
            .split(' ')
            .map(|s| s.trim_end_matches(':'))
            .filter_map(|s| s.parse().ok());
        Some(Blueprint::from(
            it.next()?,
            it.next()?,
            it.next()?,
            it.next()?,
            it.next()?,
            it.next()?,
            it.next()?,
        ))
    }

    fn from(
        num: Count,
        ore_ore: Count,
        cly_ore: Count,
        obs_ore: Count,
        obs_cly: Count,
        gde_ore: Count,
        gde_obs: Count,
    ) -> Blueprint {
        Blueprint {
            num,
            ore_ore,
            cly_ore,
            obs_ore,
            obs_cly,
            gde_ore,
            gde_obs,
            ore_max: [ore_ore, cly_ore, obs_ore, gde_ore]
                .into_iter()
                .max()
                .unwrap(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    ttg: usize,    // time to go
    robot: Counts, // no. of robots for each resource
    res: Counts,   // available resources
}

impl State {
    fn new(ttg: usize) -> State {
        State {
            ttg,
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

    fn score(&self) -> usize {
        (self.res.gde as usize) + self.ttg * (self.robot.gde as usize)
    }

    fn show(&self) -> String {
        format!(
            "@{:02}  ore{:2}+{}  cly{:2}+{}  obs{:2}+{}  gde{:2}+{}",
            self.ttg,
            self.res.ore,
            self.robot.ore,
            self.res.cly,
            self.robot.cly,
            self.res.obs,
            self.robot.obs,
            self.res.gde,
            self.robot.gde
        )
    }

    fn nexts2(&self, bp: &Blueprint) -> impl Iterator<Item = State> {
        [
            self.make_ore_robot(bp),
            self.make_cly_robot(bp),
            self.make_obs_robot(bp),
            self.make_gde_robot(bp),
        ]
        .into_iter()
        .flatten()
    }

    fn make_ore_robot(&self, bp: &Blueprint) -> Option<State> {
        if Self::res_max(self.ttg, self.res.ore, self.robot.ore, bp.ore_max) {
            return None;
        }
        let t = Self::build_turn(bp.ore_ore, self.res.ore, self.robot.ore);
        (t <= self.ttg).then(|| {
            let mut s = self.wait(t);
            s.robot.ore += 1;
            s.res.ore -= bp.ore_ore;
            self.sim_dump("ore", t, &s);
            s
        })
    }

    fn make_cly_robot(&self, bp: &Blueprint) -> Option<State> {
        if Self::res_max(self.ttg, self.res.cly, self.robot.cly, bp.obs_cly) {
            return None;
        }
        let t = Self::build_turn(bp.cly_ore, self.res.ore, self.robot.ore);
        (t <= self.ttg).then(|| {
            let mut s = self.wait(t);
            s.robot.cly += 1;
            s.res.ore -= bp.cly_ore;
            self.sim_dump("cly", t, &s);
            s
        })
    }

    fn make_obs_robot(&self, bp: &Blueprint) -> Option<State> {
        if Self::res_max(self.ttg, self.res.obs, self.robot.obs, bp.gde_obs) {
            return None;
        }
        let t = max(
            Self::build_turn(bp.obs_ore, self.res.ore, self.robot.ore),
            Self::build_turn(bp.obs_cly, self.res.cly, self.robot.cly),
        );
        (t <= self.ttg).then(|| {
            let mut s = self.wait(t);
            s.robot.obs += 1;
            s.res.ore -= bp.obs_ore;
            s.res.cly -= bp.obs_cly;
            self.sim_dump("obs", t, &s);
            s
        })
    }

    fn make_gde_robot(&self, bp: &Blueprint) -> Option<State> {
        let t = max(
            Self::build_turn(bp.gde_ore, self.res.ore, self.robot.ore),
            Self::build_turn(bp.gde_obs, self.res.obs, self.robot.obs),
        );
        (t <= self.ttg).then(|| {
            let mut s = self.wait(t);
            s.robot.gde += 1;
            s.res.ore -= bp.gde_ore;
            s.res.obs -= bp.gde_obs;
            self.sim_dump("gde", t, &s);
            s
        })
    }

    fn build_turn(goal: Count, cur: Count, inc: Count) -> usize {
        if inc == 0 {
            return usize::MAX;
        }
        if cur >= goal {
            return 1;
        }
        let d = (goal - cur + inc - 1) as usize;
        let r = d / (inc as usize);
        assert!(cur + (r as Count) * inc >= goal);
        r + 1
    }

    fn wait(&self, t: usize) -> State {
        let count = t as Count;
        State {
            ttg: self.ttg - t,
            robot: self.robot,
            res: Counts {
                ore: self.res.ore + count * self.robot.ore,
                cly: self.res.cly + count * self.robot.cly,
                obs: self.res.obs + count * self.robot.obs,
                gde: self.res.gde + count * self.robot.gde,
            },
        }
    }

    // https://www.reddit.com/r/adventofcode/comments/zpy5rm/2022_day_19_what_are_your_insights_and/
    fn res_max(ttg: usize, cur: Count, inc: Count, us: Count) -> bool {
        (cur as usize) + ttg * (inc as usize) >= ttg * (us as usize)
    }

    fn sim_dump(&self, what: &str, t: usize, next: &State) {
        const DUMP_SIM: bool = false;

        if DUMP_SIM {
            println!(
                " {}  +{} in {:2} -> {} ({})",
                self.show(),
                what,
                t,
                next.show(),
                next.score()
            );
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Counts {
    ore: Count,
    cly: Count,
    obs: Count,
    gde: Count,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bfs_sim_works() {
        let sample = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.\n";
        assert_eq!(bfs_sim(&Blueprint::parse(sample).unwrap(), 24), 9);
    }

    #[test]
    fn bfs_sim2_works() {
        let sample = "Blueprint 24: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 7 clay. Each geode robot costs 3 ore and 9 obsidian.\n";
        assert_eq!(bfs_sim(&Blueprint::parse(sample).unwrap(), 24), 9);
    }
}
