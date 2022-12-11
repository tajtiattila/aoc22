use crate::Options;

pub fn run(input: &str, _: &Options) -> anyhow::Result<String> {
    let (stk, vrearr) = parse(input);

    let p1 = sim(move1, &stk, &vrearr);
    let p2 = sim(move2, &stk, &vrearr);
    Ok(format!("{} {}", p1, p2))
}

type Stacks = Vec<Vec<char>>;

type Rearr = (usize, usize, usize);

fn parse(input: &str) -> (Stacks, Vec<Rearr>) {
    let mut stkv = Vec::new();
    let mut pref = true;
    let mut moves = Vec::new();
    for line in input.lines() {
        if line.starts_with(" 1   2   3 ") {
            pref = false;
        } else if pref {
            stkv.push(line);
        } else if line.starts_with("move") {
            if let Some(r) = parse_rearr(line) {
                moves.push(r);
            }
        }
    }

    let mut stk = Stacks::new();
    let mut add = |i: usize, c: char| {
        if stk.len() <= i {
            stk.resize_with(i + 1, Vec::new);
        }
        stk[i].push(c);
    };

    for line in stkv.iter().rev() {
        line.chars().enumerate().for_each(|(i, c)| {
            if i % 4 == 1 && c != ' ' {
                add(i / 4, c);
            }
        })
    }

    (stk, moves)
}

fn parse_rearr(line: &str) -> Option<Rearr> {
    let mut it = line
        .split_whitespace()
        .enumerate()
        .filter_map(|(i, p)| (i % 2 == 1).then_some(p.parse::<usize>().ok()?));
    Some((it.next()?, it.next()?, it.next()?))
}

fn sim<F>(mut f: F, stk: &Stacks, vrearr: &[Rearr]) -> String
where
    F: FnMut(&mut Stacks, Rearr),
{
    let mut stk = stk.clone();
    vrearr.iter().copied().for_each(|r| f(&mut stk, r));

    stk.iter().filter_map(|x| x.last()).collect()
}

fn move1(stk: &mut Stacks, r: Rearr) {
    let (n, from, to) = r;
    for _ in 0..n {
        if let Some(c) = stk[from - 1].pop() {
            stk[to - 1].push(c);
        }
    }
}

fn move2(stk: &mut Stacks, r: Rearr) {
    let (n, from, to) = r;
    if from == to {
        return;
    }
    let l0 = stk[from - 1].len();
    let si = if n < l0 { l0 - n } else { 0 };
    let l1 = stk[to - 1].len();
    stk[to - 1].reserve(l1 + n);
    for i in si..l0 {
        let c = stk[from - 1][i];
        stk[to - 1].push(c);
    }
    stk[from - 1].truncate(si);
}
