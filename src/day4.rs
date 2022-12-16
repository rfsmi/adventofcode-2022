use itertools::Itertools;

#[derive(Debug, PartialEq)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn contains_range(&self, range: &Range) -> bool {
        self.start <= range.start && self.end >= range.end
    }

    fn contains_point(&self, point: usize) -> bool {
        self.start <= point && self.end >= point
    }

    fn overlaps_range(&self, range: &Range) -> bool {
        self.contains_range(range)
            || range.contains_range(self)
            || self.contains_point(range.start)
            || self.contains_point(range.end)
    }
}

trait CountTrue {
    type Item;

    fn count_true(self, f: impl Fn(Self::Item) -> bool) -> i32;
}

impl<T: Iterator> CountTrue for T {
    type Item = T::Item;

    fn count_true(self, f: impl FnMut(Self::Item) -> bool) -> i32 {
        self.map(f).map(|v| v as i32).sum()
    }
}

fn parse(input: &str) -> impl Iterator<Item = (Range, Range)> + '_ {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .flat_map(|l| l.split(','))
        .flat_map(|range| range.split('-'))
        .map(|s| s.parse::<usize>().unwrap())
        .tuples()
        .map(|(start, end)| Range { start, end })
        .tuples()
        .map(|(l, r)| (l, r))
}

pub(crate) fn solve(input: &str) -> i32 {
    parse(input).count_true(|(l, r)| l.contains_range(&r) || r.contains_range(&l))
}

pub(crate) fn solve_2(input: &str) -> i32 {
    parse(input).count_true(|(l, r)| l.overlaps_range(&r))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let result: Vec<_> = parse(
            "
            2-4,6-8
            2-3,4-5
            5-7,7-9
        ",
        )
        .collect();
        assert_eq!(
            result,
            vec![
                (Range { start: 2, end: 4 }, Range { start: 6, end: 8 }),
                (Range { start: 2, end: 3 }, Range { start: 4, end: 5 }),
                (Range { start: 5, end: 7 }, Range { start: 7, end: 9 }),
            ]
        );
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve("2-4,2-4"), 1);
        assert_eq!(solve("2-6,6-8"), 0);
        assert_eq!(solve("2-4,6-8"), 0);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2("2-4,2-4"), 1);
        assert_eq!(solve_2("2-6,6-8"), 1);
        assert_eq!(solve_2("2-4,6-8"), 0);
    }
}
