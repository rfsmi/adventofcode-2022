use itertools::Itertools;

#[derive(Debug, PartialEq)]
struct Instruction1 {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, PartialEq)]
struct Instruction2 {
    count: usize,
    from: usize,
    to: usize,
}

trait Instruction {
    fn new(count: usize, from: usize, to: usize) -> Self;
    fn apply(&self, state: &mut State);
}

impl Instruction for Instruction1 {
    fn new(count: usize, from: usize, to: usize) -> Self {
        Self { count, from, to }
    }

    fn apply(&self, state: &mut State) {
        // Account for 1-based indexes
        let (from, to) = (self.from - 1, self.to - 1);
        let from_vec = &mut state[from];
        let mut moving_vec = from_vec.split_off(from_vec.len() - self.count);
        moving_vec.reverse();
        state[to].extend(moving_vec);
    }
}

impl Instruction for Instruction2 {
    fn new(count: usize, from: usize, to: usize) -> Self {
        Self { count, from, to }
    }

    fn apply(&self, state: &mut State) {
        // Account for 1-based indexes
        let (from, to) = (self.from - 1, self.to - 1);
        let from_vec = &mut state[from];
        let moving_vec = from_vec.split_off(from_vec.len() - self.count);
        state[to].extend(moving_vec);
    }
}

type State = Vec<Vec<char>>;

fn parse_instructions<'a, I: Instruction>(
    lines: impl Iterator<Item = &'a str> + 'a,
) -> impl Iterator<Item = I> + 'a {
    lines
        .flat_map(|l| l.split(|c| !char::is_numeric(c)))
        .filter(|n| !n.is_empty())
        .map(|n| n.parse::<usize>().unwrap())
        .tuples()
        .map(|(count, from, to)| I::new(count, from, to))
}

fn parse_state<'a, 'b>(lines: &'a mut impl Iterator<Item = &'b str>) -> State {
    let mut state_lines = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        state_lines.push(line);
    }
    state_lines.reverse();
    state_lines
        .iter()
        .flat_map(|l| l.chars().enumerate())
        .filter(|(_, c)| c.is_alphabetic())
        .into_group_map()
        .into_iter()
        .sorted_by_key(|(i, _)| *i)
        .map(|(_, v)| v)
        .collect()
}

fn parse<I: Instruction + 'static>(input: &str) -> (State, impl Iterator<Item = I> + '_) {
    let mut lines = input.lines().map(|l| l.trim());
    let setup = parse_state(&mut lines);
    let instructions = parse_instructions(lines);
    (setup, instructions)
}

fn compute<I: Instruction + 'static>(input: &str) -> State {
    // Initial state
    let (state, instructions) = parse::<I>(input);
    // State after applying instructions
    instructions.fold(state, |mut s, i| {
        i.apply(&mut s);
        s
    })
}

pub(crate) fn solve(input: &str) -> String {
    compute::<Instruction1>(input)
        .iter()
        .map(|col| col.last().unwrap_or(&' '))
        .collect()
}

pub(crate) fn solve_2(input: &str) -> String {
    compute::<Instruction2>(input)
        .iter()
        .map(|col| col.last().unwrap_or(&' '))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_parse() {
        let input = "
        move 1 from 1 to 9
        move 6 from 2 to 1
        ";
        let instructions = parse_instructions::<Instruction1>(input.lines()).collect_vec();
        assert_eq!(
            instructions,
            vec![
                Instruction1 {
                    count: 1,
                    from: 1,
                    to: 9
                },
                Instruction1 {
                    count: 6,
                    from: 2,
                    to: 1
                },
            ]
        );
    }

    #[test]
    fn test_state_parse() {
        let mut input = "\
            [B]     [D]
            [H] [M] [N]
             1   2   3

            etc 
        "
        .lines()
        .map(|l| l.trim());
        let state = parse_state(&mut input);
        assert_eq!(input.next(), Some("etc"));
        assert_eq!(state, vec![vec!['H', 'B'], vec!['M'], vec!['N', 'D'],]);
    }

    #[test]
    fn test_solve() {
        // Initial state
        let input = "\
            [B]     [D]
            [H] [M] [N]
             1   2   3

            move 1 from 2 to 3
            move 2 from 3 to 1
        ";
        let state = compute::<Instruction1>(input);
        assert_eq!(state, vec![vec!['H', 'B', 'M', 'D'], vec![], vec!['N'],]);
        assert_eq!(solve(input), "D N");
    }
}
