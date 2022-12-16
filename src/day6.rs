use itertools::Itertools;

pub(crate) fn solve(input: &str) -> usize {
    compute::<4>(input)
}

pub(crate) fn solve_2(input: &str) -> usize {
    compute::<14>(input)
}

fn compute<const N: usize>(input: &str) -> usize {
    let mut window = Vec::new();
    for (i, c) in input.chars().enumerate() {
        window.push(c);
        if window.len() > N {
            window.remove(0);
        }
        if window.iter().unique().count() == N {
            return i + 1;
        }
    }
    panic!("Didn't find marker");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(compute::<4>("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(compute::<4>("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(compute::<4>("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(compute::<4>("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_2() {
        assert_eq!(compute::<14>("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(compute::<14>("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    }
}
