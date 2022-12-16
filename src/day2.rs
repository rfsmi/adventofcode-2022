use itertools::Itertools;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Move {
    Paper,
    Scissors,
    Rock,
}

impl Move {
    fn bonus(self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn score(self, other: Move) -> i32 {
        let left_wins = |l, r| match (l, r) {
            (Move::Paper, Move::Rock) => true,
            (Move::Scissors, Move::Paper) => true,
            (Move::Rock, Move::Scissors) => true,
            _ => false,
        };
        if left_wins(self, other) {
            6
        } else if left_wins(other, self) {
            0
        } else {
            3
        }
    }

    fn from_str(s: &str, strs: [&str; 3]) -> Move {
        if s == strs[0] {
            Move::Rock
        } else if s == strs[1] {
            Move::Paper
        } else if s == strs[2] {
            Move::Scissors
        } else {
            panic!("Unexpected move {s}; must be one of {strs:?}")
        }
    }

    fn find(other: Move, result: &str) -> Move {
        let desired_score = match result {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => panic!("Unknown result type {result}"),
        };
        for m in [Move::Rock, Move::Paper, Move::Scissors] {
            if m.score(other) == desired_score {
                return m;
            }
        }
        panic!("Can't find move for {other:?} {result}");
    }
}

fn parse_str_tuples(input: &str) -> impl Iterator<Item = (&str, &str)> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.split_ascii_whitespace().collect_tuple().unwrap())
}

fn parse(input: &str) -> impl Iterator<Item = (Move, Move)> + '_ {
    parse_str_tuples(input).map(|(l, r)| {
        (
            Move::from_str(l, ["A", "B", "C"]),
            Move::from_str(r, ["X", "Y", "Z"]),
        )
    })
}

fn parse_2(input: &str) -> impl Iterator<Item = (Move, Move)> + '_ {
    parse_str_tuples(input)
        .map(|(l, r)| (Move::from_str(l, ["A", "B", "C"]), r))
        .map(|(l, r)| (l, Move::find(l, r)))
}

pub(crate) fn solve(input: &str) -> i32 {
    parse(input).map(|(l, r)| r.score(l) + r.bonus()).sum()
}

pub(crate) fn solve_2(input: &str) -> i32 {
    parse_2(input).map(|(l, r)| r.score(l) + r.bonus()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let result: Vec<_> = parse(
            "
            A X
            B Y
            C Z
        ",
        )
        .collect();
        assert_eq!(
            result,
            vec![
                (Move::Rock, Move::Rock),
                (Move::Paper, Move::Paper),
                (Move::Scissors, Move::Scissors)
            ]
        )
    }

    #[test]
    fn test_find() {
        assert_eq!(Move::find(Move::Rock, "X"), Move::Scissors);
        assert_eq!(Move::find(Move::Rock, "Y"), Move::Rock);
        assert_eq!(Move::find(Move::Rock, "Z"), Move::Paper);
    }

    #[test]
    fn test_draw() {
        assert_eq!(Move::score(Move::Rock, Move::Rock), 3);
        assert_eq!(Move::score(Move::Paper, Move::Paper), 3);
        assert_eq!(Move::score(Move::Scissors, Move::Scissors), 3);
    }

    #[test]
    fn test_win() {
        assert_eq!(Move::score(Move::Rock, Move::Scissors), 6);
        assert_eq!(Move::score(Move::Paper, Move::Rock), 6);
        assert_eq!(Move::score(Move::Scissors, Move::Paper), 6);
    }

    #[test]
    fn test_loss() {
        assert_eq!(Move::score(Move::Scissors, Move::Rock), 0);
        assert_eq!(Move::score(Move::Rock, Move::Paper), 0);
        assert_eq!(Move::score(Move::Paper, Move::Scissors), 0);
    }

    #[test]
    fn test_bonus() {
        assert_eq!(Move::Rock.bonus(), 1);
        assert_eq!(Move::Paper.bonus(), 2);
        assert_eq!(Move::Scissors.bonus(), 3);
    }

    #[test]
    fn test_full() {
        assert_eq!(solve("B Z"), 9);
    }
}
