fn to_snafu(mut num: i64) -> String {
    let mut result = String::new();
    while num != 0 {
        let digit = match (num + 2) % 5 - 2 {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!(),
        };
        result.insert(0, digit);
        num = (num + 2) / 5;
    }
    result
}

fn from_snafu(snafu: &str) -> i64 {
    let mut result = 0;
    for (place, c) in snafu.chars().rev().enumerate() {
        let digit = match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!(),
        };
        result += digit * 5i64.pow(place as u32);
    }
    result
}

pub(crate) fn solve(input: &str) -> String {
    to_snafu(
        input
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(from_snafu)
            .sum(),
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    const EXAMPLE: &str = "
        1=-0-2
        12111
        2=0=
        21
        2=01
        111
        20012
        112
        1=-1=
        1-12
        12
        1=
        122
    ";

    #[test]
    fn test_solve() {
        assert_eq!(solve(EXAMPLE), "2=-1=0")
    }
}
