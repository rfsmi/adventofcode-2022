use std::iter::{once, repeat};

use itertools::Itertools;

#[derive(Clone, Copy)]
enum Instruction {
    Addx(isize),
    Noop,
}

fn parse(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            if l.starts_with("noop") {
                Instruction::Noop
            } else {
                let (_, num) = l.split_ascii_whitespace().collect_tuple().unwrap();
                Instruction::Addx(num.parse::<isize>().unwrap())
            }
        })
}

fn x_reg(instructions: impl Iterator<Item = Instruction>) -> impl Iterator<Item = isize> {
    let mut x = 1;
    instructions
        .flat_map(|instr| {
            repeat(Instruction::Noop)
                .take(match instr {
                    Instruction::Noop => 0,
                    Instruction::Addx(_) => 1,
                })
                .chain(once(instr))
        })
        .map(move |instr| match instr {
            Instruction::Noop => x,
            Instruction::Addx(add) => {
                let x_pre_add = x;
                x += add;
                x_pre_add
            }
        })
}

pub(crate) fn solve(input: &str) -> usize {
    x_reg(parse(input))
        .enumerate()
        .fold(0, |mut strength, (i, x)| {
            let i = i as isize + 1;
            if (i - 20) % 40 == 0 {
                strength += i * x;
            }
            strength
        }) as usize
}

pub(crate) fn solve_2(input: &str) -> String {
    x_reg(parse(input))
        .enumerate()
        .flat_map(|(i, x)| {
            once("\n")
                .take((i % 40 == 0) as usize)
                .chain(if (i as isize % 40 - x).abs() <= 1 {
                    once("#")
                } else {
                    once(" ")
                })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
