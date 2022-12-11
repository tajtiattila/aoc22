use std::collections::HashMap;

pub fn run(input: &str, _: &crate::Options) -> anyhow::Result<String> {
    let monkeys = parse(input);
    let v = sim(&monkeys, 20);
    let p1 = v[0] * v[1];
    Ok(format!("{} {}", p1, ""))
}

fn parse(input: &str) -> Vec<Monkey> {
    input.split("\n\n").filter_map(Monkey::parse).collect()
}

fn sim(horde: &[Monkey], n: usize) -> Vec<usize> {
    let mut horde = horde.to_vec();
    let mut inspects = HashMap::new();
    for _ in 0..n {
        round(&mut horde, &mut inspects);
    }
    let mut v: Vec<usize> = inspects.values().copied().collect();
    v.sort_by_key(|x| std::cmp::Reverse(*x));
    v
}

fn round(horde: &mut [Monkey], inspects: &mut HashMap<usize, usize>) {
    for i in 0..horde.len() {
        let v = horde[i].items.split_off(0);
        inspects
            .entry(i)
            .and_modify(|n| *n += v.len())
            .or_insert(v.len());
        for item in v {
            let m = &horde[i];
            let n = m.op.apply(item) / 3;
            let j = if n % m.div == 0 {
                m.if_true
            } else {
                m.if_false
            };
            horde[j].items.push(n);
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    num: usize,
    items: Vec<i32>,
    op: Op,
    div: i32,
    if_true: usize,
    if_false: usize,
}

impl Monkey {
    fn parse(input: &str) -> Option<Monkey> {
        let mut monkey = Monkey {
            num: 0,
            items: Vec::new(),
            op: Op::Add(0),
            div: 1,
            if_true: 0,
            if_false: 0,
        };
        let mut mask: i32 = 0;
        for line in input.trim().lines().map(|x| x.trim()) {
            if let Some(s) = line.strip_prefix("Monkey ") {
                monkey.num = s.trim_end_matches(':').parse().ok()?;
                mask |= 1;
            } else if let Some(s) = line.strip_prefix("Starting items: ") {
                monkey.items = s.split(", ").filter_map(|x| x.parse().ok()).collect();
                mask |= 2;
            } else if let Some(s) = line.strip_prefix("Operation: new = old") {
                if s.trim() == "* old" {
                    monkey.op = Op::Square;
                } else if let Some(p) = s.trim().strip_prefix('+') {
                    monkey.op = Op::Add(p.trim().parse().ok()?);
                } else if let Some(p) = s.trim().strip_prefix('*') {
                    monkey.op = Op::Mul(p.trim().parse().ok()?);
                } else {
                    println!("operr {}", s.trim());
                    return None;
                }
                mask |= 4;
            } else if let Some(s) = line.strip_prefix("Test: divisible by ") {
                monkey.div = s.parse().ok()?;
                mask |= 8;
            } else if let Some(s) = line.strip_prefix("If true: throw to monkey ") {
                monkey.if_true = s.parse().ok()?;
                mask |= 16;
            } else if let Some(s) = line.strip_prefix("If false: throw to monkey ") {
                monkey.if_false = s.parse().ok()?;
                mask |= 32;
            }
        }

        (mask == 63).then_some(monkey)
    }
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Add(i32),
    Mul(i32),
    Square,
}

impl Op {
    fn apply(&self, n: i32) -> i32 {
        match *self {
            Op::Add(m) => n + m,
            Op::Mul(m) => n * m,
            Op::Square => n * n,
        }
    }
}
