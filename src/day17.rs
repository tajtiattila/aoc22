use anyhow::Result;
use once_cell::sync::Lazy;

pub fn run(input: &str, _: &crate::Options) -> Result<String> {
    println!("len: {}", input.trim().len());
    let p1 = tower_height(input, 7, 2022);
    // 1566227410342 too low
    let p2 = tower_height(input, 7, 1000000000000);
    Ok(format!("{} {}", p1, p2))
}

fn tower_height(input: &str, width: i32, nrocks: usize) -> usize {
    let mut s = towersim(width, input);
    let rept = ROCKS.len() * input.trim().len();

    println!("{} {}", nrocks, rept);
    if rept > nrocks {
        s.step_n(nrocks);
        return s.height() as usize;
    }

    s.step_n(rept);
    let a_height = s.height() as usize;
    let a_rocks = rept;

    let mut irept = rept;

    let sig = s.chamber.signature();

    let mut mid = 0;
    loop {
        if nrocks <= irept + rept {
            s.step_n(nrocks - irept);
            return s.height() as usize;
        }

        s.step_n(rept);
        irept += rept;
        mid += 1;

        if s.chamber.signature() == sig {
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

struct TowerSim<IT: Iterator<Item = i32>> {
    chamber: Chamber,
    irock: usize,
    wind: IT,
}

fn towersim(width: i32, input: &str) -> TowerSim<impl Iterator<Item = i32> + '_> {
    TowerSim {
        chamber: Chamber::new(width),
        irock: 0,
        wind: input
            .trim()
            .chars()
            .cycle()
            .map(|c| if c == '<' { -1 } else { 1 }),
    }
}

impl<IT: Iterator<Item = i32>> TowerSim<IT> {
    fn height(&self) -> i32 {
        self.chamber.height()
    }

    fn step_n(&mut self, n: usize) {
        for _ in 0..n {
            let (p, i) = self.next_rock();
            let rocks = &ROCKS;
            self.chamber.freeze_rock(p, &rocks[i]);
        }
    }

    fn step_print_n(&mut self, n: usize, nmod: usize) {
        for _ in 0..n {
            let prt = self.irock % nmod == 0;
            let (p, i) = self.next_rock();
            let rocks = &ROCKS;
            let rock = &rocks[i];
            if prt {
                println!("\nRock {}, Sig: {:x}", self.irock, self.chamber.signature());
                self.chamber.print_with_rock(p, rock, 20);
            }
            self.chamber.freeze_rock(p, rock);
        }
    }

    fn next_rock(&mut self) -> ((i32, i32), usize) {
        let rocks = &ROCKS;
        let i = self.irock % rocks.len();
        self.irock += 1;

        let rock = &rocks[i];
        let (mut px, mut py) = self.chamber.rock_start(rock);
        loop {
            let nx = px + self.wind.next().unwrap();
            if self.chamber.is_free((nx, py), rock) {
                px = nx;
            }
            if self.chamber.is_free((px, py - 1), rock) {
                py -= 1;
            } else {
                return ((px, py), i);
            }
        }
    }
}

static ROCKS: Lazy<Vec<Rock>> = Lazy::new(rock_shapes);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day17_works() {
        let chamber = Chamber::new(7);
        let rock = &rock_shapes()[0];
        assert!(chamber.is_free((0, 0), rock));

        let sample = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

        towersim(7, sample).step_print_n(10000, 200);
        assert_eq!(tower_height(sample, 7, 2022), 3068);
        assert_eq!(tower_height(sample, 7, 1000000000000), 1514285714288);
    }
}

#[derive(Debug, Clone)]
struct Chamber {
    dx: i32,
    v: Vec<u8>,
}

impl Chamber {
    fn new(dx: i32) -> Chamber {
        Chamber { dx, v: vec![] }
    }

    fn height(&self) -> i32 {
        (self.v.len() as i32) / self.dx
    }

    fn rock_start(&self, _rock: &Rock) -> (i32, i32) {
        (2, self.height() + 3)
    }

    fn is_free(&self, p: (i32, i32), rock: &Rock) -> bool {
        let (px, py) = p;
        if px < 0 || self.dx < px + rock.dx || py < 0 {
            return false;
        }
        rock.bits(p).all(|p| self.is_space(p))
    }

    fn freeze_rock(&mut self, p: (i32, i32), rock: &Rock) {
        self.freeze_rock_impl(p, rock, b'#')
    }

    fn freeze_rock_impl(&mut self, p: (i32, i32), rock: &Rock, ch: u8) {
        rock.bits(p).for_each(|p| self.set_rock(p, ch));
    }

    fn is_space(&self, p: (i32, i32)) -> bool {
        let (px, py) = p;
        if px < 0 || self.dx <= px || py < 0 {
            return false;
        }
        let row = (py * self.dx) as usize;
        if self.v.len() <= row {
            return true;
        }
        self.v[row + px as usize] == b'.'
    }

    fn set_rock(&mut self, p: (i32, i32), ch: u8) {
        let (px, py) = p;
        if px < 0 || self.dx <= px || py < 0 {
            panic!("invalid rock position {} {}", px, py);
        }
        let row = (py * self.dx) as usize;
        let need_len = row + self.dx as usize;
        if self.v.len() < need_len {
            self.v.resize(need_len, b'.');
        }
        self.v[row + px as usize] = ch
    }

    fn print_with_rock(&self, p: (i32, i32), rock: &Rock, nlines: usize) {
        let mut x = self.clone();
        x.freeze_rock_impl(p, rock, b'@');
        for (i, line) in x.v.chunks(x.dx as usize).enumerate().rev().take(nlines) {
            println!("{:6}│{}│", i, std::str::from_utf8(line).unwrap());
        }
        println!("{:6}└{:─<width$}┘", "", "", width = self.dx as usize);
    }

    fn signature(&self) -> u128 {
        self.v
            .iter()
            .rev()
            .take(128)
            .fold(0_u128, |acc, &c| acc << 1 | u128::from(c != b'.'))
    }
}

struct Rock {
    dx: i32,
    dy: i32,
    bits: u32,
    mask0: u32,
}

impl Rock {
    fn new(dx: i32, dy: i32, s: &str) -> Rock {
        let mut bits = 0;
        let mut n: i32 = 0;
        for c in s.chars() {
            if c == '#' || c == '.' {
                bits = (bits << 1) | u32::from(c == '#');
                n += 1;
            }
        }
        if n != dx * dy {
            panic!("invalid shape");
        }
        Rock {
            dx,
            dy,
            bits,
            mask0: 1 << n,
        }
    }

    fn bits(&self, at: (i32, i32)) -> impl Iterator<Item = (i32, i32)> + '_ {
        let mut mask = self.mask0;
        (0..self.dy)
            .flat_map(|y| (0..self.dx).map(move |x| (x, y)))
            .filter_map(move |(x, y)| {
                mask >>= 1;
                (self.bits & mask != 0).then_some((at.0 + x, at.1 + y))
            })
    }
}

fn rock_shapes() -> Vec<Rock> {
    vec![
        Rock::new(4, 1, "####"),
        Rock::new(3, 3, ".#. ### .#."),
        Rock::new(3, 3, "### ..# ..#"), // note: upside down
        Rock::new(1, 4, "# # # #"),
        Rock::new(2, 2, "## ##"),
    ]
}
