use crate::Options;
use anyhow::{anyhow, bail};
use std::collections::HashMap;

pub fn run(input: &str, o: &Options) -> anyhow::Result<String> {
    let t = tree(input).ok_or_else(|| anyhow!("invalid tree"))?;
    let sizes = dir_sizes(&t);
    if sizes.is_empty() {
        bail!("sizes empty");
    }

    const LIM: usize = 100000;
    let p1: usize = sizes.iter().filter(|&&x| x <= LIM).sum();

    const CAP: usize = 70000000;
    const NEED: usize = 30000000;
    let free = CAP - *sizes.last().unwrap();
    if o.verbose {
        println!("free space: {}/{}", free, CAP);
    }
    let p2 = sizes.iter().filter(|&&x| free + x >= NEED).min().unwrap();

    Ok(format!("{} {}", p1, p2))
}

fn dir_sizes(tree: &Vec<Tree>) -> Vec<usize> {
    let mut r = Vec::new();
    collect_dir_sizes(&mut r, tree);
    r
}

fn collect_dir_sizes(m: &mut Vec<usize>, tree: &Vec<Tree>) -> usize {
    let mut total = 0;
    for te in tree {
        match te {
            Tree::File(size) => {
                total += size;
            }
            Tree::Dir(t) => {
                total += collect_dir_sizes(m, t);
            }
        }
    }
    m.push(total);
    total
}

#[derive(Debug)]
enum Tree {
    File(usize),
    Dir(Vec<Tree>),
}

fn tree(input: &str) -> Option<Vec<Tree>> {
    let mut path = String::from("/");
    let mut tree = DirMap::new();

    for ll in proc(input) {
        match ll {
            Log::Cd(dir) => {
                if dir == ".." {
                    let p = path.rfind('/').unwrap();
                    if p > 1 {
                        path.truncate(p);
                    } else {
                        path.truncate(1); // at root
                    }
                } else if dir == "/" {
                    path.truncate(1); // at root
                } else {
                    if path != "/" {
                        path.push('/'); // non-root
                    }
                    path.push_str(&dir);
                }
            }
            Log::Ls(ents) => {
                tree.insert(path.clone(), ents);
            }
        };
    }

    tree_impl(&tree, "/")
}

type DirMap = HashMap<String, Vec<DirEnt>>;

fn tree_impl(m: &DirMap, path: &str) -> Option<Vec<Tree>> {
    Some(
        m.get(path)?
            .iter()
            .filter_map(|de| match de.stat {
                Stat::Dir => Some(Tree::Dir(tree_impl(m, &path_append(path, &de.name))?)),
                Stat::File(siz) => Some(Tree::File(siz)),
            })
            .collect(),
    )
}

fn proc(input: &str) -> Vec<Log> {
    let mut log = Vec::new();

    for line in input.lines() {
        if let Some(cd) = line.strip_prefix("$ cd ") {
            // $ cd «path»
            log.push(Log::Cd(String::from(cd)));
        } else if let Some((pfx, name)) = line.split_once(' ') {
            let mut addl = |stat, name| {
                let de = DirEnt {
                    stat,
                    name: String::from(name),
                };
                match log.last_mut().unwrap() {
                    Log::Ls(list) => {
                        list.push(de);
                    }
                    _ => {
                        log.push(Log::Ls(vec![de]));
                    }
                }
            };
            if pfx == "dir" {
                addl(Stat::Dir, name);
            } else if let Ok(siz) = pfx.parse() {
                addl(Stat::File(siz), name);
            }
        }
    }

    log
}

#[derive(Debug)]
enum Log {
    Cd(String),      // cd to folder
    Ls(Vec<DirEnt>), // ls output
}

#[derive(Debug)]
enum Stat {
    Dir,
    File(usize),
}

#[derive(Debug)]
struct DirEnt {
    stat: Stat,
    name: String,
}

fn path_append(base: &str, path: &str) -> String {
    let mut r = String::from(base);
    if r != "/" {
        r.push('/'); // non-root
    }
    r.push_str(path);
    r
}
