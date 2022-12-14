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
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

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
        (day09::run as DayFunc),
        (day10::run as DayFunc),
        (day11::run as DayFunc),
        (day12::run as DayFunc),
        (day13::run as DayFunc),
        (day14::run as DayFunc),
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
            Ok(result) => println!("{}", result),
            Err(e) => eprintln!("{}", e),
        }
    }

    Ok(())
}

pub struct Options {
    verbose: bool,
}

type DayFunc = fn(&str, &Options) -> Result<String>;

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
