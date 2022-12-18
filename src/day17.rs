use std::{
    collections::BTreeMap,
    iter::{once, zip},
};

use itertools::Itertools;

const WIDTH: i8 = 7;
const STARTING_COL: i8 = 2;

struct Board {
    rows: Vec<u8>,
}

impl Board {
    fn new() -> Self {
        Self { rows: Vec::new() }
    }

    fn height(&self) -> usize {
        self.rows.len()
    }

    fn impassable_ceiling(&self) -> Option<[u8; 4]> {
        if self.height() < 4 {
            return None;
        }
        let ceiling = &self.rows[self.height() - 4..];
        if ceiling.iter().fold(0, |or, r| or | r).count_ones() as i8 != WIDTH {
            return None;
        }
        Some(ceiling.try_into().unwrap())
    }

    fn intersects(&self, shape: &Shape, shape_bottom: isize) -> bool {
        if shape_bottom < 0 {
            return true;
        }
        let shape_bottom = shape_bottom as usize;
        if shape_bottom >= self.rows.len() {
            return false;
        }
        zip(&shape.rows, &self.rows[shape_bottom..]).any(|(s, b)| s & b != 0)
    }

    fn fix_shape(&mut self, shape: Shape, shape_bottom: isize) {
        if self.intersects(&shape, shape_bottom) {
            panic!("Shape intersects board");
        }
        let shape_bottom = shape_bottom as usize;
        if shape_bottom > self.height() {
            panic!("Shape is higher than board");
        }
        for (s, b) in zip(&shape.rows, &mut self.rows[shape_bottom..]) {
            *b |= s;
        }
        if shape_bottom + shape.rows.len() <= self.height() {
            return;
        }
        for &shape_row in &shape.rows[self.height() - shape_bottom..] {
            self.rows.push(shape_row);
        }
        while let Some(row) = self.rows.last() {
            if row.count_ones() > 0 {
                break;
            }
            self.rows.pop();
        }
    }

    fn play_single_iteration(
        &mut self,
        dirs: &mut impl Iterator<Item = (usize, Dir)>,
        shapes: &mut impl Iterator<Item = (usize, Shape)>,
    ) -> (usize, usize) {
        let (shape_index, mut shape) = shapes.next().unwrap();
        let mut shape_bottom = self.height() as isize + 3;
        let dir_index = loop {
            let (dir_index, dir) = dirs.next().unwrap();
            let mut shifted_shape = shape.clone();
            shifted_shape.shift(dir);
            if !self.intersects(&shifted_shape, shape_bottom) {
                shape = shifted_shape;
            }
            if self.intersects(&shape, shape_bottom - 1) {
                self.fix_shape(shape, shape_bottom);
                break dir_index;
            }
            shape_bottom -= 1;
        };
        (shape_index, dir_index)
    }

    fn to_string(&self, shape: &Shape, shape_bottom: isize) -> String {
        let shape_bottom = shape_bottom as usize;
        let shape_top = shape_bottom + shape.rows.len();
        let mut result = String::new();
        for height in (0..self.rows.len().max(shape_top)).rev().take(15) {
            let mut row = vec!['.'; WIDTH as usize];
            if height < self.rows.len() {
                let b = &self.rows[height];
                for i in 0..WIDTH {
                    if b & (1 << 7 - i) != 0 {
                        row[i as usize] = '#';
                    }
                }
            }
            if height >= shape_bottom && height < shape_top {
                let b = &shape.rows[height - shape_bottom];
                for i in 0..WIDTH {
                    if b & (1 << 7 - i) != 0 {
                        row[i as usize] = '@';
                    }
                }
            }
            result.extend(row.into_iter().chain(once('\n')));
        }
        result
    }
}

#[derive(Clone)]
struct Shape {
    rows: Vec<u8>,
    first_col: i8,
    last_col: i8,
}

impl Shape {
    fn new(cells: &[&[u8]]) -> Self {
        let rows = cells
            .iter()
            .rev()
            .map(|row| row.iter().fold(0, |accum, cell| (accum << 1) | cell))
            .collect_vec();
        let first_col = rows.iter().map(|row| row.leading_zeros()).min().unwrap() as i8;
        let last_col = 7 - rows.iter().map(|row| row.trailing_zeros()).min().unwrap() as i8;
        let mut result = Self {
            rows,
            first_col,
            last_col,
        };
        result.shift_impl(STARTING_COL - first_col);
        result
    }

    fn shift_impl(&mut self, amount: i8) {
        if self.first_col + amount < 0 || self.last_col + amount >= WIDTH {
            return;
        }
        for row in &mut self.rows {
            if amount < 0 {
                *row <<= -amount;
            } else {
                *row >>= amount;
            }
        }
        self.first_col += amount;
        self.last_col += amount;
    }

    fn shift(&mut self, dir: Dir) {
        match dir {
            Dir::Left => self.shift_impl(-1),
            Dir::Right => self.shift_impl(1),
        }
    }

    #[rustfmt::skip]
    fn spawn() -> impl Iterator<Item = (usize, Shape)> {
        [
            Shape::new(&[
                &[1, 1, 1, 1],
            ]),
            Shape::new(&[
                &[0, 1, 0],
                &[1, 1, 1],
                &[0, 1, 0],
            ]),
            Shape::new(&[
                &[0, 0, 1],
                &[0, 0, 1],
                &[1, 1, 1],
            ]),
            Shape::new(&[
                &[1],
                &[1],
                &[1],
                &[1],
            ]),
            Shape::new(&[
                &[1, 1],
                &[1, 1],
            ]),
        ]
        .into_iter()
        .enumerate()
        .cycle()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Left,
    Right,
}

fn parse(input: &str) -> impl Iterator<Item = (usize, Dir)> + '_ {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '<' => Dir::Left,
            '>' => Dir::Right,
            _ => panic!(),
        })
        .enumerate()
        .cycle()
}

struct Loop {
    starts: usize,
    length: usize,
    gains_height: usize,
}

fn find_loop(input: &str) -> Loop {
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    struct Key {
        dir_index: usize,
        shape_index: usize,
        ceiling: [u8; 4],
    }
    let mut dirs = parse(input);
    let mut shapes = Shape::spawn();
    let mut board = Board::new();
    let mut cache = BTreeMap::<Key, (usize, usize)>::new();
    for iteration in 0.. {
        let (shape_index, dir_index) = board.play_single_iteration(&mut dirs, &mut shapes);
        if let Some(ceiling) = board.impassable_ceiling() {
            let key = Key {
                dir_index,
                shape_index,
                ceiling,
            };
            if let Some(&(prev_iteration, prev_height)) = cache.get(&key) {
                return Loop {
                    starts: prev_iteration,
                    length: iteration - prev_iteration,
                    gains_height: board.height() - prev_height,
                };
            }
            cache.insert(key, (iteration, board.height()));
        }
    }
    panic!()
}

fn compute(input: &str, mut count: usize) -> usize {
    let l = find_loop(input);

    let mut dirs = parse(input);
    let mut shapes = Shape::spawn();
    let mut board = Board::new();

    for _ in 0..l.starts {
        board.play_single_iteration(&mut dirs, &mut shapes);
    }
    count -= l.starts;
    let looped_height = (count / l.length) * l.gains_height;
    count %= l.length;
    for _ in 0..count {
        board.play_single_iteration(&mut dirs, &mut shapes);
    }
    board.height() + looped_height
}

pub(crate) fn solve(input: &str) -> usize {
    compute(input, 2022)
}

pub(crate) fn solve_2(input: &str) -> usize {
    compute(input, 1000000000000)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    const EXAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_parse() {
        let dirs = parse("<><>>").take(7).collect_vec();
        assert_eq!(
            dirs,
            vec![
                (0, Dir::Left),
                (1, Dir::Right),
                (2, Dir::Left),
                (3, Dir::Right),
                (4, Dir::Right),
                (0, Dir::Left),
                (1, Dir::Right)
            ]
        );
    }

    #[test]
    fn test_new_shape() {
        assert_eq!(Shape::new(&[&[1]]).rows, vec![0b00100000]);
        assert_eq!(
            Shape::new(&[&[1, 0, 0, 0, 0, 0, 0, 0]]).rows,
            vec![0b00100000]
        );
        assert_eq!(
            Shape::new(&[&[0, 1, 0, 0, 0, 0, 1, 0], &[0, 0, 0, 1, 0, 0, 0, 0]]).rows,
            vec![0b00010000, 0b01000010]
        );
        assert_eq!(
            Shape::new(&[&[0, 0, 0], &[0, 1, 1], &[1, 0, 0],]).rows,
            vec![0b00100000, 0b00011000, 0b00000000]
        );
    }

    #[test]
    fn test_shift_right() {
        // Can't shift because rhs is at edge (WIDTH = 7)
        assert_eq!(
            Shape::new(&[&[0, 1, 0, 0, 0, 0, 1, 0]]).rows,
            vec![0b01000010]
        );
        // Wants to shift right by 2, but can't
        assert_eq!(
            Shape::new(&[&[1, 0, 0, 0, 0, 1, 0, 0]]).rows,
            vec![0b10000100]
        );
        // Will shift right by 1
        assert_eq!(
            Shape::new(&[&[0, 1, 0, 0, 0, 1, 0, 0]]).rows,
            vec![0b00100010]
        );
    }

    #[test]
    fn test_board() {
        let shape = Shape::new(&[&[1]]);
        let tall_shape = Shape::new(&[&[1], &[1]]);
        let mut shape2 = shape.clone();
        shape2.shift(Dir::Left);

        let mut board = Board::new();

        assert_eq!(board.intersects(&shape, -1), true);

        board.fix_shape(shape.clone(), 0);
        assert_eq!(board.rows, vec![0b00100000]);
        board.fix_shape(shape.clone(), 1);
        assert_eq!(board.rows, vec![0b00100000, 0b00100000]);
        board.fix_shape(shape2.clone(), 0);
        assert_eq!(board.rows, vec![0b01100000, 0b00100000]);

        assert_eq!(board.intersects(&shape2, 0), true);
        assert_eq!(board.intersects(&tall_shape, 1), true);
        assert_eq!(board.intersects(&shape2, 1), false);
        assert_eq!(board.intersects(&shape2, 2), false);
    }

    #[test]
    fn test_bug() {
        /*
            .....@.
            ....@@@
            .....@.

            Moving Right
            ......@
            .....@@
            ......@
        */

        let mut shape = Shape::new(&[&[0, 1, 0], &[1, 1, 1], &[0, 1, 0]]);
        assert_eq!(shape.first_col, 2);
        assert_eq!(shape.last_col, 4);
    }

    #[test]
    fn test_solve() {
        assert_eq!(compute(EXAMPLE, 2022), 3068);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(compute(EXAMPLE, 1000000000000), 1514285714288);
    }

    #[test]
    fn test_loop() {
        let l = find_loop(EXAMPLE);
        assert_eq!(l.starts, 42);
        assert_eq!(l.length, 35);
        assert_eq!(l.gains_height, 53);
    }
}
