use regex::Regex;

struct EmptiesIter<'a> {
    ranges: &'a [(isize, isize)],
    current: isize,
    end: isize,
}

impl<'a> Iterator for EmptiesIter<'a> {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current < self.end {
            if let Some(&(start, end)) = self.ranges.first() {
                if self.current < start {
                    // Do nothing
                } else if self.current < end {
                    self.current = end;
                    continue;
                } else {
                    self.ranges = &self.ranges[1..];
                    continue;
                }
            }
            let result = self.current;
            self.current += 1;
            return Some(result);
        }
        None
    }
}

#[derive(Clone)]
struct Ranges {
    ranges: Vec<(isize, isize)>,
}

impl Ranges {
    fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    fn add(&mut self, mut range: (isize, isize)) {
        let mut i = 0;
        while i < self.ranges.len() {
            let other_range = self.ranges[i];
            if range.1 < other_range.0 {
                // ..aaa.bbb..
                break;
            }
            if range.0 <= other_range.1 {
                // ..bbbaaa..
                self.ranges.remove(i);
                range.0 = range.0.min(other_range.0);
                range.1 = range.1.max(other_range.1);
                continue;
            }
            i += 1;
        }
        self.ranges.insert(i, range);
    }

    fn empties(&self, range: (isize, isize)) -> impl Iterator<Item = isize> + '_ {
        EmptiesIter {
            ranges: &self.ranges,
            current: range.0,
            end: range.1,
        }
    }

    fn count(&self) -> usize {
        self.ranges.iter().map(|(l, r)| (r - l) as usize).sum()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Area {
    center: (isize, isize),
    radius: isize,
}

fn parse(input: &str) -> impl Iterator<Item = Area> + '_ {
    let re = Regex::new(r"^.*=(-?\d+).*=(-?\d+).*=(-?\d+).*=(-?\d+)$").unwrap();
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(move |l| {
            let cap = re.captures(l).unwrap();
            let sensor = (
                cap[1].parse::<isize>().unwrap(),
                cap[2].parse::<isize>().unwrap(),
            );
            let beacon = (
                cap[3].parse::<isize>().unwrap(),
                cap[4].parse::<isize>().unwrap(),
            );
            Area {
                center: sensor,
                radius: (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs(),
            }
        })
}

fn compute<const N: isize>(input: &str) -> usize {
    parse(input)
        .filter_map(|area| match (area.center.1 - N).abs() {
            y_dist if y_dist < area.radius => {
                let x_dist = area.radius - y_dist;
                Some((area.center.0 - x_dist, area.center.0 + x_dist))
            }
            _ => None,
        })
        .fold(Ranges::new(), |mut ranges, r| {
            ranges.add(r);
            ranges
        })
        .count()
}

fn compute_2<const MAX: isize>(input: &str) -> isize {
    let mut rows = vec![Ranges::new(); MAX as usize];
    for area in parse(input) {
        let y_min = (area.center.1 - area.radius).max(0);
        let y_max = (area.center.1 + area.radius).min(MAX);
        for y in y_min..y_max {
            let y_dist = (area.center.1 - y).abs();
            let x_dist = area.radius - y_dist;
            rows[y as usize].add((area.center.0 - x_dist, area.center.0 + x_dist + 1));
        }
    }
    for (y, ranges) in rows.iter().enumerate() {
        if let Some(x) = ranges.empties((0, MAX)).next() {
            return x * 4000000 + y as isize;
        }
    }
    panic!()
}

pub(crate) fn solve(input: &str) -> usize {
    compute::<2000000>(input)
}

pub(crate) fn solve_2(input: &str) -> isize {
    compute_2::<4000000>(input)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    const EXAMPLE: &str = "
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3
    ";

    #[test]
    fn test_parse() {
        let area = parse(EXAMPLE).next().unwrap();
        assert_eq!(
            area,
            Area {
                center: (2, 18),
                radius: 7
            }
        );
    }

    #[test]
    fn test_ranges() {
        let mut ranges = Ranges::new();
        ranges.add((10, 20));
        assert_eq!(ranges.ranges, vec![(10, 20)]);
        ranges.add((15, 25));
        assert_eq!(ranges.ranges, vec![(10, 25)]);
        ranges.add((0, 1));
        assert_eq!(ranges.ranges, vec![(0, 1), (10, 25)]);
        ranges.add((9, 10));
        assert_eq!(ranges.ranges, vec![(0, 1), (9, 25)]);
        ranges.add((8, 26));
        assert_eq!(ranges.ranges, vec![(0, 1), (8, 26)]);
        ranges.add((27, 29));
        assert_eq!(ranges.ranges, vec![(0, 1), (8, 26), (27, 29)]);
        ranges.add((-10, 40));
        assert_eq!(ranges.ranges, vec![(-10, 40)]);
    }

    #[test]
    fn test_empties() {
        let mut ranges = Ranges::new();
        ranges.add((5, 10));
        ranges.add((11, 12));
        assert_eq!(ranges.empties((4, 13)).collect_vec(), vec![4, 10, 12]);
    }

    #[test]
    fn test_solve() {
        assert_eq!(compute::<10>(EXAMPLE), 26);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(compute_2::<20>(EXAMPLE), 56000011);
    }
}
