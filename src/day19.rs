use std::{collections::HashMap, iter::zip};

use itertools::Itertools;
use regex::Regex;

struct Robot {
    costs: [u8; 3],
    _produces: usize,
}

struct Blueprint {
    robots: [Robot; 4],
    caps: [u8; 3],
}

fn parse(input: &str) -> impl Iterator<Item = Blueprint> + '_ {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let re = Regex::new(r"\d+").unwrap();
            let (_, ore_ore, clay_ore, obs_ore, obs_clay, geo_ore, geo_obs) = re
                .find_iter(l)
                .map(|m| m.as_str().parse().unwrap())
                .collect_tuple()
                .unwrap();
            Blueprint {
                robots: [
                    Robot {
                        costs: [ore_ore, 0, 0],
                        _produces: 0,
                    },
                    Robot {
                        costs: [clay_ore, 0, 0],
                        _produces: 1,
                    },
                    Robot {
                        costs: [obs_ore, obs_clay, 0],
                        _produces: 2,
                    },
                    Robot {
                        costs: [geo_ore, 0, geo_obs],
                        _produces: 3,
                    },
                ],
                caps: [
                    ore_ore.max(clay_ore).max(obs_ore).max(geo_ore),
                    obs_clay,
                    geo_obs,
                ],
            }
        })
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    robots: [u8; 4],
    resources: [u8; 4],
    budget: i8,
}

fn compute(minutes: i8, blueprint: Blueprint) -> usize {
    fn recurse(memo: &mut HashMap<State, usize>, blueprint: &Blueprint, state: State) -> usize {
        if state.budget == 0 {
            return state.resources[3] as usize;
        }
        if zip(state.robots, blueprint.caps).any(|(a, b)| a > b) {
            return state.resources[3] as usize;
        }
        if let Some(&result) = memo.get(&state) {
            return result;
        }
        let mut best_score = state.resources[3] as usize;
        for build_index in (0..state.robots.len()).rev() {
            let costs = &blueprint.robots[build_index].costs;
            let affordable = zip(&state.resources, costs).all(|(a, b)| a >= b);
            let mut state = state.clone();
            zip(&mut state.resources, state.robots).for_each(|(a, b)| *a += b);
            if affordable {
                zip(&mut state.resources, costs).for_each(|(a, b)| *a -= b);
                state.robots[build_index] += 1;
            }
            state.budget -= 1;
            best_score = best_score.max(recurse(memo, blueprint, state));
            if affordable && build_index == 3 {
                break;
            }
        }
        memo.insert(state, best_score);
        best_score
    }

    let initial_state = State {
        robots: [1, 0, 0, 0],
        resources: [0, 0, 0, 0],
        budget: minutes,
    };
    let mut memo = HashMap::new();
    recurse(&mut memo, &blueprint, initial_state)
}

pub(crate) fn solve(input: &str) -> usize {
    parse(input)
        .enumerate()
        .map(|(i, bp)| (i + 1) * compute(24, bp))
        .sum()
}

pub(crate) fn solve_2(input: &str) -> usize {
    parse(input).take(3).map(|bp| compute(32, bp)).product()
}

#[cfg(test)]
mod tests {

    use super::*;

    const EXAMPLE: &str = "
    Blueprint 1: \
        Each ore robot costs 4 ore. \
        Each clay robot costs 2 ore. \
        Each obsidian robot costs 3 ore and 14 clay. \
        Each geode robot costs 2 ore and 7 obsidian.
    ";

    #[test]
    fn test_parse() {
        let blueprint = parse(EXAMPLE).next().unwrap();
        assert_eq!(blueprint.robots[0].costs, [4, 0, 0]);
        assert_eq!(blueprint.robots[1].costs, [2, 0, 0]);
        assert_eq!(blueprint.robots[2].costs, [3, 14, 0]);
        assert_eq!(blueprint.robots[3].costs, [2, 0, 7]);
        assert_eq!(blueprint.robots[0]._produces, 0);
        assert_eq!(blueprint.robots[1]._produces, 1);
        assert_eq!(blueprint.robots[2]._produces, 2);
        assert_eq!(blueprint.robots[3]._produces, 3);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(EXAMPLE), 9);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(EXAMPLE), 56);
    }
}
