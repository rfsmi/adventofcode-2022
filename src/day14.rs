use std::collections::BTreeMap;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum Line {
    Vertical(isize, (isize, isize)),
    Horizontal((isize, isize), isize),
}

impl Line {
    fn new((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> Self {
        if x1 == x2 {
            Line::Vertical(x1, (y1.min(y2), y1.max(y2)))
        } else if y1 == y2 {
            Line::Horizontal((x1.min(x2), x1.max(x2)), y1)
        } else {
            panic!("Expected a horizontal or vertical line");
        }
    }
}

enum CellType {
    Sand,
    Wall,
}

struct Cells {
    occupied_cells: BTreeMap<(isize, isize), CellType>,
    min_bound: Option<(isize, isize)>,
    max_bound: Option<(isize, isize)>,
}

impl Cells {
    fn new() -> Self {
        Self {
            occupied_cells: BTreeMap::new(),
            min_bound: None,
            max_bound: None,
        }
    }

    fn add_line(&mut self, line: Line) {
        let points = match line {
            Line::Horizontal((x1, x2), y) => (x1..=x2).map(|x| ((x, y))).collect_vec(),
            Line::Vertical(x, (y1, y2)) => (y1..=y2).map(|y| (x, y)).collect_vec(),
        };
        for &point in &points {
            self.min_bound = Some(self.new_min_bound(point));
            self.max_bound = Some(self.new_max_bound(point));
            self.occupied_cells.insert(point, CellType::Wall);
        }
    }

    fn new_min_bound(&self, (x, y): (isize, isize)) -> (isize, isize) {
        match self.min_bound {
            Some((min_x, min_y)) => (min_x.min(x), min_y.min(y)),
            None => (x, 0),
        }
    }

    fn new_max_bound(&self, (x, y): (isize, isize)) -> (isize, isize) {
        match self.max_bound {
            Some((max_x, max_y)) => (max_x.max(x), max_y.max(y)),
            None => (x, y),
        }
    }

    fn add_sand(&mut self, mut point: (isize, isize)) -> bool {
        if self.occupied_cells.contains_key(&point) {
            return false;
        }
        loop {
            match self.min_bound {
                Some(min) if self.new_min_bound(point) == min => (),
                _ => return false,
            }
            match self.max_bound {
                Some(max) if self.new_max_bound(point) == max => (),
                _ => return false,
            }
            let (x, y) = point;
            let next_point = [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)]
                .into_iter()
                .find(|p| !self.occupied_cells.contains_key(p));
            if let Some(p) = next_point {
                point = p;
            } else {
                // Sand comes to rest at `point`
                self.occupied_cells.insert(point, CellType::Sand);
                return true;
            }
        }
    }

    fn _to_string(&self) -> String {
        let mut result = String::new();
        let (min, max) = match (self.min_bound, self.max_bound) {
            (Some(min), Some(max)) => (min, max),
            _ => return result,
        };
        for y in min.1..=max.1 {
            for x in min.0..=max.0 {
                let c = match self.occupied_cells.get(&(x, y)) {
                    Some(CellType::Wall) => '#',
                    Some(CellType::Sand) => 'o',
                    None => '.',
                };
                result.push(c);
            }
            result.push('\n');
        }
        result
    }
}

fn parse(input: &str) -> impl Iterator<Item = Line> + '_ {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .flat_map(|l| {
            l.split(" -> ")
                .map(|p| {
                    p.split(",")
                        .map(|n| n.parse::<isize>().unwrap())
                        .collect_tuple::<(_, _)>()
                        .unwrap()
                })
                .tuple_windows()
                .map(|(l, r)| Line::new(l, r))
        })
}

pub(crate) fn solve(input: &str) -> usize {
    let mut cells = parse(input).fold(Cells::new(), |mut cell, line| {
        cell.add_line(line);
        cell
    });
    for i in 0.. {
        if !cells.add_sand((500, 0)) {
            return i;
        }
    }
    panic!()
}

pub(crate) fn solve_2(input: &str) -> usize {
    let mut cells = parse(input).fold(Cells::new(), |mut cell, line| {
        cell.add_line(line);
        cell
    });
    let depth = cells.max_bound.unwrap().1 + 2;
    cells.add_line(Line::Horizontal((500 - depth, 500 + depth), depth));
    for i in 0.. {
        if !cells.add_sand((500, 0)) {
            return i;
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9
    ";

    #[test]
    fn test_parse() {
        let lines = parse(EXAMPLE).collect_vec();
        assert_eq!(
            lines,
            vec![
                Line::Vertical(498, (4, 6)),
                Line::Horizontal((496, 498), 6),
                Line::Horizontal((502, 503), 4),
                Line::Vertical(502, (4, 9)),
                Line::Horizontal((494, 502), 9),
            ]
        );
    }

    #[test]
    fn test_add_line() {
        let mut cells = Cells::new();
        cells.add_line(Line::Vertical(498, (4, 6)));
        cells.add_line(Line::Horizontal((496, 498), 6));
        assert_eq!(cells.occupied_cells.len(), 5);
        assert_eq!(cells.min_bound, Some((496, 4)));
        assert_eq!(cells.max_bound, Some((498, 6)));
        assert_eq!(
            cells.occupied_cells.keys().copied().collect_vec(),
            vec![(496, 6), (497, 6), (498, 4), (498, 5), (498, 6)]
        );
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(EXAMPLE), 24);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(EXAMPLE), 93);
    }

    #[test]
    fn test_solve_2_simple() {
        // ...o...
        // ..o#o..
        // .ooooo.
        // #######
        assert_eq!(solve_2("500,1 -> 500,1"), 8);
    }
}
