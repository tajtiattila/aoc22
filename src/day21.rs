use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fmt;

pub fn run(input: &str) -> Result<String> {
    let m = parse(input)?;
    let p1 = Eval::new(&m).root()?;
    let p2 = find_yell(&m)?;
    Ok(format!("{} {}", p1, p2))
}

fn find_yell(m: &MonkeyMap) -> Result<Num> {
    find_zero(0, |x| Eval::with_human(m, x).root().unwrap())
        .ok_or_else(|| anyhow!("can't find what to yell"))
}

fn find_zero<FN>(start: Num, mut f: FN) -> Option<Num>
where
    FN: FnMut(Num) -> Num,
{
    let mut p = (start, f(start));
    let mut q = (0..1_000_000).map(|x| (x, f(x))).find(|(_, y)| *y != p.1)?;
    let mut dx = q.0 - p.0;
    let dir = sign(0 - p.0) - sign(q.1 - p.1);

    while sign(p.1) * sign(q.1) > 0 {
        let x = q.0 + dir * dx;
        p = q;
        q = (x, f(x));
        dx *= 2;
    }

    loop {
        if p.0 + 1 == q.0 {
            return None;
        }

        let x = (p.0 + q.0) / 2;
        let m = (x, f(x));
        if m.1 == 0 {
            return Some(m.0);
        }

        if sign(p.1) * sign(m.1) > 0 {
            p = m;
        } else {
            q = m;
        }
    }
}

fn sign(x: Num) -> Num {
    if x != 0 {
        if x < 0 {
            -1
        } else {
            1
        }
    } else {
        0
    }
}

type MonkeyMap = HashMap<Monkey, Yell>;

struct Eval<'a> {
    m: &'a MonkeyMap,
    humn: Option<Num>,
}

static ROOT: Monkey = Monkey::from_chars('r', 'o', 'o', 't');
static HUMN: Monkey = Monkey::from_chars('h', 'u', 'm', 'n');

impl<'a> Eval<'a> {
    fn new(m: &'a MonkeyMap) -> Self {
        Self { m, humn: None }
    }

    fn with_human(m: &'a MonkeyMap, humn: Num) -> Self {
        Self {
            m,
            humn: Some(humn),
        }
    }

    fn root(&self) -> Result<Num> {
        self.eval(ROOT)
    }

    fn eval(&self, mky: Monkey) -> Result<Num> {
        let yell = self
            .m
            .get(&mky)
            .ok_or_else(|| anyhow!("invalid monkey {}", mky.to_string()))?;
        Ok(match yell {
            Yell::Const(x) => {
                let humn = if mky == HUMN { self.humn } else { None };
                if let Some(z) = humn {
                    z
                } else {
                    *x
                }
            }
            Yell::Calc(op, l, r) => {
                let l = self.eval(*l)?;
                let r = self.eval(*r)?;
                if mky == ROOT && self.humn.is_some() {
                    l - r
                } else {
                    op.calc(l, r)
                }
            }
        })
    }
}

fn parse(input: &str) -> Result<MonkeyMap> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let errf = || anyhow!("parsing error on line {}: {}", i + 1, line);
            let (m, r) = line.split_once(": ").ok_or_else(errf)?;
            Yell::parse(r)
                .map(|y| (Monkey::from(m), y))
                .ok_or_else(errf)
        })
        .collect()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Monkey(u32);

impl Monkey {
    fn from(s: &str) -> Monkey {
        Monkey(
            s.chars()
                .map(|c| (c as u32))
                .fold(0, |acc, x| (acc << 8) | x),
        )
    }

    const fn from_chars(a: char, b: char, c: char, d: char) -> Monkey {
        Monkey(((a as u32) << 24) | ((b as u32) << 16) | ((c as u32) << 8) | (d as u32))
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
    Calc(Op, Monkey, Monkey),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn calc(self, l: Num, r: Num) -> Num {
        match self {
            Self::Add => l + r,
            Self::Sub => l - r,
            Self::Mul => l * r,
            Self::Div => l / r,
        }
    }
}

impl Yell {
    fn parse(s: &str) -> Option<Yell> {
        let s = s.trim();
        if let Ok(x) = s.parse::<Num>() {
            return Some(Yell::Const(x));
        }
        let mut it = s.split(' ');
        let lm = it.next().map(Monkey::from)?;
        let op = it.next()?.chars().next()?;
        let rm = it.next().map(Monkey::from)?;
        match op {
            '+' => Some(Yell::Calc(Op::Add, lm, rm)),
            '-' => Some(Yell::Calc(Op::Sub, lm, rm)),
            '*' => Some(Yell::Calc(Op::Mul, lm, rm)),
            '/' => Some(Yell::Calc(Op::Div, lm, rm)),
            _ => None,
        }
    }
}
