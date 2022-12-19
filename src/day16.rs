use std::collections::{HashMap, HashSet, VecDeque};

use regex::Regex;

struct Edge {
    cost: u8,
    to_node: u8,
}

struct Node {
    rate: u8,
    edges: Vec<Edge>,
}

struct Graph {
    nodes: Vec<Node>,
    initial_node: u8,
}

impl Graph {
    fn new<'a>(valves: impl Iterator<Item = Valve<'a>>) -> Self {
        let mut nodes = Vec::new();
        let mut name_id_map = HashMap::new();
        let mut named_valves = HashMap::new();
        for valve in valves {
            if valve.rate > 0 || valve.name == "AA" {
                name_id_map.insert(valve.name, nodes.len());
                nodes.push(Node {
                    rate: valve.rate,
                    edges: Vec::new(),
                })
            }
            named_valves.insert(valve.name, valve);
        }
        for (&root_name, &root_id) in &name_id_map {
            let mut queue = VecDeque::from([(0, root_name)]);
            let mut seen = HashSet::new();
            while let Some((distance, name)) = queue.pop_front() {
                if !seen.insert(name) {
                    continue;
                }
                for &child in &named_valves[name].connections {
                    queue.push_back((distance + 1, child));
                }
                let Some(&id) = name_id_map.get(name) else {
                    continue;
                };
                if id != root_id {
                    nodes[root_id].edges.push(Edge {
                        cost: distance + 1,
                        to_node: id as u8,
                    })
                }
            }
        }
        Self {
            nodes,
            initial_node: name_id_map["AA"] as u8,
        }
    }
}

trait BuildableMemo<Value> {
    fn new(graph: &Graph, budget: i8) -> Self;
}
trait Memo<Value: Clone> {
    fn get(&self, key: &SolveState) -> Option<Value>;
    fn insert(&mut self, key: SolveState, value: Value);
}

impl<Value> BuildableMemo<Value> for HashMap<SolveState, Value> {
    fn new(_graph: &Graph, _budget: i8) -> Self {
        Self::new()
    }
}

impl<Value: Clone> Memo<Value> for HashMap<SolveState, Value> {
    fn get(&self, key: &SolveState) -> Option<Value> {
        HashMap::get(self, key).cloned()
    }
    fn insert(&mut self, key: SolveState, value: Value) {
        HashMap::insert(self, key, value);
    }
}

struct VecCache<T> {
    data: Vec<Vec<Vec<Option<T>>>>,
}

impl<Value: Clone> BuildableMemo<Value> for VecCache<Value> {
    fn new(graph: &Graph, budget: i8) -> Self {
        let n = graph.nodes.len();
        Self {
            data: vec![vec![vec![None; 1 + budget as usize]; n]; 2usize.pow(n as u32)],
        }
    }
}

impl<Value: Clone> Memo<Value> for VecCache<Value> {
    fn get(&self, key: &SolveState) -> Option<Value> {
        self.data[key.allowed as usize][key.node as usize][key.budget as usize].clone()
    }
    fn insert(&mut self, key: SolveState, value: Value) {
        self.data[key.allowed as usize][key.node as usize][key.budget as usize] = Some(value)
    }
}

#[derive(PartialEq, Eq, Hash)]
struct SolveState {
    node: u8,
    allowed: u32,
    budget: i8,
}

struct Solver<'a> {
    graph: &'a Graph,
    memo: Option<Box<dyn Memo<usize>>>,
}

impl<'a> Solver<'a> {
    fn new(graph: &'a Graph) -> Self {
        Self { memo: None, graph }
    }

    fn solve<T: BuildableMemo<usize> + Memo<usize> + 'static>(
        &mut self,
        num_actors: usize,
        budget: i8,
    ) -> usize {
        self.memo.replace(Box::new(T::new(self.graph, budget)));
        let initial_node = self.graph.initial_node;
        let initial_state = SolveState {
            node: initial_node,
            allowed: 1 << initial_node,
            budget,
        };

        let num_nodes = self.graph.nodes.len();
        let mut best_score = 0;
        let mut stack = vec![(0, vec![0; num_actors])];
        while let Some((node, actor_nodes)) = stack.pop() {
            if node == num_nodes {
                let mut score = 0;
                for nodes in actor_nodes {
                    let state = SolveState {
                        allowed: initial_state.allowed | nodes,
                        ..initial_state
                    };
                    score += self.recurse(state, 0);
                }
                best_score = best_score.max(score);
                continue;
            }
            for actor in 0..num_actors {
                let mut allowed_nodes = actor_nodes.clone();
                allowed_nodes[actor] |= 1 << node;
                stack.push((node + 1, allowed_nodes));
            }
        }
        best_score
    }

    fn recurse(&mut self, mut state: SolveState, mut score: usize) -> usize {
        if state.budget <= 1 {
            return score;
        }
        if state.allowed & (1 << state.node) == 0 {
            return score;
        }
        if let Some(result) = self.memo.as_ref().unwrap().get(&state) {
            return result;
        }

        state.allowed &= !(1 << state.node);
        let rate = self.graph.nodes[state.node as usize].rate;
        score += state.budget as usize * rate as usize;
        let mut best_score = score;
        for &Edge { to_node, cost } in &self.graph.nodes[state.node as usize].edges {
            let state = SolveState {
                node: to_node,
                budget: state.budget - cost as i8,
                ..state
            };
            best_score = self.recurse(state, score).max(best_score);
        }

        self.memo.as_mut().unwrap().insert(state, best_score);
        best_score
    }
}

struct Valve<'a> {
    name: &'a str,
    rate: u8,
    connections: Vec<&'a str>,
}

impl<'a> Valve<'a> {
    fn new(input: &'a str) -> Self {
        let re = Regex::new(r"^Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)$")
            .unwrap();
        let cap = re.captures(input).unwrap();
        Self {
            name: cap.get(1).unwrap().into(),
            rate: cap.get(2).unwrap().as_str().parse().unwrap(),
            connections: cap.get(3).unwrap().as_str().split(", ").collect(),
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
    let graph = Graph::new(parse(input));
    let mut solver = Solver::new(&graph);
    solver.solve::<HashMap<SolveState, usize>>(1, 30)
}

pub(crate) fn solve_2(input: &str) -> usize {
    let graph = Graph::new(parse(input));
    let mut solver = Solver::new(&graph);
    solver.solve::<VecCache<usize>>(2, 26)
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
    fn test_solve_2_vec() {
        let graph = Graph::new(parse(EXAMPLE));
        let mut solver = Solver::new(&graph);
        let result = solver.solve::<VecCache<usize>>(2, 26);
        assert_eq!(result, 1707)
    }

    #[test]
    fn test_solve_2_hashmap() {
        let graph = Graph::new(parse(EXAMPLE));
        let mut solver = Solver::new(&graph);
        let result = solver.solve::<HashMap<SolveState, usize>>(2, 26);
        assert_eq!(result, 1707)
    }
}
