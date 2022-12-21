use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fmt;

pub fn run(input: &str) -> Result<String> {
    let m = parse(input)?;
    let p1 = eval(&m, Monkey::parse("root"))?;
    let p2 = "";
    Ok(format!("{} {}", p1, p2))
}

type MonkeyMap = HashMap<Monkey, Yell>;

fn eval(m: &MonkeyMap, mky: Monkey) -> Result<Num> {
    Ok(
        match m
            .get(&mky)
            .ok_or_else(|| anyhow!("invalid monkey {}", mky.to_string()))?
        {
            Yell::Const(n) => *n,
            Yell::Add(l, r) => eval(m, *l)? + eval(m, *r)?,
            Yell::Sub(l, r) => eval(m, *l)? - eval(m, *r)?,
            Yell::Mul(l, r) => eval(m, *l)? * eval(m, *r)?,
            Yell::Div(l, r) => eval(m, *l)? / eval(m, *r)?,
        },
    )
}

fn parse(input: &str) -> Result<MonkeyMap> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let errf = || anyhow!("parsing error on line {}: {}", i + 1, line);
            let (m, r) = line.split_once(": ").ok_or_else(errf)?;
            Yell::parse(r)
                .map(|y| (Monkey::parse(m), y))
                .ok_or_else(errf)
        })
        .collect()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Monkey(u32);

impl Monkey {
    fn parse(s: &str) -> Monkey {
        Monkey(
            s.chars()
                .map(|c| (c as u32))
                .fold(0, |acc, x| (acc << 8) | x),
        )
    }

    fn as_string(&self) -> String {
        (0..4)
            .rev()
            .filter_map(|n| char::from_u32((self.0 >> (n * 8)) & 0xFF))
            .collect()
    }
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.as_string())
    }
}

type Num = i64;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Yell {
    Const(Num),
    Add(Monkey, Monkey),
    Sub(Monkey, Monkey),
    Mul(Monkey, Monkey),
    Div(Monkey, Monkey),
}

impl Yell {
    fn parse(s: &str) -> Option<Yell> {
        let s = s.trim();
        if let Ok(x) = s.parse::<Num>() {
            return Some(Yell::Const(x));
        }
        let mut it = s.split(' ');
        let lm = it.next().map(Monkey::parse)?;
        let op = it.next()?.chars().next()?;
        let rm = it.next().map(Monkey::parse)?;
        match op {
            '+' => Some(Yell::Add(lm, rm)),
            '-' => Some(Yell::Sub(lm, rm)),
            '*' => Some(Yell::Mul(lm, rm)),
            '/' => Some(Yell::Div(lm, rm)),
            _ => None,
        }
    }
}
