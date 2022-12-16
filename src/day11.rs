use std::{collections::HashMap, iter::Peekable};

use itertools::Itertools;

struct Monkey {
    items: Vec<isize>,
    operation: Box<dyn Fn(isize) -> isize>,
    test: isize,
    on_true: isize,
    on_false: isize,
}

impl Monkey {
    fn compute(&mut self) -> Vec<(isize, isize)> {
        self.items
            .drain(..)
            .map(|item| {
                let new_item = (self.operation)(item) / 3;
                let destination = if new_item % self.test == 0 {
                    self.on_true
                } else {
                    self.on_false
                };
                (destination, new_item)
            })
            .collect()
    }

    fn compute_2(&mut self) -> Vec<(isize, isize)> {
        self.items
            .drain(..)
            .map(|item| {
                let new_item = (self.operation)(item);
                let destination = if new_item % self.test == 0 {
                    self.on_true
                } else {
                    self.on_false
                };
                (destination, new_item)
            })
            .collect()
    }
}

struct MonkeyIterator<T: Iterator> {
    lines: Peekable<T>,
}

impl<'a, T: Iterator<Item = &'a str>> Iterator for MonkeyIterator<T> {
    type Item = Monkey;

    fn next(&mut self) -> Option<Self::Item> {
        if self.lines.peek().is_none() {
            return None;
        }

        let mut strip = |prefix: &str| {
            let line = self.lines.next().unwrap();
            line.strip_prefix(prefix).unwrap()
        };

        strip("Monkey");

        let items = strip("Starting items: ")
            .split(", ")
            .map(|item| item.parse::<isize>().unwrap())
            .collect_vec();

        let operation: Box<dyn Fn(isize) -> isize> = match &strip("Operation: new = old ")
            .split_ascii_whitespace()
            .collect_vec()[..]
        {
            &["+", "old"] => Box::new(|old: isize| old + old),
            &["*", "old"] => Box::new(|old: isize| old * old),
            &["+", num] => {
                let num = num.parse::<isize>().unwrap();
                Box::new(move |old: isize| old + num)
            }
            &["*", num] => {
                let num = num.parse::<isize>().unwrap();
                Box::new(move |old: isize| old * num)
            }
            _ => panic!("Unexpected operation"),
        };

        let test = strip("Test: divisible by ").parse::<isize>().unwrap();
        let on_true = strip("If true: throw to monkey ").parse::<isize>().unwrap();
        let on_false = strip("If false: throw to monkey ")
            .parse::<isize>()
            .unwrap();

        Some(Monkey {
            items,
            operation,
            test,
            on_true,
            on_false,
        })
    }
}

fn parse(input: &str) -> impl Iterator<Item = Monkey> + '_ {
    MonkeyIterator {
        lines: input
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .peekable(),
    }
}

pub(crate) fn solve(input: &str) -> usize {
    let mut monkeys = parse(input).collect_vec();
    let mut counts = vec![0; monkeys.len()];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            for (dest, item) in monkeys[i].compute() {
                monkeys[dest as usize].items.push(item);
                counts[i] += 1;
            }
        }
    }
    counts.sort();
    counts.iter().rev().take(2).product()
}

pub(crate) fn solve_2(input: &str) -> usize {
    let mut monkeys = parse(input).collect_vec();
    let mut counts = vec![0; monkeys.len()];
    let modulus: isize = monkeys.iter().map(|m| m.test).product();
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            for (dest, item) in monkeys[i].compute_2() {
                let item = item % modulus;
                monkeys[dest as usize].items.push(item);
                counts[i] += 1;
            }
        }
    }
    counts.sort();
    counts.iter().rev().take(2).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let monkey = parse(
            "
            Monkey 2:
              Starting items: 79, 60, 97
              Operation: new = old * old
              Test: divisible by 13
                If true: throw to monkey 1
                If false: throw to monkey 3
            ",
        )
        .next()
        .unwrap();

        assert_eq!(monkey.items, vec![79, 60, 97]);
        assert_eq!(monkey.test, 13);
        assert_eq!(monkey.on_true, 1);
        assert_eq!(monkey.on_false, 3);
    }

    #[test]
    fn test_monkey() {
        let mut monkey = parse(
            "
            Monkey 0:
              Starting items: 79, 98
              Operation: new = old * 19
              Test: divisible by 23
                If true: throw to monkey 2
                If false: throw to monkey 3
            ",
        )
        .next()
        .unwrap();

        assert_eq!(monkey.compute(), vec![(3, 500), (3, 620)]);
    }

    const example: &str = "
        Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3

        Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0

        Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3

        Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1
    ";

    #[test]
    fn test_solve() {
        assert_eq!(solve(example), 10605);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(example), 2713310158);
    }
}
