use anyhow::Result;

pub fn run(input: &str) -> Result<String> {
    let p1 = sum(input);
    let p2 = "";
    Ok(format!("{} {}", p1, p2))
}

fn sum(input: &str) -> String {
    to_snafu(input.lines().map(from_snafu).sum())
}

type Num = i64;

fn from_snafu(s: &str) -> Num {
    s.chars()
        .filter_map(from_snafu_char)
        .fold(0, |acc, n| 5 * acc + n)
}

fn to_snafu(i: Num) -> String {
    let mut v = vec![];
    let mut i = i;
    while i > 0 {
        let m = i % 5;
        i /= 5;
        let d = SNAFU_DIGIT[m as usize];
        v.push(d.0);
        i += d.1;
    }
    v.iter().rev().collect()
}

fn from_snafu_char(c: char) -> Option<Num> {
    match c {
        '2' => Some(2),
        '1' => Some(1),
        '0' => Some(0),
        '-' => Some(-1),
        '=' => Some(-2),
        _ => None,
    }
}

const SNAFU_DIGIT: &[(char, Num)] = &[('0', 0), ('1', 0), ('2', 0), ('=', 1), ('-', 1)];
