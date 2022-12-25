use std::{
    collections::{BinaryHeap, HashSet},
    hash::Hash,
};

use itertools::Itertools;

#[derive(Clone, Copy)]
struct WindTracker {
    l_bits: u128,
    r_bits: u128,
    length: usize,
}

impl WindTracker {
    fn new(length: usize) -> Self {
        assert!(length < 128);
        Self {
            l_bits: 0,
            r_bits: 0,
            length,
        }
    }

    fn set_rightward(&mut self, pos: usize) {
        self.l_bits |= 1 << pos;
    }

    fn set_leftward(&mut self, pos: usize) {
        self.r_bits |= 1 << pos;
    }

    fn is_clear(&self, time: usize, pos: usize) -> bool {
        let time = time % self.length;
        let m0 = (1 << (self.length - time)) - 1;
        let m1 = (1 << time) - 1;
        let bits = ((self.l_bits & m0) << time)
            | ((self.l_bits >> (self.length - time)) & m1)
            | ((self.r_bits >> time) & m0)
            | ((self.r_bits & m1) << (self.length - time));
        bits & (1 << pos) == 0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    time: usize,
    pos: (i8, i8),
}

struct Board {
    ver_winds: Vec<WindTracker>,
    hor_winds: Vec<WindTracker>,
    start_pos: (i8, i8),
    end_pos: (i8, i8),
}

impl Board {
    fn new(input: &str) -> Self {
        let mut lines = input
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().filter(|&c| c != '#').collect_vec())
            .collect_vec();
        lines.remove(0);
        lines.remove(lines.len() - 1);

        let width = lines[0].len();
        let height = lines.len();
        let mut ver_winds = vec![WindTracker::new(height); width];
        let mut hor_winds = vec![WindTracker::new(width); height];
        for (y, row) in lines.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                match c {
                    '^' => ver_winds[x].set_leftward(y),
                    'v' => ver_winds[x].set_rightward(y),
                    '<' => hor_winds[y].set_leftward(x),
                    '>' => hor_winds[y].set_rightward(x),
                    _ => (),
                }
            }
        }

        Self {
            start_pos: (0, -1),
            end_pos: (width as i8 - 1, height as i8),
            ver_winds,
            hor_winds,
        }
    }

    fn next_states(&self, state: State) -> impl Iterator<Item = State> + '_ {
        let width = self.ver_winds.len() as i8;
        let height = self.hor_winds.len() as i8;
        let valid_state = move |&State { time, pos: (x, y) }: &State| {
            if (x, y) == self.start_pos || (x, y) == self.end_pos {
                return true;
            }
            if x < 0 || y < 0 || x >= width || y >= height {
                return false;
            }
            let (x, y) = (x as usize, y as usize);
            self.hor_winds[y].is_clear(time, x) && self.ver_winds[x].is_clear(time, y)
        };
        [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)]
            .into_iter()
            .map(move |(x_offset, y_offset)| State {
                pos: (state.pos.0 + x_offset, state.pos.1 + y_offset),
                time: state.time + 1,
            })
            .filter(valid_state)
    }

    fn fastest_path(&self, pos: (i8, i8), end: (i8, i8), time: usize) -> usize {
        let wrap_cost = |s: State| {
            let dist_to_goal = s.pos.0.abs_diff(end.0) + s.pos.1.abs_diff(end.1);
            let best_case_cost = s.time as isize + dist_to_goal as isize;
            (-best_case_cost, s)
        };
        let mut queue: BinaryHeap<_> = [wrap_cost(State { time, pos })].into();
        let mut seen = HashSet::new();
        while let Some((_, state)) = queue.pop() {
            if !seen.insert(state) {
                continue;
            }
            if state.pos == end {
                return state.time;
            }
            queue.extend(self.next_states(state).map(wrap_cost));
        }
        panic!();
    }

    fn solve<const LENGTHS: usize>(&self) -> usize {
        [self.start_pos, self.end_pos]
            .into_iter()
            .cycle()
            .take(LENGTHS + 1)
            .tuple_windows()
            .fold(0, |time, (pos, end)| self.fastest_path(pos, end, time))
    }
}

pub(crate) fn solve(input: &str) -> usize {
    Board::new(input).solve::<1>()
}

pub(crate) fn solve_2(input: &str) -> usize {
    Board::new(input).solve::<3>()
}

#[cfg(test)]
mod tests {

    use super::*;

    const EXAMPLE: &str = "
        #.######
        #>>.<^<#
        #.<..<<#
        #>v.><>#
        #<^v^^>#
        ######.#
    ";

    #[test]
    fn test_solve() {
        assert_eq!(solve(EXAMPLE), 18);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(EXAMPLE), 54);
    }
}
