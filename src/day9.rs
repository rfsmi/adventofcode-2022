use std::{
    collections::HashSet,
    iter::{once, repeat},
    ops::{Add, Mul, Sub},
};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Vector {
    x: isize,
    y: isize,
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul for Vector {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Vector {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn dot(self, rhs: Self) -> isize {
        let product = self * rhs;
        product.x + product.y
    }

    fn funky_norm(self) -> Self {
        let (x, y) = [self.x, self.y]
            .into_iter()
            .map(|p| {
                if p == 0 {
                    0
                } else {
                    let sign = if p < 0 { -1 } else { 1 };
                    let abs = p.abs();
                    sign * (abs + abs - 1) / abs
                }
            })
            .collect_tuple()
            .unwrap();
        Self::new(x, y)
    }
}

struct Snake<const N: usize> {
    head: Vector,
    tail: [Vector; N],
}

trait SnakeLike {
    fn new() -> Self;
    fn move_one(&mut self, direction: Direction);
    fn end(&self) -> Vector;
    fn to_string(&self) -> String;
}

impl<const N: usize> SnakeLike for Snake<N> {
    fn new() -> Self {
        let origin = Vector::new(0, 0);
        Snake {
            head: origin,
            tail: [origin; N],
        }
    }

    fn move_one(&mut self, direction: Direction) {
        self.head = self.head + direction.into();
        let mut prev = self.head;
        for next in &mut self.tail {
            let difference = prev - *next;
            if difference.dot(difference) > 2 {
                *next = *next + difference.funky_norm();
            }
            prev = *next;
        }
    }

    fn end(&self) -> Vector {
        self.tail[N - 1]
    }
    fn to_string(&self) -> String {
        // Find the bounds of the snake
        let (min, max) = self
            .tail
            .iter()
            .fold((self.head, self.head), |(min, max), &part| {
                let min = Vector::new(min.x.min(part.x), min.y.min(part.y));
                let max = Vector::new(max.x.max(part.x), max.y.max(part.y));
                (min, max)
            });
        // Fill an appropriately sized Vec with the snake
        let width = (max.x - min.x + 1) as usize;
        let height = (max.y - min.y + 1) as usize;
        let mut data = vec![vec![None; width]; height];
        for (i, &part) in once(&self.head).chain(self.tail.iter()).enumerate() {
            let coord = part - min;
            if let None = data[coord.y as usize][coord.x as usize] {
                data[coord.y as usize][coord.x as usize] = Some(i);
            }
        }
        // Convert it to a String
        data.iter_mut()
            .map(|row| {
                row.into_iter()
                    .map(|cell| match cell {
                        None => ".".into(),
                        Some(0) => "H".into(),
                        Some(i) => i.to_string(),
                    })
                    .chain(once("\n".into()))
                    .collect::<String>()
            })
            .collect()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl From<Direction> for Vector {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Left => Vector::new(-1, 0),
            Direction::Right => Vector::new(1, 0),
            Direction::Down => Vector::new(0, -1),
            Direction::Up => Vector::new(0, 1),
        }
    }
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Can't parse {s} into Direction"),
        }
    }
}

fn parse(input: &str) -> impl Iterator<Item = Direction> + '_ {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .flat_map(|l| {
            l.split(" ")
                .tuples()
                .flat_map(|(dir, num)| repeat(dir.into()).take(num.parse::<usize>().unwrap()))
        })
}

fn compute<S: SnakeLike>(input: &str) -> usize {
    let snake = S::new();
    let hs: HashSet<_> = [snake.end()].into();
    parse(input)
        .fold((hs, snake), |(mut hs, mut snake), d| {
            snake.move_one(d);
            hs.insert(snake.end());
            (hs, snake)
        })
        .0
        .len()
}

pub(crate) fn solve(input: &str) -> usize {
    compute::<Snake<1>>(input)
}

pub(crate) fn solve_2(input: &str) -> usize {
    compute::<Snake<9>>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_funky_norm() {
        assert_eq!(Vector::new(1, 0).funky_norm(), Vector::new(1, 0));
        assert_eq!(Vector::new(2, 0).funky_norm(), Vector::new(1, 0));
        assert_eq!(Vector::new(-1, 0).funky_norm(), Vector::new(-1, 0));
        assert_eq!(Vector::new(-2, 0).funky_norm(), Vector::new(-1, 0));
    }

    #[test]
    fn test_snake() {
        let mut snake = Snake::<1>::new();
        snake.move_one(Direction::Down);
        assert_eq!(
            (snake.head, snake.tail),
            (Vector::new(0, -1), [Vector::new(0, 0)])
        );
        snake.move_one(Direction::Down);
        assert_eq!(
            (snake.head, snake.tail),
            (Vector::new(0, -2), [Vector::new(0, -1)])
        );
        snake.move_one(Direction::Right);
        assert_eq!(
            (snake.head, snake.tail),
            (Vector::new(1, -2), [Vector::new(0, -1)])
        );
        snake.move_one(Direction::Right);
        assert_eq!(
            (snake.head, snake.tail),
            (Vector::new(2, -2), [Vector::new(1, -2)])
        );
    }

    #[test]
    fn test_longer_snake() {
        let mut snake = Snake::<2>::new();
        // x...

        snake.move_one(Direction::Right);
        // xx..

        snake.move_one(Direction::Right);
        // xxx.

        snake.move_one(Direction::Right);
        // .xxx

        assert_eq!(
            (snake.head, snake.tail),
            (Vector::new(3, 0), [Vector::new(2, 0), Vector::new(1, 0)])
        );
    }

    #[test]
    fn test_parse() {
        let directions = parse(
            "
            U 2
            D 1
            L 1
            R 3
        ",
        )
        .collect_vec();
        assert_eq!(
            directions,
            vec![
                Direction::Up,
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
                Direction::Right,
                Direction::Right,
            ]
        );
    }

    #[test]
    fn test_solve() {
        let input = "
            R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2
        ";
        assert_eq!(solve(input), 13);
    }

    #[test]
    fn test_solve_2() {
        let input = "
            R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20
        ";
        assert_eq!(solve_2(input), 36);
    }

    #[test]
    fn test_print() {
        let input = "
            R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20
        ";
        let mut snake = Snake::<9>::new();
        for direction in parse(input) {
            snake.move_one(direction);
            println!("{}", snake.to_string());
        }
    }
}
