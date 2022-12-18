use std::cmp::Ordering;

use itertools::{EitherOrBoth, Itertools};

#[derive(Debug, Clone)]
enum Value {
    Integer(usize),
    List(Vec<Value>),
}

impl Value {
    fn new(input: &str) -> Self {
        let mut stack = Vec::new();
        let mut vec = Vec::new();
        let mut int = None;
        let mut chars = input.chars();

        assert!(chars.next() == Some('['));
        for c in chars {
            if let Some(d) = c.to_digit(10) {
                int = Some(match int.take() {
                    Some(int) => int * 10 + d as usize,
                    None => d as usize,
                });
                continue;
            }
            if let Some(int) = int.take() {
                vec.push(Value::Integer(int));
            }
            if c == '[' {
                stack.push(vec);
                vec = Vec::new();
            } else if c == ']' {
                vec = match stack.pop() {
                    Some(mut parent) => {
                        parent.push(Value::List(vec));
                        parent
                    }
                    None => return Value::List(vec),
                }
            }
        }
        panic!("Unexpected end of input");
    }

    fn compare(&self, other: &Self) -> Ordering {
        let cmp_single = |l: &Value, b: &[Value]| {
            if let Some(r) = b.first() {
                let ordering = l.compare(r);
                if ordering != Ordering::Equal {
                    return ordering;
                }
            }
            1.cmp(&b.len())
        };
        let cmp_slice = |a: &[Value], b: &[Value]| {
            for item in a.iter().zip_longest(b.iter()) {
                match item {
                    EitherOrBoth::Left(_) => return Ordering::Greater,
                    EitherOrBoth::Right(_) => return Ordering::Less,
                    EitherOrBoth::Both(l, r) => match l.compare(r) {
                        Ordering::Equal => continue,
                        ordering => return ordering,
                    },
                }
            }
            Ordering::Equal
        };
        match (self, other) {
            (Self::List(l), Self::List(r)) => cmp_slice(l, r),
            (l, Self::List(r)) => cmp_single(l, r),
            (Self::List(l), r) => cmp_single(r, l).reverse(),
            (Self::Integer(l), Self::Integer(r)) => l.cmp(r),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.compare(other) == Ordering::Equal
    }
}

impl Eq for Value {}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare(other)
    }
}

fn parse(input: &str) -> impl Iterator<Item = Value> + '_ {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| Value::new(l))
}

pub(crate) fn solve(input: &str) -> usize {
    parse(input)
        .tuples()
        .map(|(l, r)| (l, r))
        .enumerate()
        .filter(|(_, (l, r))| l < r)
        .map(|(i, _)| i + 1)
        .sum()
}

pub(crate) fn solve_2(input: &str) -> usize {
    let mut packets = parse(input).collect_vec();
    let extra_packets = [Value::new("[[6]]"), Value::new("[[2]]")];
    packets.extend(extra_packets.iter().cloned());
    packets.sort();
    packets
        .iter()
        .enumerate()
        .filter(|(_, packet)| extra_packets.contains(packet))
        .map(|(i, _)| i + 1)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
        [1,1,3,1,1]
        [1,1,5,1,1]
        
        [[1],[2,3,4]]
        [[1],4]
        
        [9]
        [[8,7,6]]
        
        [[4,4],4,4]
        [[4,4],4,4,4]
        
        [7,7,7,7]
        [7,7,7]
        
        []
        [3]
        
        [[[]]]
        [[]]
        
        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]
    ";

    #[test]
    fn test_new() {
        assert_eq!(Value::new("[9]"), Value::List(vec![Value::Integer(9)]));
        assert_eq!(
            Value::new("[9, 1]"),
            Value::List(vec![Value::Integer(9), Value::Integer(1)])
        );
        assert_eq!(
            Value::new("[9, [1]]"),
            Value::List(vec![
                Value::Integer(9),
                Value::List(vec![Value::Integer(1)])
            ])
        );
        assert_eq!(
            Value::new("[9, [1, [2], 3]]"),
            Value::List(vec![
                Value::Integer(9),
                Value::List(vec![
                    Value::Integer(1),
                    Value::List(vec![Value::Integer(2)]),
                    Value::Integer(3),
                ])
            ])
        );
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse(EXAMPLE).collect_vec().len(), 16);
    }

    #[test]
    fn test_ord() {
        let l = Value::List(vec![
            Value::Integer(1),
            Value::Integer(1),
            Value::Integer(1),
        ]);
        let r = Value::List(vec![
            Value::Integer(1),
            Value::List(vec![Value::Integer(2), Value::Integer(1)]),
            Value::Integer(1),
        ]);
        assert_eq!(l, l);
        assert_eq!(r, r);
        assert!(l < r);
        assert!(r > l);
    }

    #[test]
    fn test_ord_2() {
        assert!(Value::new("[]") < Value::new("[[]]"));
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(EXAMPLE), 13);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(EXAMPLE), 140);
    }
}
