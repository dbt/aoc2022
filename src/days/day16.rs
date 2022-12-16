use std::{str::FromStr, collections::{HashMap, HashSet}};

use itertools::Itertools;

use adventools::prelude::*;
use anyhow::anyhow;
use lazy_static::lazy_static;
use regex::Regex;

pub struct D {}

impl DayParsed for D {}

impl Day for D {
    fn number(&self) -> u8 {
        16
    }
    fn part01(&self) -> Result<()> {
        println!("{}", ValveSet::new(self.input_as::<Valve>()?).best_path());
        Ok(())
    }
    fn part02(&self) -> Result<()> {
        println!("{}", ValveSet::new(self.input_as::<Valve>()?).best_path_with_help());
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Valve {
    id: String,
    flow_rate: usize,
    neighbors: Vec<String>,
}

impl FromStr for Valve {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
        }
        let caps = RE.captures(s).ok_or_else(|| anyhow!("Invalid input: '{}'", s))?;
        let id = caps[1].to_string();
        let flow_rate: usize = caps[2].parse()?;
        let neighbors: Vec<String> = caps[3].split(", ").map(|s| s.to_string()).collect();
        Ok(Valve { id, flow_rate, neighbors })
    }
}

struct ValveSet {
    valves: Vec<Valve>,
    mapped: HashMap<String, Valve>,
    distances: HashMap<String, usize>,
}

impl ValveSet {
    fn new(inputs: Vec<Valve>) -> Self {
        // println!("{:?}", inputs);
        let copied = inputs.clone();
        let mapped = map_of_valves(&copied);
        let distances = compute_distances(&mapped);

        ValveSet { valves: copied, mapped, distances }
    }
    fn dist(&self, pos: &String, other: &String) -> usize {
        self.distances[&format!("{}-{}", pos, other)]
    }

    fn best_path_with_help(&self) -> usize {
        let useful_valves: Vec<_> = self.valves.iter().filter_map(|v| if v.flow_rate > 0 { Some(v.id.to_string())} else { None }).collect();
        (1..(useful_valves.len()))
            .flat_map(|k| useful_valves.iter().combinations(k))
            .map(|mine| {
                // println!("{}", mine.len());
                let elephants: Vec<_> = useful_valves.iter().filter(|v| !mine.contains(v)).collect();
                self.best_path_dfs("AA".to_string(), 26, 0, 0, &mine) +
                    self.best_path_dfs("AA".to_string(), 26, 0, 0, &elephants)
            }).max().unwrap()
    }
    fn best_path(&self) -> usize {
        self.best_path_dfs("AA".to_string(), 30, 0, 0, &vec![])
    }
    fn worth_opening(&self, pos: &String, dest: &Valve, opened: &Vec<&String>, time_left: usize) -> bool {
        dest.flow_rate > 0 &&
            !opened.contains(&&dest.id) &&
            self.dist(pos, &dest.id) + 1 <= time_left
    }
    fn best_path_dfs(&self, pos: String, time_left: usize, cur_flow: usize, total_flow: usize, opened: &Vec<&String>) -> usize {
        // println!("at node {} at time {} with total_flow {} having opened {} valves: {:?}", pos, time_left, total_flow, opened.len(), opened);
        let v = &self.mapped[&pos];
        if v.flow_rate > 0 && !opened.contains(&&pos) {
            // open it
            let mut new_opened = opened.clone();
            new_opened.push(&pos);
            return self.best_path_dfs(pos.to_string(), time_left - 1, cur_flow + v.flow_rate, total_flow + cur_flow, &new_opened);
        }
        let worth_opening: Vec<_> = self.valves.iter().filter(|v| self.worth_opening(&pos, v, opened, time_left)).collect();
        // println!("worth opening: {:?}", worth_opening);
        if worth_opening.is_empty() {
            // maxed flow rate so just wait it out
            return total_flow + time_left * cur_flow;
        }
        worth_opening.iter().map(|&v| {
            let dist = self.dist(&pos, &v.id);
            self.best_path_dfs(v.id.to_string(), time_left - dist, cur_flow, total_flow + cur_flow * dist, &opened)
        }).max().unwrap()
    }
}

fn compute_distances(valves: &HashMap<String, Valve>) -> HashMap<String, usize> {
    let mut output: HashMap<String, usize> = HashMap::new();

    for (k, _v) in valves {
        let mut seen = HashSet::new();
        seen.insert(k);
        let mut dist = 0;
        let mut queue = vec![k];
        while seen.len() < valves.len() {
            dist += 1;
            let mut nextq = vec![];
            for &qe in queue.iter() {
                for ne in &valves[qe].neighbors {
                    if !seen.contains(ne) {
                        nextq.push(ne);
                        output.insert(format!("{}-{}", k, ne), dist);
                        seen.insert(ne);
                    }
                }
            }
            queue.clear();
            queue.append(&mut nextq);
        }
    }

    output
}

fn map_of_valves(v: &Vec<Valve>) -> HashMap<String, Valve> {
    v.iter().map(|va| (va.id.to_string(), va.clone())).collect()
}


#[cfg(test)]
mod tests {
    use super::*;
    fn test_data() -> Vec<Valve> {
        parse_lines::<Valve>(&split_str(r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II")).unwrap()
    }

    #[test]
    fn test_p1() {
        assert_eq!(ValveSet::new(test_data()).best_path(), 1651);
    }
    #[test]
    fn test_p2() {
        assert_eq!(ValveSet::new(test_data()).best_path_with_help(), 1707);
    }
}
