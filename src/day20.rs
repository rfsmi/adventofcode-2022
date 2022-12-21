use std::{
    cell::RefCell,
    fmt::Debug,
    iter::zip,
    rc::{Rc, Weak},
};

use itertools::Itertools;

#[derive(Debug)]
struct Node {
    value: isize,
    prev: Weak<RefCell<Node>>,
    next: Weak<RefCell<Node>>,
}

impl Node {
    fn new(value: isize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            prev: Weak::new(),
            next: Weak::new(),
            value,
        }))
    }
}

enum Direction {
    Forwards,
    Backwards,
}

struct NodeIter {
    dir: Direction,
    node: Rc<RefCell<Node>>,
}

impl Iterator for NodeIter {
    type Item = Rc<RefCell<Node>>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.node.clone();
        self.node = match self.dir {
            Direction::Backwards => self.node.borrow().prev.upgrade().unwrap(),
            Direction::Forwards => self.node.borrow().next.upgrade().unwrap(),
        };
        Some(result)
    }
}

struct List {
    zero: Rc<RefCell<Node>>,
    nodes: Vec<Rc<RefCell<Node>>>,
}

impl Debug for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nodes = Self::iter(Direction::Forwards, self.zero.clone())
            .map(|n| n.borrow().value)
            .take(self.nodes.len());
        f.debug_list().entries(nodes).finish()
    }
}

impl Eq for List {}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        if self.nodes.len() != other.nodes.len() {
            return false;
        }
        let a = Self::iter(Direction::Forwards, self.zero.clone());
        let b = Self::iter(Direction::Forwards, other.zero.clone());
        zip(a, b)
            .take(self.nodes.len())
            .all(|(a, b)| a.borrow().value == b.borrow().value)
    }
}

impl List {
    fn new(values: impl Iterator<Item = isize>) -> Self {
        let nodes = values.map(|value| Node::new(value)).collect_vec();
        for (node, next) in nodes.iter().chain(nodes.first()).tuple_windows() {
            node.borrow_mut().next = Rc::downgrade(next);
            next.borrow_mut().prev = Rc::downgrade(node);
        }
        let zero = nodes
            .iter()
            .find(|n| n.borrow().value == 0)
            .unwrap()
            .clone();
        Self { zero, nodes }
    }

    fn scale(&self, factor: isize) {
        for node in &self.nodes {
            node.borrow_mut().value *= factor;
        }
    }

    fn shift(&mut self, node: Rc<RefCell<Node>>, offset: isize) {
        // Remove the node from the list
        let prev = node.borrow().prev.upgrade().unwrap();
        let next = node.borrow().next.upgrade().unwrap();

        prev.borrow_mut().next = Rc::downgrade(&next);
        next.borrow_mut().prev = Rc::downgrade(&prev);

        // Determine how far to shift, and in which direction
        let len = self.nodes.len() - 1;
        let mut distance = offset.rem_euclid(len as isize) as usize;
        let mut dir = Direction::Forwards;
        if distance > len / 2 {
            distance = len - distance;
            dir = Direction::Backwards;
        }

        // Find the new (prev, next) nodes
        let prev = Self::iter(dir, prev)
            .take(1 + distance as usize)
            .last()
            .unwrap();
        let next = prev.borrow().next.upgrade().unwrap();

        // Insert the node between the new (prev, next) nodes
        prev.borrow_mut().next = Rc::downgrade(&node);
        next.borrow_mut().prev = Rc::downgrade(&node);
        node.borrow_mut().next = Rc::downgrade(&next);
        node.borrow_mut().prev = Rc::downgrade(&prev);
    }

    fn mix(&mut self, node: Rc<RefCell<Node>>) {
        let offset = node.borrow().value;
        self.shift(node, offset);
    }

    fn iter(dir: Direction, node: Rc<RefCell<Node>>) -> impl Iterator<Item = Rc<RefCell<Node>>> {
        NodeIter { node, dir }
    }
}

fn parse(input: &str) -> impl Iterator<Item = isize> + '_ {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
}

pub(crate) fn solve(input: &str) -> isize {
    let mut l = List::new(parse(input));
    let nodes = l.nodes.iter().cloned().collect_vec();
    for node in nodes {
        l.mix(node);
    }
    List::iter(Direction::Forwards, l.zero.clone())
        .skip(1)
        .chunks(1000)
        .into_iter()
        .take(3)
        .flat_map(|chunk| chunk.last())
        .map(|node| node.borrow().value)
        .sum::<isize>()
}

pub(crate) fn solve_2(input: &str) -> isize {
    let mut l = List::new(parse(input));
    l.scale(811589153);
    for _ in 0..10 {
        let nodes = l.nodes.iter().cloned().collect_vec();
        for node in nodes {
            l.mix(node);
        }
    }
    List::iter(Direction::Forwards, l.zero.clone())
        .skip(1)
        .chunks(1000)
        .into_iter()
        .take(3)
        .flat_map(|chunk| chunk.last())
        .map(|node| node.borrow().value)
        .sum::<isize>()
}

#[cfg(test)]
mod tests {

    use super::*;

    const EXAMPLE: &str = "
        1
        2
        -3
        3
        -2
        0
        4
    ";

    #[test]
    fn test_parse() {
        let list = List::new(parse(EXAMPLE));
        assert_eq!(&list, &List::new([1, 2, -3, 3, -2, 0, 4].into_iter()));
    }

    #[test]
    fn test_shifty() {
        let test = |a: &[isize], offset, b: &[isize]| {
            let mut l = List::new(a.iter().cloned());
            l.shift(l.zero.clone(), offset);
            assert_eq!(&l, &List::new(b.iter().cloned()));
        };
        test(&[0, 1, 2], 1, &[1, 0, 2]);
        test(&[0, 1, 2], 2, &[0, 1, 2]);
        test(&[0, 1, 2], 3, &[1, 0, 2]);
        test(&[0, 1, 2], -1, &[1, 0, 2]);
        test(&[0, 1, 2, 3, 4], 3, &[1, 2, 3, 0, 4]);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(EXAMPLE), 3);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(EXAMPLE), 1623178306);
    }
}
