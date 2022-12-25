fn to_base_5(mut base_10: i64) -> String {
    let mut result = String::new();
    while base_10 != 0 {
        let digit = match (base_10 + 2) % 5 - 2 {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!(),
        };
        result.insert(0, digit);
        base_10 = (base_10 + 2) / 5;
    }
    result
}

fn from_base_5(base_5: &str) -> i64 {
    let mut result = 0;
    for (place, c) in base_5.chars().rev().enumerate() {
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
    to_base_5(
        input
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(from_base_5)
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
    fn test_to_base_5() {
        assert_eq!(to_base_5(4890), "2=-1=0");
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(EXAMPLE), "2=-1=0")
    }
}
