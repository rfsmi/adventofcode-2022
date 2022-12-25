use std::collections::HashMap;

use itertools::Itertools;

#[derive(Clone, Copy)]
enum Cell {
    Wall,
    Open,
}

struct Board {
    cells: HashMap<(isize, isize), Cell>,
    col_lens: HashMap<isize, isize>,
    row_lens: HashMap<isize, isize>,
    initial_player: Player,
}

impl Board {
    fn new(rows: Vec<Vec<Option<Cell>>>) -> Self {
        let mut col_lens = HashMap::new();
        let mut row_lens = HashMap::new();
        let mut cells = HashMap::new();
        let mut initial_pos = (isize::MAX, isize::MAX);
        for (y, row) in rows.iter().enumerate() {
            for (x, maybe_cell) in row.iter().enumerate() {
                if let &Some(cell) = maybe_cell {
                    cells.insert((x as isize, y as isize), cell);
                    *col_lens.entry(x as isize).or_default() += 1;
                    *row_lens.entry(y as isize).or_default() += 1;
                    initial_pos = initial_pos.min((y as isize, x as isize));
                };
            }
        }
        Self {
            cells,
            col_lens,
            row_lens,
            initial_player: Player {
                x: initial_pos.1,
                y: initial_pos.0,
                facing: Facing::Right,
            },
        }
    }

    fn walk(&self, player: Player) -> impl Iterator<Item = Player> + '_ {
        PlayerWalker {
            board: self,
            player: Some(player),
        }
    }
}

#[derive(Clone, Copy)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

struct PlayerWalker<'a> {
    board: &'a Board,
    player: Option<Player>,
}

impl<'a> Iterator for PlayerWalker<'a> {
    type Item = Player;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(player) = self.player.take() else {
            return None;
        };
        let x = player.x;
        let y = player.y;
        let (mut new_x, mut new_y) = match player.facing {
            Facing::Right => (x + 1, y),
            Facing::Left => (x - 1, y),
            Facing::Up => (x, y - 1),
            Facing::Down => (x, y + 1),
        };
        if !self.board.cells.contains_key(&(new_x, new_y)) {
            match player.facing {
                Facing::Right => new_x -= self.board.row_lens[&y],
                Facing::Left => new_x += self.board.row_lens[&y],
                Facing::Down => new_y -= self.board.col_lens[&x],
                Facing::Up => new_y += self.board.col_lens[&x],
            }
        }
        if let Cell::Open = self.board.cells.get(&(new_x, new_y)).unwrap() {
            self.player = Some(Player {
                x: new_x,
                y: new_y,
                facing: player.facing,
            });
        }
        Some(player)
    }
}

#[derive(Clone, Copy)]
struct Player {
    x: isize,
    y: isize,
    facing: Facing,
}

impl Player {
    fn turn_left(self) -> Self {
        Self {
            facing: match self.facing {
                Facing::Down => Facing::Right,
                Facing::Right => Facing::Up,
                Facing::Up => Facing::Left,
                Facing::Left => Facing::Down,
            },
            ..self
        }
    }

    fn turn_right(self) -> Self {
        Self {
            facing: match self.facing {
                Facing::Right => Facing::Down,
                Facing::Up => Facing::Right,
                Facing::Left => Facing::Up,
                Facing::Down => Facing::Left,
            },
            ..self
        }
    }
}

enum Instruction {
    Forward(usize),
    Left,
    Right,
}

fn parse(input: &str) -> (Board, Vec<Instruction>) {
    let mut lines = input.lines().skip_while(|l| l.is_empty());

    let board = Board::new(
        lines
            .take_while_ref(|l| !l.is_empty())
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        ' ' => None,
                        '.' => Some(Cell::Open),
                        '#' => Some(Cell::Wall),
                        _ => panic!("Unexpected character in board: {c}"),
                    })
                    .collect_vec()
            })
            .collect(),
    );

    let mut chars = lines.skip(1).next().unwrap().trim().chars().peekable();
    let mut instructions = Vec::new();
    while let Some(c) = chars.next() {
        if let Some(mut num) = c.to_digit(10) {
            while let Some(d) = chars.peek().and_then(|d| d.to_digit(10)) {
                num = num * 10 + d;
                chars.next();
            }
            instructions.push(Instruction::Forward(num as usize));
        } else if c == 'L' {
            instructions.push(Instruction::Left);
        } else if c == 'R' {
            instructions.push(Instruction::Right);
        } else {
            panic!("Unexpected character in instruction list: {c}")
        }
    }
    (board, instructions)
}

pub(crate) fn solve(input: &str) -> isize {
    let (board, instructions) = parse(input);
    let mut player = board.initial_player;
    for instruction in instructions {
        player = match instruction {
            Instruction::Left => player.turn_left(),
            Instruction::Right => player.turn_right(),
            Instruction::Forward(distance) => board.walk(player).take(distance + 1).last().unwrap(),
        }
    }
    1000 * (player.y + 1)
        + 4 * (player.x + 1)
        + match player.facing {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        }
}

#[cfg(test)]
mod tests {

    use super::*;

    const EXAMPLE: &str = "
        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

        10R5L5R10L4R5L5
    ";

    #[test]
    fn test_solve() {
        assert_eq!(solve(EXAMPLE), 6032);
    }
}
