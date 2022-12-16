use std::collections::HashSet;

struct Grid {
    cells: Vec<Vec<isize>>,
    start: (usize, usize),
    end: (usize, usize),
    size: (usize, usize),
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut cells = vec![];
        let mut start = None;
        let mut end = None;
        let lines = input.lines().map(|l| l.trim()).filter(|l| !l.is_empty());
        for (y, line) in lines.enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let mut row_cells = vec![];
            for (x, mut c) in line.chars().enumerate() {
                if c == 'S' {
                    start = Some((x, y));
                    c = 'a';
                } else if c == 'E' {
                    end = Some((x, y));
                    c = 'z';
                }
                row_cells.push(c as isize - 'a' as isize);
            }
            cells.push(row_cells);
        }

        let x_dim = cells.first().unwrap().len();
        let y_dim = cells.len();

        Self {
            cells,
            start: start.unwrap(),
            end: end.unwrap(),
            size: (x_dim, y_dim),
        }
    }
}

struct BFS<'a> {
    grid: &'a Grid,
    queue: Vec<((usize, usize), usize)>,
    seen: HashSet<(usize, usize)>,
}

impl<'a> BFS<'a> {
    fn new(grid: &'a Grid) -> Self {
        Self {
            grid,
            queue: vec![(grid.end, 0)],
            seen: HashSet::new(),
        }
    }
}

impl Iterator for BFS<'_> {
    type Item = ((usize, usize), usize);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((pos, steps)) = self.queue.first().copied() {
            self.queue.remove(0);
            if !self.seen.insert(pos) {
                continue;
            }
            self.queue.extend(
                [
                    (pos.0, pos.1 + 1),
                    (pos.0, pos.1.wrapping_sub(1)),
                    (pos.0 + 1, pos.1),
                    (pos.0.wrapping_sub(1), pos.1),
                ]
                .into_iter()
                .filter(|&(x, y)| x < self.grid.size.0 && y < self.grid.size.1)
                .filter(|&(x, y)| self.grid.cells[pos.1][pos.0] <= self.grid.cells[y][x] + 1)
                .map(|p| (p, steps + 1)),
            );
            return Some((pos, steps));
        }
        return None;
    }
}

pub(crate) fn solve(input: &str) -> usize {
    let grid = Grid::new(input);
    BFS::new(&grid)
        .find(|&(pos, _)| pos == grid.start)
        .unwrap()
        .1
}

pub(crate) fn solve_2(input: &str) -> usize {
    let grid = Grid::new(input);
    BFS::new(&grid)
        .filter(|&((x, y), _)| grid.cells[y][x] == 0)
        .min_by_key(|&(_, steps)| steps)
        .unwrap()
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
    ";

    #[test]
    fn test_parse() {
        let grid = Grid::new(EXAMPLE);
        assert_eq!(grid.size, (8, 5));
        assert_eq!(grid.start, (0, 0));
        assert_eq!(grid.end, (5, 2));
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(EXAMPLE), 31);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(EXAMPLE), 29);
    }
}
