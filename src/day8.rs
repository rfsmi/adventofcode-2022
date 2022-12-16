use std::iter::empty;

use itertools::{FoldWhile, Itertools};

#[derive(Debug, PartialEq)]
struct Tree {
    height: u8,
    position: (usize, usize),
}

struct Forest {
    trees: Vec<Vec<Tree>>,
}

impl Forest {
    fn new(heights: Vec<Vec<u8>>) -> Self {
        Forest {
            trees: heights
                .into_iter()
                .enumerate()
                .map(|(y, row)| {
                    row.into_iter()
                        .enumerate()
                        .map(|(x, height)| Tree {
                            position: (x, y),
                            height,
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn size(&self) -> (usize, usize) {
        let y_size = self.trees.len();
        let x_size = self.trees.first().map_or(0, |row| row.len());
        (x_size, y_size)
    }

    fn trees(&self, line: Line) -> impl Iterator<Item = &'_ Tree> {
        LineIter {
            forest: self,
            line,
            pos: 0,
        }
    }

    fn visible_trees(&self, line: Line) -> impl Iterator<Item = &'_ Tree> {
        let mut maybe_prev: Option<&Tree> = None;
        self.trees(line).filter(move |tree| match maybe_prev {
            Some(prev) if prev.height >= tree.height => false,
            _ => {
                maybe_prev = Some(*tree);
                true
            }
        })
    }

    fn view_distances(&self, line: Line) -> impl Iterator<Item = (&Tree, usize)> {
        let mut heights: Vec<u8> = Vec::new();
        self.trees(line.reverse()).map(move |tree| {
            let count = heights
                .iter()
                .copied()
                .rev()
                .fold_while(0, |count, height| {
                    if height >= tree.height {
                        FoldWhile::Done(count + 1)
                    } else {
                        FoldWhile::Continue(count + 1)
                    }
                })
                .into_inner();
            heights.push(tree.height);
            (tree, count)
        })
    }
}

enum Line {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl Line {
    fn reverse(self) -> Self {
        match self {
            Self::Down(i) => Self::Up(i),
            Self::Up(i) => Self::Down(i),
            Self::Left(i) => Self::Right(i),
            Self::Right(i) => Self::Left(i),
        }
    }
}

struct LineIter<'a> {
    forest: &'a Forest,
    line: Line,
    pos: usize,
}

impl<'a> Iterator for LineIter<'a> {
    type Item = &'a Tree;

    fn next(&mut self) -> Option<Self::Item> {
        let (x_size, y_size) = self.forest.size();
        let (x, y) = match self.line {
            Line::Right(y) => (self.pos as isize, y as isize),
            Line::Left(y) => (x_size as isize - 1 - self.pos as isize, y as isize),
            Line::Down(x) => (x as isize, self.pos as isize),
            Line::Up(x) => (x as isize, y_size as isize - 1 - self.pos as isize),
        };
        if 0 <= x && x < x_size as isize && 0 <= y && y < y_size as isize {
            self.pos += 1;
            Some(&self.forest.trees[y as usize][x as usize])
        } else {
            None
        }
    }
}

fn parse(input: &str) -> Forest {
    Forest::new(
        input
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect_vec()
            })
            .collect_vec(),
    )
}

pub(crate) fn solve(input: &str) -> usize {
    let forest = parse(input);
    let (x_size, y_size) = forest.size();
    empty()
        .chain((0..x_size).flat_map(|x| forest.visible_trees(Line::Up(x))))
        .chain((0..x_size).flat_map(|x| forest.visible_trees(Line::Down(x))))
        .chain((0..y_size).flat_map(|y| forest.visible_trees(Line::Left(y))))
        .chain((0..y_size).flat_map(|y| forest.visible_trees(Line::Right(y))))
        .unique_by(|tree| tree.position)
        .count()
}

pub(crate) fn solve_2(input: &str) -> usize {
    let forest = parse(input);
    let (x_size, y_size) = forest.size();
    empty()
        .chain((0..x_size).flat_map(|x| forest.view_distances(Line::Up(x))))
        .chain((0..x_size).flat_map(|x| forest.view_distances(Line::Down(x))))
        .chain((0..y_size).flat_map(|y| forest.view_distances(Line::Left(y))))
        .chain((0..y_size).flat_map(|y| forest.view_distances(Line::Right(y))))
        .into_group_map_by(|(tree, _)| tree.position)
        .values()
        .map(|distances| {
            distances
                .iter()
                .map(|(_, distance)| distance)
                .copied()
                .product()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let forest = parse(
            "
            12
            20
        ",
        );
        let heights = forest
            .trees
            .iter()
            .map(|row| row.iter().map(|t| t.height).collect_vec())
            .collect_vec();
        assert_eq!(heights, vec![vec![1, 2], vec![2, 0],]);
    }

    #[test]
    fn test_size() {
        let forest = parse(
            "
            123
            223
        ",
        );
        assert_eq!(forest.size(), (3, 2));
        assert_eq!(parse("").size(), (0, 0));
    }

    #[test]
    fn test_iter() {
        macro_rules! test {
            ($input:literal @ $line:expr => $($tree: literal)*) => {
                let actual = parse($input).trees($line).map(|t| t.height).collect_vec();
                let expected = vec![$($tree),*];
                assert_eq!(actual, expected);
            };
        }
        test!("1234" @ Line::Left(0) => 4 3 2 1);
        test!("1234" @ Line::Right(0) => 1 2 3 4);
        test!("1\n2\n3\n4" @ Line::Up(0) => 4 3 2 1);
        test!("1\n2\n3\n4" @ Line::Down(0) => 1 2 3 4);
        test!("1234" @ Line::Up(3) => 4);
        test!("1234" @ Line::Down(2) => 3);
        test!("1\n2\n3\n4" @ Line::Left(1) => 2);
        test!("1\n2\n3\n4" @ Line::Right(0) => 1);
        test!("" @ Line::Down(0) => );
        test!("" @ Line::Left(0) => );
    }

    #[test]
    fn test_visible_iter() {
        macro_rules! test {
            ($input:literal @ $line:expr => $($tree: literal)*) => {
                let actual = parse($input).visible_trees($line).map(|t| t.height).collect_vec();
                let expected = vec![$($tree),*];
                assert_eq!(actual, expected);
            };
        }
        test!("1234" @ Line::Left(0) => 4);
        test!("1234" @ Line::Right(0) => 1 2 3 4);
        test!("1334" @ Line::Right(0) => 1 3 4);
        test!("1354" @ Line::Right(0) => 1 3 5);
    }

    #[test]
    fn test_view_distances() {
        macro_rules! test {
            ($input:literal @ $line:expr => $($tree: literal)*) => {
                let actual = parse($input)
                    .view_distances($line)
                    .sorted_by_key(|(tree, _)| tree.position)
                    .map(|(_, distance)| distance)
                    .collect_vec();
                let expected = vec![$($tree),*];
                assert_eq!(actual, expected);
            };
        }
        test!("1234" @ Line::Left(0) => 0 1 2 3);
        test!("1234" @ Line::Right(0) => 1 1 1 0);
        test!("1\n2\n3\n4" @ Line::Up(0) => 0 1 2 3);
        test!("1\n2\n3\n4" @ Line::Down(0) => 1 1 1 0);
        test!("25512" @ Line::Left(0) => 0 1 1 1 2);
        test!("33549" @ Line::Left(0) => 0 1 2 1 4);
    }

    #[test]
    fn test_solve() {
        let input = "
            30373
            25512
            65332
            33549
            35390
        ";
        assert_eq!(solve(input), 21);
    }

    #[test]
    fn test_solve_2() {
        let input = "
            30373
            25512
            65332
            33549
            35390
        ";
        assert_eq!(solve_2(input), 8);
    }
}
