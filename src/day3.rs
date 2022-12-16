use std::collections::HashSet;

use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Item(char);

impl Item {
    fn priority(self) -> i32 {
        match self.0 {
            'a'..='z' => self.0 as i32 - 'a' as i32 + 1,
            'A'..='Z' => self.0 as i32 - 'A' as i32 + 27,
            _ => panic!("Unknown item {self:?}"),
        }
    }
}

fn parse(input: &str) -> impl Iterator<Item = Vec<Item>> + '_ {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.bytes().map(|b| Item(b as char)))
        .map(|items| -> Vec<_> { items.collect() })
}

fn pairs(
    rucksacks: impl Iterator<Item = Vec<Item>>,
) -> impl Iterator<Item = impl IntoIterator<Item = Item>> {
    rucksacks.flat_map(|mut lhs| {
        let rhs = lhs.split_off(lhs.len() / 2);
        [lhs, rhs].into_iter()
    })
}

pub(crate) fn solve(input: &str) -> i32 {
    let rucksack_pairs = pairs(parse(input));
    get_shared_item(rucksack_pairs)
        .map(|item| item.priority())
        .sum()
}

fn get_shared_item(
    rucksacks: impl Iterator<Item = impl IntoIterator<Item = Item>>,
) -> impl Iterator<Item = Item> {
    rucksacks
        .map(|rs| -> HashSet<Item> { rs.into_iter().collect() })
        .tuples()
        .map(|(a, b)| -> Item { a.intersection(&b).copied().next().unwrap() })
}

fn get_shared_item_2(
    rucksacks: impl Iterator<Item = impl IntoIterator<Item = Item>>,
) -> impl Iterator<Item = Item> {
    rucksacks
        .map(|rs| -> HashSet<Item> { rs.into_iter().collect() })
        .tuples()
        .map(|(a, b, c)| -> Item {
            a.into_iter()
                .filter(|item| b.contains(item))
                .filter(|item| c.contains(item))
                .next()
                .unwrap()
        })
}

pub(crate) fn solve_2(input: &str) -> i32 {
    let rucksacks = parse(input);
    get_shared_item_2(rucksacks)
        .map(|item| item.priority())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let result: Vec<_> = parse(
            "
            aA
            bbBB
        ",
        )
        .collect();
        assert_eq!(
            result,
            vec![
                vec![Item('a'), Item('A')],
                vec![Item('b'), Item('b'), Item('B'), Item('B')],
            ]
        );
    }

    #[test]
    fn test_priority() {
        assert_eq!(Item('a').priority(), 1);
        assert_eq!(Item('z').priority(), 26);
        assert_eq!(Item('A').priority(), 27);
        assert_eq!(Item('Z').priority(), 52);
    }

    #[test]
    fn test_solve_1() {
        let rucksacks = pairs(parse("aaba"));
        let items: Vec<_> = get_shared_item(rucksacks).collect();
        assert_eq!(items, vec![Item('a')]);
    }

    #[test]
    fn test_solve_2() {
        let rucksacks = parse(
            "
            abc
            dae
            fga

            abc
            dab
            fgb
        ",
        );
        let items: Vec<_> = get_shared_item_2(rucksacks).collect();
        assert_eq!(items, vec![Item('a'), Item('b')]);
    }
}
