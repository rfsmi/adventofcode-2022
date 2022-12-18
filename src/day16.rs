use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap},
    iter::once,
};

use itertools::Itertools;
use regex::Regex;

#[derive(Clone, Copy)]
struct BitSet {
    field: u64,
}

impl BitSet {
    fn new() -> Self {
        Self { field: 0 }
    }

    fn insert(&mut self, index: u8) -> bool {
        let already_present = self.contains(index);
        self.field |= 1 << index as u64;
        !already_present
    }

    fn contains(&self, index: u8) -> bool {
        (self.field & (1 << index as u64)) != 0
    }
}

struct State<const N: usize> {
    score: isize,
    actor_states: [(u8, u8); N],
    visited_valves: BitSet,
    remaining_rate: isize,
}

impl<const N: usize> PartialEq for State<N> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<const N: usize> Eq for State<N> {}

impl<const N: usize> PartialOrd for State<N> {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ordering::Equal)
    }
}

impl<const N: usize> Ord for State<N> {
    fn cmp(&self, _other: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}

struct Graph {
    valves: Vec<Valve>,
    connection_costs: Vec<Vec<(u8, u8)>>,
    start_index: u8,
}

impl Graph {
    fn new(valves: &[Valve]) -> Self {
        let valves = valves
            .iter()
            .cloned()
            .sorted_by_key(|v| v.rate == 0)
            .collect_vec();
        let index_name_map: BTreeMap<_, _> = valves
            .iter()
            .map(|v| &v.name)
            .enumerate()
            .map(|(i, n)| (n.clone(), i))
            .collect();
        let connection_costs = valves
            .iter()
            .map(|v| Graph::bfs(&v.name, &valves, &index_name_map))
            .collect();
        Self {
            valves,
            connection_costs,
            start_index: index_name_map[&String::from("AA")] as u8,
        }
    }

    fn bfs(
        start: &String,
        valves: &[Valve],
        index_name_map: &BTreeMap<String, usize>,
    ) -> Vec<(u8, u8)> {
        let mut seen = BTreeSet::new();
        let mut stack = vec![(start, 0)];
        let mut result = Vec::new();
        while !stack.is_empty() {
            let (name, cost) = stack.remove(0);
            let index = index_name_map[name];
            if !seen.insert(index) {
                continue;
            }
            if valves[index].rate != 0 {
                result.push((index as u8, cost));
            }
            stack.extend(valves[index].connections.iter().map(|c| (c, cost + 1)));
        }
        result
    }

    fn connections<'b, const N: usize>(
        &'b self,
        state: &'b State<N>,
        actor_index: usize,
    ) -> impl Iterator<Item = (u8, u8)> + 'b {
        let (current_valve, remaining_steps) = state.actor_states[actor_index];
        self.connection_costs[current_valve as usize]
            .iter()
            .copied()
            .filter(move |(_, distance)| distance + 1 < remaining_steps)
            .filter(|(i, _)| !state.visited_valves.contains(*i))
            .map(move |(i, cost)| (i, remaining_steps - cost - 1))
            .chain(once((current_valve, 0)))
    }

    fn heuristic<const N: usize>(&self, state: &State<N>) -> isize {
        // The actor with the most remaining time visits everything instantly.
        let remaining_steps = state
            .actor_states
            .iter()
            .map(|(_, remaining_steps)| remaining_steps)
            .max()
            .unwrap();
        *remaining_steps as isize * state.remaining_rate
    }

    fn next_actor_states<const N: usize>(&self, state: &State<N>) -> Vec<[(u8, u8); N]> {
        let new_states = (0..N)
            .map(|actor_index| self.connections(state, actor_index).collect_vec())
            .collect_vec();
        let mut stack = vec![(0, state.actor_states.clone())];
        let mut next_actor_states = Vec::new();
        while let Some((actor_index, mut actor_states)) = stack.pop() {
            if actor_index == N {
                next_actor_states.push(actor_states);
                continue;
            }
            for new_state in &new_states[actor_index] {
                if actor_states[..actor_index]
                    .iter()
                    .any(|&(i, _)| i == new_state.0)
                {
                    continue;
                }
                actor_states[actor_index] = *new_state;
                stack.push((actor_index + 1, actor_states));
            }
        }
        next_actor_states
    }

    fn a_star<const N: usize>(&self, initial_time: u8) -> isize {
        let mut heap: BinaryHeap<_> = [(
            0,
            State {
                actor_states: [(self.start_index, initial_time); N],
                score: 0,
                visited_valves: BitSet::new(),
                remaining_rate: self.valves.iter().map(|v| v.rate).sum(),
            },
        )]
        .into_iter()
        .collect();

        while let Some((_, state)) = heap.pop() {
            if state
                .actor_states
                .iter()
                .all(|(_, remaining_steps)| *remaining_steps == 0)
            {
                return state.score;
            }
            // Try to go to each of the neighbours...
            for actor_states in self.next_actor_states(&state) {
                let mut visited_valves = state.visited_valves.clone();
                let mut score = state.score;
                let mut remaining_rate = state.remaining_rate;
                for &(i, remaining_steps) in actor_states.iter() {
                    if visited_valves.insert(i) {
                        score += remaining_steps as isize * self.valves[i as usize].rate;
                        remaining_rate -= self.valves[i as usize].rate;
                    }
                }
                let child = State {
                    visited_valves,
                    actor_states,
                    score,
                    remaining_rate,
                };
                let heuristic = self.heuristic(&child);
                heap.push((heuristic + child.score, child));
            }
        }
        panic!()
    }
}

#[derive(Clone)]
struct Valve {
    name: String,
    rate: isize,
    connections: Vec<String>,
}

impl Valve {
    fn new(input: &str) -> Self {
        let re = Regex::new(r"^Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)$")
            .unwrap();
        let cap = re.captures(input).unwrap();
        Self {
            name: cap[1].into(),
            rate: cap[2].parse::<isize>().unwrap(),
            connections: cap[3].split(", ").map(|s| s.to_string()).collect(),
        }
    }
}

fn parse(input: &str) -> impl Iterator<Item = Valve> + '_ {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(Valve::new)
}

pub(crate) fn solve(input: &str) -> usize {
    let valves = parse(input).collect_vec();
    let graph = Graph::new(&valves);
    graph.a_star::<1>(30) as usize
}

pub(crate) fn solve_2(input: &str) -> usize {
    let valves = parse(input).collect_vec();
    let graph = Graph::new(&valves);
    graph.a_star::<2>(26) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II
    ";

    #[test]
    fn test_parse() {
        let valve = parse(EXAMPLE).next().unwrap();
        assert_eq!(valve.name, "AA");
        assert_eq!(valve.rate, 0);
        assert_eq!(valve.connections, vec!["DD", "II", "BB"]);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(EXAMPLE), 1651);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(EXAMPLE), 1707);
    }
}
