use anyhow::Result;

pub fn run(input: &str, _: &crate::Options) -> Result<String> {
    println!("len: {}", input.trim().len());
    let p1 = tower_height(input, 2022);
    // 1566227410342 too low
    let p2 = tower_height(input, 1000000000000);
    Ok(format!("{} {}", p1, p2))
}

fn tower_height(input: &str, nrocks: usize) -> usize {
    let wind_len = Wind::iter(input).count();
    let mut s = Sim::from(Wind::iter(input).cycle());
    let rept = rocks().len() * wind_len;

    println!("{} {}", nrocks, rept);
    if rept > nrocks {
        s.step_n(nrocks);
        return s.height() as usize;
    }

    s.step_n(rept);
    let a_height = s.height() as usize;
    let a_rocks = rept;

    let mut irept = rept;

    let sig = s.sig();

    let mut mid = 0;
    loop {
        if nrocks <= irept + rept {
            s.step_n(nrocks - irept);
            return s.height() as usize;
        }

        s.step_n(rept);
        irept += rept;
        mid += 1;

        if s.sig() == sig {
            break;
        }
    }

    let cur = s.height() as usize;
    let b_height = cur - a_height;
    let b_rocks = mid * rept;

    let b_times = (nrocks - a_rocks) / b_rocks;
    let c_rocks = (nrocks - a_rocks) % b_rocks;

    s.step_n(c_rocks);
    let c_height = s.height() as usize - cur;
    println!("{} {}·{} {}", a_height, b_times, b_height, c_height);

    a_height + b_times * b_height + c_height
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day17_works() {
        let sample = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

        let mut sim = Sim::from(Wind::iter(sample).cycle());
        for i in 0..2022 {
            sim.step();
            println!(
                "\nRock {} (height={}):\n{}",
                i,
                sim.height,
                sim.sig().render_lines()
            );
        }
        assert_eq!(sim.height, 3068);

        assert_eq!(tower_height(sample, 2022), 3068);
        assert_eq!(tower_height(sample, 1000000000000), 1514285714288);
    }
}

const SIM_ROWS: usize = 64;
const SIG_ROWS: usize = 16;

#[derive(Eq, PartialEq, Copy, Clone)]
struct Sig([u8; SIG_ROWS]);

impl Sig {
    #[cfg(test)]
    fn render_lines(&self) -> String {
        let mut s = String::new();
        s.reserve(self.0.len() * 10);
        for row in self.0.iter().rev() {
            s.push('│');
            let mut bit = 0x40;
            while bit != 0 {
                s.push(if (row & bit) != 0 { '#' } else { '.' });
                bit >>= 1;
            }
            s.push_str("│\n");
        }
        s
    }
}

struct Sim<IT> {
    rocks: Vec<Rock>, // rock shapes, bottom up

    bits: [u8; SIM_ROWS], // top down

    irock: usize,
    height: usize,
    wind_iter: IT,
}

impl<IT: Iterator<Item = Wind>> Sim<IT> {
    fn from(wind_iter: IT) -> Sim<IT> {
        let mut bits = [0; SIM_ROWS];
        bits[SIM_ROWS - 1] = 0x7f;
        Sim {
            rocks: rocks(),
            bits,
            irock: 0,
            height: 0,
            wind_iter,
        }
    }

    fn height(&self) -> usize {
        self.height
    }

    fn step_n(&mut self, n_rocks: usize) {
        for _ in 0..n_rocks {
            self.step();
        }
    }

    fn step(&mut self) {
        let i = self.irock % self.rocks.len();
        self.irock += 1;

        let rock = self.rocks[i];
        let mut falling = rock.shape;

        let mut blow = |rock| self.wind_iter.next().unwrap().shift(rock);

        // Blow falling rock four times before it can interfere with rocks at rest.
        for _ in 0..4 {
            falling = blow(falling);
        }

        let mut y_acc = self.height % SIM_ROWS;
        let mut window_acc = 0_u32;

        let mut next_window = || {
            let y = y_acc;
            y_acc = (y_acc + SIM_ROWS - 1) % SIM_ROWS;
            window_acc = window_acc << 8 | (self.bits[y_acc] as u32);
            (y, window_acc)
        };

        // Check if the falling rock grows the tower.
        for grow in 0..rock.nrows {
            let (y, window) = next_window();
            //println!("{}", rock_window_str(falling, window));
            if window & falling != 0 {
                self.grow(rock.nrows - grow);
                self.add_rock(y, falling, rock.nrows);
                return;
            }
            let next = blow(falling);
            if window & next == 0 {
                falling = next;
            }
        }

        // Find falling rock position inside the existing tower.
        for (y, window) in (rock.nrows..SIM_ROWS).map(|_| next_window()) {
            //println!("{}", rock_window_str(falling, window));
            if window & falling != 0 {
                self.add_rock(y, falling, rock.nrows);
                return;
            }
            let next = blow(falling);
            if window & next == 0 {
                falling = next;
            }
        }

        panic!("SIM_ROWS too small for rock {}", self.irock);
    }

    fn sig(&self) -> Sig {
        let mut s = [0; SIG_ROWS];
        let p = self.height.wrapping_sub(SIG_ROWS) % SIM_ROWS;
        let q = self.height % SIM_ROWS;
        if p < q {
            s.copy_from_slice(&self.bits[p..q]);
        } else {
            let ps = &self.bits[p..];
            let qs = &self.bits[..q];
            s[..ps.len()].copy_from_slice(ps);
            s[ps.len()..].copy_from_slice(qs);
        }
        Sig(s)
    }

    fn grow(&mut self, nrows: usize) {
        let mut y = self.height % SIM_ROWS;
        for _ in 0..nrows {
            self.bits[y] = 0;
            y = (y + 1) % SIM_ROWS;
        }
        self.height += nrows;
    }

    fn add_rock(&mut self, y: usize, rock: u32, nrows: usize) {
        let mut rock = rock;
        let mut y = y;
        for _ in 0..nrows {
            let r = (rock & 0xFF) as u8;
            assert_eq!(self.bits[y] & r, 0);
            self.bits[y] |= r;
            rock >>= 8;
            y = (y + 1) % SIM_ROWS;
        }
    }
}

#[allow(dead_code)]
fn rock_window_str(rock: u32, win: u32) -> String {
    let mut s = String::new();
    for i in (0..4).rev() {
        let r = ((rock >> (i * 8)) & 0xFF) as u8;
        let w = ((win >> (i * 8)) & 0xFF) as u8;
        let x = r & w;
        s.push('│');
        for m in (0..7).rev().map(|i| 1_u8 << i) {
            s.push(if x & m != 0 {
                '@'
            } else if r & m != 0 {
                '#'
            } else {
                '.'
            });
        }

        s.push_str("│  │");
        for m in (0..7).rev().map(|i| 1_u8 << i) {
            s.push(if x & m != 0 {
                '@'
            } else if w & m != 0 {
                '#'
            } else {
                '.'
            });
        }
        s.push_str("│\n");
    }
    s
}

fn rocks() -> Vec<Rock> {
    vec![
        Rock::from("####"),        // -
        Rock::from(".#. ### .#."), // +
        Rock::from("..# ..# ###"), // ┘
        Rock::from("# # # #"),     // |
        Rock::from("## ##"),       // ■
    ]
}

#[derive(Debug, Clone, Copy)]
struct Rock {
    shape: u32, // shape in bytes, lowest byte is at bottom
    nrows: usize,
}

impl Rock {
    fn from(s: &str) -> Rock {
        let (shape, nrows) = s
            .split(' ')
            .map(|line| {
                line.chars()
                    .enumerate()
                    .filter_map(|(i, c)| (c == '#').then_some(1_u8 << (4 - i)))
                    .fold(0, |acc, m| acc | m)
            })
            .fold((0_u32, 0_usize), |(shape, nrows), m| {
                (shape << 8 | m as u32, nrows + 1)
            });
        Rock { shape, nrows }
    }
}

#[derive(Debug, Clone, Copy)]
enum Wind {
    Left,
    Right,
}

const LEFT_MASK: u32 = 0x40404040; // mask of falling rock at the left wall
const RGHT_MASK: u32 = 0x01010101; // mask of falling rock at the right wall
                                   //
impl Wind {
    fn iter(src: &str) -> impl Iterator<Item = Wind> + Clone + '_ {
        src.trim().chars().filter_map(|c| match c {
            '<' => Some(Wind::Left),
            '>' => Some(Wind::Right),
            _ => None,
        })
    }

    fn shift(&self, rock: u32) -> u32 {
        match self {
            Wind::Left => {
                if rock & LEFT_MASK == 0 {
                    return rock << 1;
                }
            }
            Wind::Right => {
                if rock & RGHT_MASK == 0 {
                    return rock >> 1;
                }
            }
        }
        rock
    }
}
