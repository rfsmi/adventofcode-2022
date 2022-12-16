fn parse(input: &str) -> impl Iterator<Item = i32> {
    input
        .lines()
        .map(|l| l.trim())
        .fold(vec![vec![]], |mut v, line| {
            if line.is_empty() {
                v.push(vec![]);
            } else {
                let cals = line.parse::<i32>().unwrap();
                v.last_mut().unwrap().push(cals);
            }
            v
        })
        .into_iter()
        .map(|v| v.into_iter().sum())
        .filter(|&cals| cals != 0)
}

pub(crate) fn solve(input: &str) -> i32 {
    parse(input).max().unwrap()
}

pub(crate) fn solve_2(input: &str) -> i32 {
    let mut values: Vec<_> = parse(input).collect();
    values.sort();
    values.reverse();
    values.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = solve(
            "
            100
            200

            200
            300
            ",
        );
        assert_eq!(result, 500);
    }

    #[test]
    fn test_2() {
        let result = solve_2(
            "
            200

            100

            50

            200
            ",
        );
        assert_eq!(result, 500);
    }
}
