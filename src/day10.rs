use crate::{day_ok, DayResult, Options};

pub fn run(input: &str, _: &Options) -> DayResult {
    let p1 = signal_strength(input);
    day_ok(p1, "")
}

fn signal_strength(input: &str) -> i32 {
    let mut x = 1;
    input
        .lines()
        .filter_map(asm)
        .flat_map(|instr| {
            let x0 = x;
            let n = match instr {
                Instr::Addx(v) => {
                    x += v;
                    2
                }
                Instr::Noop => 1,
            };
            std::iter::repeat(x0).take(n)
        })
        .enumerate()
        .filter_map(|(i, x)| {
            let i = (i + 1) as i32;
            ((i % 40) == 20).then_some(i * x)
        })
        .sum()
}

fn asm(line: &str) -> Option<Instr> {
    let v: Vec<_> = line.split(' ').collect();
    if v.is_empty() {
        return None;
    }
    if v[0] == "addx" && v.len() == 2 {
        return Some(Instr::Addx(v[1].parse().ok()?));
    } else if v[0] == "noop" {
        return Some(Instr::Noop);
    }
    None
}

enum Instr {
    Addx(i32),
    Noop,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn signal_strength_works() {
        let sample = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

        assert_eq!(signal_strength(sample), 13140);
    }
}
