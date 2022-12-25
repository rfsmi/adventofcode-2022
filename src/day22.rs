use std::{collections::HashMap, iter::zip};

use itertools::Itertools;

#[derive(Clone, Copy)]
enum Cell {
    Wall,
    Open,
}

#[derive(Copy, Clone)]
enum Line {
    Top(isize, isize),
    Left(isize, isize),
    Bottom(isize, isize),
    Right(isize, isize),
}

impl Line {
    fn src_facing(&self) -> Facing {
        match self {
            Line::Top(_, _) => Facing::Up,
            Line::Bottom(_, _) => Facing::Down,
            Line::Left(_, _) => Facing::Left,
            Line::Right(_, _) => Facing::Right,
        }
    }

    fn dst_facing(&self) -> Facing {
        match self {
            Line::Top(_, _) => Facing::Down,
            Line::Bottom(_, _) => Facing::Up,
            Line::Left(_, _) => Facing::Right,
            Line::Right(_, _) => Facing::Left,
        }
    }

    fn to_coords(self, scale: isize) -> impl Iterator<Item = (isize, isize)> {
        let (x_range, y_range) = match self {
            Line::Top(x, y) => (x * scale..(1 + x) * scale, y * scale..1 + y * scale),
            Line::Bottom(x, y) => (x * scale..(1 + x) * scale, y * scale - 1..y * scale),
            Line::Left(x, y) => (x * scale..1 + x * scale, y * scale..(1 + y) * scale),
            Line::Right(x, y) => (x * scale - 1..x * scale, y * scale..(1 + y) * scale),
        };
        [x_range.into_iter(), y_range.into_iter()]
            .into_iter()
            .multi_cartesian_product()
            .map(|v| (v[0], v[1]))
    }
}

struct Board {
    cells: HashMap<(isize, isize), Cell>,
    discontinuities: HashMap<Player, Player>,
    initial_player: Player,
}

impl Board {
    fn new(rows: Vec<Vec<Option<Cell>>>) -> Self {
        let mut cells = HashMap::new();
        let mut initial_pos = (isize::MAX, isize::MAX);
        for (y, row) in rows.iter().enumerate() {
            for (x, maybe_cell) in row.iter().enumerate() {
                if let &Some(cell) = maybe_cell {
                    cells.insert((x as isize, y as isize), cell);
                    initial_pos = initial_pos.min((y as isize, x as isize));
                };
            }
        }
        Self {
            cells,
            discontinuities: HashMap::new(),
            initial_player: Player {
                x: initial_pos.1,
                y: initial_pos.0,
                facing: Facing::Right,
            },
        }
    }

    fn add_discontinuity(&mut self, scale: isize, a: Line, b: Line) {
        let mut add_directional_discontinuity = |from: Line, to: Line| {
            let src_facing = from.src_facing();
            let dst_facing = to.dst_facing();
            let mut dst_players = to
                .to_coords(scale)
                .map(|(x, y)| Player {
                    x,
                    y,
                    facing: dst_facing,
                })
                .collect_vec();
            match (src_facing, dst_facing) {
                (Facing::Up, Facing::Up | Facing::Right) => (),
                (Facing::Right, Facing::Right | Facing::Up) => (),
                (Facing::Left, Facing::Left | Facing::Down) => (),
                (Facing::Down, Facing::Down | Facing::Left) => (),
                _ => dst_players.reverse(),
            }
            self.discontinuities.extend(zip(
                from.to_coords(scale).map(|(x, y)| Player {
                    x,
                    y,
                    facing: src_facing,
                }),
                dst_players.into_iter(),
            ));
        };
        add_directional_discontinuity(a, b);
        add_directional_discontinuity(b, a);
    }

    fn walk(&self, player: Player) -> impl Iterator<Item = Player> + '_ {
        PlayerWalker {
            board: self,
            player: Some(player),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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
        let new_player = match self.board.discontinuities.get(&player) {
            Some(&new_player) => new_player,
            None => {
                let x = player.x;
                let y = player.y;
                let (new_x, new_y) = match player.facing {
                    Facing::Right => (x + 1, y),
                    Facing::Left => (x - 1, y),
                    Facing::Up => (x, y - 1),
                    Facing::Down => (x, y + 1),
                };
                Player {
                    x: new_x,
                    y: new_y,
                    facing: player.facing,
                }
            }
        };
        if let Cell::Open = self
            .board
            .cells
            .get(&(new_player.x, new_player.y))
            .expect(&format!("OOB (x: {}, y: {})", new_player.x, new_player.y))
        {
            self.player = Some(new_player);
        }
        Some(player)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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

fn compute(board: Board, instructions: Vec<Instruction>) -> isize {
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

pub(crate) fn solve(input: &str) -> isize {
    let (mut board, instructions) = parse(input);
    //       0  1  2  3
    //           4  3
    //    0     oooxxx
    //         1oooxxx1
    //          oooxxx
    //    1     xxx 3
    //         2xxx2
    //        6 xxx
    //    2  xxxooo
    //      5xxxooo5
    //       xxxooo
    //    3  ooo 4
    //      7ooo7
    //       ooo
    //    4   6

    board.add_discontinuity(50, Line::Left(1, 0), Line::Right(3, 0)); // 1
    board.add_discontinuity(50, Line::Left(1, 1), Line::Right(2, 1)); // 2
    board.add_discontinuity(50, Line::Top(2, 0), Line::Bottom(2, 1)); // 3
    board.add_discontinuity(50, Line::Top(1, 0), Line::Bottom(1, 3)); // 4
    board.add_discontinuity(50, Line::Left(0, 2), Line::Right(2, 2)); // 5
    board.add_discontinuity(50, Line::Top(0, 2), Line::Bottom(0, 4)); // 6
    board.add_discontinuity(50, Line::Left(0, 3), Line::Right(1, 3)); // 7
    compute(board, instructions)
}

pub(crate) fn solve_2(input: &str) -> isize {
    let (mut board, instructions) = parse(input);
    //       0  1  2  3
    //           7  6
    //    0     oooxxx
    //         5o2ox1x4
    //          oooxxx
    //    1     xxx 1
    //         2x3x1
    //        2 xxx
    //    2  xxxooo
    //      5x5xo4o4
    //       xxxooo
    //    3  ooo 3
    //      7o6o3
    //       ooo
    //    4   6

    board.add_discontinuity(50, Line::Bottom(2, 1), Line::Right(2, 1)); // 1
    board.add_discontinuity(50, Line::Left(1, 1), Line::Top(0, 2)); // 2
    board.add_discontinuity(50, Line::Bottom(1, 3), Line::Right(1, 3)); // 3
    board.add_discontinuity(50, Line::Right(3, 0), Line::Right(2, 2)); // 4
    board.add_discontinuity(50, Line::Left(1, 0), Line::Left(0, 2)); // 5
    board.add_discontinuity(50, Line::Top(2, 0), Line::Bottom(0, 4)); // 6
    board.add_discontinuity(50, Line::Top(1, 0), Line::Left(0, 3)); // 7
    compute(board, instructions)
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
    fn test_line_bottom() {
        let players = Line::Bottom(2, 2).to_coords(5).collect_vec();
        assert_eq!(
            players,
            vec![(10, 9,), (11, 9,), (12, 9,), (13, 9,), (14, 9,),]
        );
    }

    #[test]
    fn test_line_top() {
        let players = Line::Top(1, 0).to_coords(5).collect_vec();
        assert_eq!(players, vec![(5, 0,), (6, 0,), (7, 0,), (8, 0,), (9, 0,),]);
    }

    #[test]
    fn test_line_left() {
        let players = Line::Left(1, 0).to_coords(5).collect_vec();
        assert_eq!(players, vec![(5, 0,), (5, 1,), (5, 2,), (5, 3,), (5, 4,),]);
    }

    #[test]
    fn test_line_right() {
        let players = Line::Right(0, 1).to_coords(5).collect_vec();
        assert_eq!(
            players,
            vec![(-1, 5,), (-1, 6,), (-1, 7,), (-1, 8,), (-1, 9,),]
        );
    }
}
