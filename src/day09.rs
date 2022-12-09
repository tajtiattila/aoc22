use crate::{day_ok, DayResult, Options};
use std::collections::HashSet;

pub fn run(input: &str, _: &Options) -> DayResult {
    let p1 = stretch(input, 2);
    let p2 = stretch(input, 10);
    day_ok(p1, p2)
}

fn stretch(input: &str, rope_len: usize) -> usize {
    let mut seen = HashSet::new();
    let mut rope = Vec::new();
    rope.resize(rope_len, (0, 0));

    for (step, n_steps) in input.lines().filter_map(parse) {
        for _ in 0..n_steps {
            let head = rope[0];
            rope[0] = (head.0 + step.0, head.1 + step.1);

            for i in 1..rope_len {
                let last = rope[i - 1];
                pull(&mut rope[i], last);
            }

            seen.insert(rope[rope_len - 1]);
        }
    }

    seen.len()
}

fn pull(t: &mut (i32, i32), h: (i32, i32)) {
    let dx = h.0 - t.0;
    let dy = h.1 - t.1;
    let mut mx = |v: i32| {
        if dx < -v {
            t.0 -= 1
        } else if dx > v {
            t.0 += 1
        }
    };
    let mut my = |v: i32| {
        if dy < -v {
            t.1 -= 1
        } else if dy > v {
            t.1 += 1
        }
    };
    if dx == 0 {
        my(1);
    } else if dy == 0 {
        mx(1);
    } else if dx.abs() > 1 || dy.abs() > 1 {
        mx(0);
        my(0);
    }
}

fn parse(line: &str) -> Option<((i32, i32), i32)> {
    let (d, n) = line.split_once(' ')?;

    let d = if d == "U" {
        (0, 1)
    } else if d == "D" {
        (0, -1)
    } else if d == "L" {
        (-1, 0)
    } else if d == "R" {
        (1, 0)
    } else {
        return None;
    };

    Some((d, n.parse().ok()?))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stretch_works() {
        let sample = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

        assert_eq!(stretch(sample, 2), 13);
    }
}
