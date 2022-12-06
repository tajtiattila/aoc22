use crate::{day_ok, DayResult, Options};

pub fn run(input: &str, _: &Options) -> DayResult {
    let input = input.trim_matches(char::is_whitespace);

    day_ok(nproc_start(input, 4), nproc_start(input, 14))
}

fn nproc_start(s: &str, n: usize) -> usize {
    s.as_bytes()
        .windows(n)
        .enumerate()
        .filter(|(_, x)| x.iter().enumerate().all(|(i, c)| !x[..i].contains(c)))
        .map(|(i, _)| i + n)
        .next()
        .unwrap_or(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_06() {
        let tests = [
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        for (s, w) in tests {
            println!("{}", s);
            assert_eq!(nproc_start(s, 4), w);
        }
    }
}
