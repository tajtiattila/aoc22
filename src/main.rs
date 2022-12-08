use anyhow::Result;
use clap::Parser;
use std::collections::HashSet;

const AOC_YEAR: u32 = 22;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

fn day_funcs() -> Vec<DayFunc> {
    vec![
        (day01::run as DayFunc),
        (day02::run as DayFunc),
        (day03::run as DayFunc),
        (day04::run as DayFunc),
        (day05::run as DayFunc),
        (day06::run as DayFunc),
        (day07::run as DayFunc),
        (day08::run as DayFunc),
    ]
}

mod util;
use util::InputSource;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,

    #[arg(short, long)]
    all: bool,

    days: Vec<usize>,
}

fn main() -> anyhow::Result<()> {
    let is = InputSource::new()?;

    let cli = Cli::parse();
    let options = Options {
        verbose: cli.verbose,
    };
    let dfs = get_day_funcs(&cli);

    for (i, f) in dfs {
        let r = is.get(i).and_then(|s| f(&s, &options));
        print!("Day {:?}: ", i);
        match r {
            Ok((x, y)) => println!("{} {}", x, y),
            Err(e) => println!("{}", e),
        }
    }

    Ok(())
}

pub struct Options {
    verbose: bool,
}

type DayResult = Result<(String, String)>;
type DayFunc = fn(&str, &Options) -> DayResult;

fn get_day_funcs(cli: &Cli) -> Vec<(usize, DayFunc)> {
    let v: Vec<(usize, DayFunc)> = day_funcs()
        .into_iter()
        .enumerate()
        .map(|(n, f)| (n + 1, f))
        .collect();
    if !cli.days.is_empty() {
        let s: HashSet<_> = cli.days.iter().collect();
        v.into_iter().filter(|(x, _)| s.contains(&x)).collect()
    } else if cli.all {
        v
    } else {
        vec![*v.last().unwrap()]
    }
}

fn day_ok<T, U>(t: T, u: U) -> DayResult
where
    T: std::fmt::Display,
    U: std::fmt::Display,
{
    Ok((t.to_string(), u.to_string()))
}
