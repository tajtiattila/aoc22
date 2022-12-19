use anyhow::{anyhow, bail, Result};
use std::cmp::Ordering;

pub fn run(input: &str) -> Result<String> {
    let trees = parse(input)?;

    let p1 = check_sort(&trees);
    let p2 = decoder_key(&trees);
    Ok(format!("{} {}", p1, p2))
}

fn check_sort(trees: &[Tree]) -> usize {
    trees
        .chunks(2)
        .enumerate()
        .filter_map(|(i, v)| (v.len() == 2 && Tree::order_ok(&v[0], &v[1])).then_some(i + 1))
        .sum()
}

fn decoder_key(trees: &[Tree]) -> usize {
    let mut v = Vec::from(trees);
    let divider = |n| Tree::List(vec![Tree::List(vec![Tree::Num(n)])]);
    let d2 = divider(2);
    let d6 = divider(6);
    v.push(d2.clone());
    v.push(d6.clone());
    v.sort();

    let idx = |item| {
        v.iter()
            .enumerate()
            .find(|(_, x)| x == &item)
            .map(|(i, _)| i + 1)
            .unwrap()
    };
    idx(&d2) * idx(&d6)
}

fn parse(input: &str) -> Result<Vec<Tree>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(Tree::parse)
        .collect()
}

#[derive(Debug, Eq, Clone)]
enum Tree {
    Num(u32),
    List(Vec<Tree>),
}

impl Tree {
    fn parse(line: &str) -> Result<Tree> {
        let mut tokens = Vec::new();
        let mut is_num = false;
        let mut num: u32 = 0;
        let mut nopen = 0;
        for c in line.chars() {
            if c == ',' || c == ']' {
                if is_num {
                    tokens.push(Token::Num(num));
                }
                is_num = false;
                num = 0;
            }
            if c == '[' {
                tokens.push(Token::Open);
                nopen += 1;
            } else if c == ']' {
                tokens.push(Token::Close);
                nopen -= 1;
                if nopen < 0 {
                    bail!("unmatched parenthesis");
                }
            } else if c == ',' {
                // pass
            } else if let Some(i) = c.to_digit(10) {
                num = num * 10 + i;
                is_num = true;
            } else {
                bail!("invalid char {}", c);
            }
        }

        let mut stack = vec![Tree::List(Vec::new())];
        for tok in tokens {
            match tok {
                Token::Open => stack.push(Tree::List(Vec::new())),
                Token::Close => {
                    let last = stack.pop().unwrap();
                    match stack.last_mut().unwrap() {
                        Tree::List(v) => v.push(last),
                        _ => panic!("impossible"),
                    }
                }
                Token::Num(n) => {
                    let m = stack.last_mut().unwrap();
                    match m {
                        Tree::List(v) => v.push(Tree::Num(n)),
                        _ => panic!("impossible"),
                    }
                }
            }
        }

        stack.pop().ok_or_else(|| anyhow!("empty input"))
    }

    fn order_ok(l: &Tree, r: &Tree) -> bool {
        l <= r
    }

    fn list_order(av: &[Self], bv: &[Self]) -> Ordering {
        std::iter::zip(av, bv)
            .map(|(a, b)| a.cmp(b))
            .find(|&x| x != Ordering::Equal)
            .unwrap_or_else(|| av.len().cmp(&bv.len()))
    }
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tree {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Num(a), Self::Num(b)) => a.cmp(b),
            (Self::List(av), Self::List(bv)) => Self::list_order(av, bv),
            (Self::Num(a), Self::List(bv)) => Self::list_order(&[Self::Num(*a)], bv),
            (Self::List(av), Self::Num(b)) => Self::list_order(av, &[Self::Num(*b)]),
        }
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Open,
    Close,
    Num(u32),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day13_works() {
        assert_eq!(check_sort(&parse(SAMPLE).unwrap()), 13);
    }

    const SAMPLE: &str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";
}
