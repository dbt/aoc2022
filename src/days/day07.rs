use anyhow::anyhow;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

use crate::interface::*;

struct Dirs {
    dirs: HashMap<String, usize>,
}

fn join(s1: &String, s2: &String) -> String {
    let sep = if s1.ends_with("/") { "" } else { "/" };
    vec![s1.to_string(), s2.to_string()].join(sep)
}

impl Dirs {
    fn new(input: &Vec<String>) -> Result<Dirs> {
        let mut last_cmd = String::new();
        let mut paths: Vec<String> = Vec::new();
        let mut map: HashMap<String, usize> = HashMap::new();

        lazy_static! {
            static ref CMD_RE: Regex = Regex::new(r"^\$ (?P<cmd>[a-z]+)( (?P<arg>.*))?$").unwrap();
            static ref LS_RE: Regex =
                Regex::new(r"^((?P<dir>dir)|(?P<size>[0-9]+)) (?P<name>.*)$").unwrap();
            static ref EMPTY_S: String = String::new();
        }
        for s in input {
            if let Some(cmd) = CMD_RE.captures(&s) {
                last_cmd = cmd["cmd"].to_string();
                if last_cmd == "cd" {
                    let arg = &cmd["arg"];
                    if arg == ".." {
                        paths.pop();
                    } else {
                        if arg == "/" {
                            paths.clear();
                            paths.push("/".to_string());
                        } else if arg.contains("/") {
                            Err(anyhow!("Invalid {} arg '{}'", last_cmd, arg))?;
                        } else {
                            let path = join(paths.last().unwrap(), &arg.to_string());
                            paths.push(path);
                        }
                    }
                }
            } else if let Some(dir) = LS_RE.captures(&s) {
                if last_cmd != "ls" {
                    unimplemented!();
                }
                if dir.name("dir").is_some() {
                    continue;
                }
                let size: usize = dir["size"].parse().unwrap();
                for p in &paths[..] {
                    if p != "" {
                        let e = map.entry(p.to_string()).or_insert(0);
                        *e += size;
                    }
                }
            } else {
                eprintln!("Unknown input: {}", s);
                unimplemented!();
            }
        }

        Ok(Dirs { dirs: map })
    }
    fn small_size(&self) -> usize {
        self.dirs.values().filter(|v| **v <= 100000).sum()
    }
    fn find_min_delete(&self, target: usize) -> usize {
        let total = 70000000;
        let current = self.dirs["/"];
        let required = target - (total - current);
        self.dirs.values().fold(total, |a, e| {
            let v = *e;
            if v < required {
                // println!("Too small! {}", v);
                a
            } else if a < v {
                // println!("keeping previous better value {} over {}", a, v);
                a
            } else {
                // println!("Very nice! {}", v);
                v
            }
        })
    }
}

pub struct D {}

impl Day for D {
    fn number(&self) -> u8 {
        7
    }

    fn part01(&self) -> Result<()> {
        let dir = Dirs::new(&self.input()?)?;
        println!("{}", dir.small_size());

        Ok(())
    }
    fn part02(&self) -> Result<()> {
        let dir = Dirs::new(&self.input()?)?;
        println!("{}", dir.find_min_delete(30000000));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_vals() -> Vec<String> {
        r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
            .split("\n")
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn test_p1() {
        let dir: Dirs = Dirs::new(&test_vals()).unwrap();
        assert_eq!(dir.dirs.len(), 4);
        assert_eq!(dir.small_size(), 95437);
    }
    #[test]
    fn test_p2() {
        let dir: Dirs = Dirs::new(&test_vals()).unwrap();
        assert_eq!(dir.dirs.len(), 4);
        assert_eq!(dir.find_min_delete(30000000), 24933642);
    }
}
