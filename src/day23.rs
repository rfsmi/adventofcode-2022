use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Write},
};

use itertools::Itertools;

struct BBox {
    top_left: Point,
    bottom_right: Point,
}

impl BBox {
    fn new(mut points: impl Iterator<Item = Point>) -> Self {
        let mut top_left = points.next().unwrap();
        let mut bottom_right = top_left;
        for point in points {
            top_left.0[0] = top_left.0[0].min(point.0[0]);
            top_left.0[1] = top_left.0[1].min(point.0[1]);
            bottom_right.0[0] = bottom_right.0[0].max(point.0[0]);
            bottom_right.0[1] = bottom_right.0[1].max(point.0[1]);
        }
        Self {
            top_left,
            bottom_right,
        }
    }

    fn width(&self) -> usize {
        (self.bottom_right.0[0] - self.top_left.0[0] + 1) as usize
    }

    fn height(&self) -> usize {
        (self.bottom_right.0[1] - self.top_left.0[1] + 1) as usize
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Point([isize; 2]);

impl Point {
    fn apply_vector(mut self, vector: [isize; 2]) -> Self {
        self.0[0] += vector[0];
        self.0[1] += vector[1];
        self
    }

    fn apply_direction(mut self, direction: Direction) -> Self {
        self.0[direction.dim_index] += direction.vector;
        self
    }
}

fn adjacent_vectors() -> impl Iterator<Item = [isize; 2]> {
    (0..2)
        .map(|_| [-1, 0, 1].into_iter())
        .multi_cartesian_product()
        .map(|v| v.try_into().unwrap())
        .filter(|&p| p != [0, 0])
}

#[derive(Clone, Copy)]
struct Direction {
    dim_index: usize,
    vector: isize,
}

impl Direction {
    fn points(&self, point: Point) -> impl Iterator<Item = Point> + '_ {
        adjacent_vectors()
            .filter(|p| p[self.dim_index] == self.vector)
            .map(move |p| point.apply_vector(p))
    }
}

struct Elves {
    positions: HashSet<Point>,
    directions: Vec<Direction>,
}

impl Display for Elves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bounds = self.bounds();
        for row in 0..bounds.height() as isize {
            for col in 0..bounds.width() as isize {
                let point = bounds.top_left.apply_vector([col, row]);
                f.write_char(match self.positions.contains(&point) {
                    true => '#',
                    false => '.',
                })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Elves {
    fn new(input: &str) -> Self {
        Self {
            positions: input
                .lines()
                .map(|l| l.trim())
                .filter(|l| !l.is_empty())
                .enumerate()
                .flat_map(|(y, l)| {
                    l.char_indices()
                        .filter(|&(_, c)| c == '#')
                        .map(move |(x, _)| Point([x as isize, y as isize]))
                })
                .collect(),
            directions: vec![
                Direction {
                    dim_index: 1,
                    vector: -1,
                },
                Direction {
                    dim_index: 1,
                    vector: 1,
                },
                Direction {
                    dim_index: 0,
                    vector: -1,
                },
                Direction {
                    dim_index: 0,
                    vector: 1,
                },
            ],
        }
    }

    fn round(&mut self) -> bool {
        // Which points have at least one other point adjacent to them?
        let mut will_propose = HashSet::new();
        for &point in &self.positions {
            if adjacent_vectors()
                .map(|v| point.apply_vector(v))
                .any(|p| self.positions.contains(&p))
            {
                will_propose.insert(point);
            }
        }

        // And where do these points propose to move to?
        let mut proposals: HashMap<Point, Point> = HashMap::new();
        let mut destination_counts: HashMap<Point, usize> = HashMap::new();
        for point in will_propose {
            for &direction in &self.directions {
                if direction.points(point).any(|p| self.positions.contains(&p)) {
                    continue;
                }
                let new_point = point.apply_direction(direction);
                proposals.insert(point, new_point);
                *destination_counts.entry(new_point).or_default() += 1;
                break;
            }
        }
        self.directions.rotate_left(1);

        // Find the non-conflicting proposals
        let mut good_proposals: HashMap<Point, Point> = HashMap::new();
        for (point, new_point) in proposals {
            if destination_counts[&new_point] == 1 {
                good_proposals.insert(point, new_point);
            }
        }

        // Move the points to their new positions
        let mut new_positions = HashSet::new();
        for &point in &self.positions {
            new_positions.insert(*good_proposals.get(&point).unwrap_or(&point));
        }
        self.positions = new_positions;
        !good_proposals.is_empty()
    }

    fn bounds(&self) -> BBox {
        BBox::new(self.positions.iter().copied())
    }
}

pub(crate) fn solve(input: &str) -> usize {
    let mut elves = Elves::new(input);
    for _ in 0..10 {
        elves.round();
    }
    let bounds = elves.bounds();
    bounds.width() * bounds.height() - elves.positions.len()
}

pub(crate) fn solve_2(input: &str) -> usize {
    let mut elves = Elves::new(input);
    for i in 1.. {
        if !elves.round() {
            return i;
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {

    use super::*;

    const EXAMPLE: &str = "
        ....#..
        ..###.#
        #...#.#
        .#...##
        #.###..
        ##.#.##
        .#..#..
    ";

    #[test]
    fn test_parse() {
        let elves = Elves::new(
            "
            .#.#
            #.#.
        ",
        );
        assert_eq!(elves.positions.len(), 4);
        assert!(elves.positions.contains(&Point([1, 0])));
        assert!(elves.positions.contains(&Point([3, 0])));
        assert!(elves.positions.contains(&Point([2, 1])));
        assert!(elves.positions.contains(&Point([0, 1])));
        assert_eq!(&elves.bounds().top_left, &Point([0, 0]));
        assert_eq!(&elves.bounds().bottom_right, &Point([3, 1]));
        assert_eq!(elves.bounds().width(), 4);
        assert_eq!(elves.bounds().height(), 2);
    }

    #[test]
    fn test_adjacent_vectors() {
        let mut vecs = adjacent_vectors().collect_vec();
        vecs.sort();
        assert_eq!(
            vecs,
            vec![
                [-1, -1],
                [-1, 0],
                [-1, 1],
                [0, -1],
                // [0, 0],
                [0, 1],
                [1, -1],
                [1, 0],
                [1, 1]
            ]
        )
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(EXAMPLE), 110);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(EXAMPLE), 20);
    }
}
