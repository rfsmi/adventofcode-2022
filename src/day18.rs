use std::collections::{BTreeSet, HashSet};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Cube {
    x: i8,
    y: i8,
    z: i8,
}

impl Cube {
    fn new(x: i8, y: i8, z: i8) -> Self {
        Self { x, y, z }
    }

    fn adjacent_cubes(&self) -> [Cube; 6] {
        [
            Cube::new(self.x + 1, self.y, self.z),
            Cube::new(self.x - 1, self.y, self.z),
            Cube::new(self.x, self.y + 1, self.z),
            Cube::new(self.x, self.y - 1, self.z),
            Cube::new(self.x, self.y, self.z + 1),
            Cube::new(self.x, self.y, self.z - 1),
        ]
    }
}

struct BBox {
    min: Cube,
    max: Cube,
}

impl BBox {
    fn new(cube: Cube) -> Self {
        Self {
            min: cube,
            max: Cube::new(cube.x + 1, cube.y + 1, cube.z + 1),
        }
    }

    fn extend(&mut self, cube: Cube) {
        self.min.x = self.min.x.min(cube.x);
        self.min.y = self.min.y.min(cube.y);
        self.min.z = self.min.z.min(cube.z);
        self.max.x = self.max.x.max(cube.x + 1);
        self.max.y = self.max.y.max(cube.y + 1);
        self.max.z = self.max.z.max(cube.z + 1);
    }

    fn loosely_contains(&self, cube: Cube) -> bool {
        cube.x >= self.min.x - 1
            && cube.y >= self.min.y - 1
            && cube.z >= self.min.z - 1
            && cube.x < self.max.x + 1
            && cube.y < self.max.y + 1
            && cube.z < self.max.z + 1
    }
}

struct Droplet {
    cubes: BTreeSet<Cube>,
    total_surface_area: usize,
    bbox: Option<BBox>,
}

impl Droplet {
    fn new() -> Self {
        Self {
            cubes: BTreeSet::new(),
            total_surface_area: 0,
            bbox: None,
        }
    }

    fn add_cube(&mut self, cube: Cube) {
        self.cubes.insert(cube);
        self.total_surface_area += 6;
        for adj_cube in cube.adjacent_cubes() {
            if self.cubes.contains(&adj_cube) {
                self.total_surface_area -= 2;
            }
        }
        self.bbox = match self.bbox.take() {
            None => Some(BBox::new(cube)),
            Some(mut bbox) => {
                bbox.extend(cube);
                Some(bbox)
            }
        };
    }

    fn exterior_surface_area(&self) -> usize {
        let bbox = match &self.bbox {
            None => return 0,
            Some(bbox) => bbox,
        };
        let mut queue: Vec<_> = [bbox.max].into_iter().collect();
        let mut seen: HashSet<_> = [bbox.max].into_iter().collect();
        let mut result = 0;
        while let Some(parent) = queue.pop() {
            for cube in parent.adjacent_cubes() {
                if self.cubes.contains(&cube) {
                    result += 1;
                } else if bbox.loosely_contains(cube) && seen.insert(cube) {
                    queue.push(cube);
                }
            }
        }
        result
    }
}

fn parse(input: &str) -> impl Iterator<Item = Cube> + '_ {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .flat_map(|l| l.split(','))
        .map(|s| s.parse().unwrap())
        .tuples()
        .map(|(x, y, z)| Cube::new(x, y, z))
}

pub(crate) fn solve(input: &str) -> usize {
    let mut droplet = Droplet::new();
    for cube in parse(input) {
        droplet.add_cube(cube);
    }
    droplet.total_surface_area
}

pub(crate) fn solve_2(input: &str) -> usize {
    let mut droplet = Droplet::new();
    for cube in parse(input) {
        droplet.add_cube(cube);
    }
    droplet.exterior_surface_area()
}

#[cfg(test)]
mod tests {

    use super::*;

    const EXAMPLE: &str = "
        2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5
    ";

    #[test]
    fn test_solve() {
        assert_eq!(solve(EXAMPLE), 64);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(EXAMPLE), 58);
    }
}
