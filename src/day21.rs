use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Write},
    rc::Rc,
};

use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
enum Monkey<'a> {
    Immediate(isize),
    Delayed(&'a str, &'a str, Op),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Op {
    Mul,
    Div,
    Add,
    Sub,
}

impl Op {
    fn eval(self, lhs: isize, rhs: isize) -> isize {
        match self {
            Op::Add => lhs + rhs,
            Op::Sub => lhs - rhs,
            Op::Mul => lhs * rhs,
            Op::Div => lhs / rhs,
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Op::Add => '+',
            Op::Sub => '-',
            Op::Mul => '*',
            Op::Div => '/',
        })
    }
}

enum SimplifiedExpr {
    LhsExpr(Rc<SimplifiedExpr>, isize, Op),
    RhsExpr(isize, Rc<SimplifiedExpr>, Op),
    Unknown,
}

impl SimplifiedExpr {
    fn find_unknown(&self, accum: isize) -> isize {
        match self {
            Self::Unknown => accum,
            Self::LhsExpr(lhs, rhs, op) => {
                let accum = match op {
                    Op::Mul => Op::Div.eval(accum, *rhs),
                    Op::Div => Op::Mul.eval(accum, *rhs),
                    Op::Add => Op::Sub.eval(accum, *rhs),
                    Op::Sub => Op::Add.eval(accum, *rhs),
                };
                lhs.find_unknown(accum)
            }
            Self::RhsExpr(lhs, rhs, op) => {
                let accum = match op {
                    Op::Mul => Op::Div.eval(accum, *lhs),
                    Op::Div => Op::Div.eval(*lhs, accum),
                    Op::Add => Op::Sub.eval(accum, *lhs),
                    Op::Sub => Op::Sub.eval(*lhs, accum),
                };
                rhs.find_unknown(accum)
            }
        }
    }
}

enum Expr {
    BinaryOperation(Rc<Expr>, Rc<Expr>, Op),
    Literal(isize),
    Unknown,
}

impl Expr {
    fn simplify(&self) -> Rc<SimplifiedExpr> {
        let Self::BinaryOperation(lhs, rhs, op) = self else {
            if let Self::Unknown = self {
                return Rc::new(SimplifiedExpr::Unknown);
            }
            panic!("Can't simplify this expression (unexpected literal)");
        };
        Rc::new(match (lhs.as_ref(), rhs.as_ref()) {
            (expr, Expr::Literal(value)) => SimplifiedExpr::LhsExpr(expr.simplify(), *value, *op),
            (Expr::Literal(value), expr) => SimplifiedExpr::RhsExpr(*value, expr.simplify(), *op),
            _ => panic!("Can't simplify this expression (too complex)"),
        })
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => f.write_char('x'),
            Self::Literal(value) => f.write_fmt(format_args!("{}", value)),
            Self::BinaryOperation(lhs, rhs, op) => {
                f.write_char('(')?;
                lhs.fmt(f)?;
                f.write_char(' ')?;
                op.fmt(f)?;
                f.write_char(' ')?;
                rhs.fmt(f)?;
                f.write_char(')')
            }
        }
    }
}

fn topsort<'a>(monkeys: &HashMap<&'a str, Monkey<'a>>) -> Vec<&'a str> {
    let mut graph: HashMap<&str, HashSet<&str>> =
        monkeys.keys().map(|k| (*k, HashSet::new())).collect();
    let mut backward_graph = graph.clone();
    for (name, op) in monkeys {
        if let Monkey::Delayed(lhs, rhs, _) = op {
            graph.get_mut(name).unwrap().extend([lhs, rhs]);
            backward_graph.get_mut(lhs).unwrap().insert(name);
            backward_graph.get_mut(rhs).unwrap().insert(name);
        }
    }

    let mut stack: Vec<&str> = graph
        .iter()
        .filter(|(_, edges)| edges.is_empty())
        .map(|(name, _)| *name)
        .collect();

    let mut result = Vec::new();
    while let Some(node) = stack.pop() {
        result.push(node);
        for dependent in &backward_graph[node] {
            let edges = graph.get_mut(dependent).unwrap();
            edges.remove(node);
            if edges.is_empty() {
                stack.push(dependent);
            }
        }
    }
    result
}

fn parse(input: &str) -> impl Iterator<Item = (&str, Monkey)> {
    let re = Regex::new(r"^(\w+): (?:(\w+) (.) (\w+)|(\d+))$").unwrap();
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(move |l| {
            let cap = re.captures(l).unwrap();
            let solution = if let Some(literal) = cap.get(5) {
                Monkey::Immediate(literal.as_str().parse().unwrap())
            } else {
                let op = match &cap[3] {
                    "*" => Op::Mul,
                    "/" => Op::Div,
                    "+" => Op::Add,
                    "-" => Op::Sub,
                    s => panic!("Unknown operation: {}", s),
                };
                let lhs = cap.get(2).unwrap().as_str();
                let rhs = cap.get(4).unwrap().as_str();
                Monkey::Delayed(lhs, rhs, op)
            };
            let name = cap.get(1).unwrap().as_str();
            (name, solution)
        })
}

pub(crate) fn solve(input: &str) -> isize {
    let monkeys: HashMap<_, _> = parse(input).collect();
    let mut values = HashMap::new();
    for name in topsort(&monkeys) {
        let value = match &monkeys[name] {
            Monkey::Immediate(v) => *v,
            Monkey::Delayed(lhs, rhs, op) => op.eval(values[lhs], values[rhs]),
        };
        values.insert(name, value);
    }
    values["root"]
}

fn get_expression(input: &str) -> Rc<Expr> {
    let monkeys: HashMap<_, _> = parse(input).collect();
    let mut expressions: HashMap<&str, Rc<Expr>> = HashMap::new();
    for name in topsort(&monkeys) {
        let expr = match (name, &monkeys[name]) {
            ("humn", _) => Expr::Unknown,
            ("root", Monkey::Delayed(lhs, rhs, _)) => {
                let lhs = &expressions[lhs];
                let rhs = &expressions[rhs];
                Expr::BinaryOperation(Rc::clone(lhs), Rc::clone(rhs), Op::Sub)
            }
            (_, Monkey::Immediate(v)) => Expr::Literal(*v),
            (_, Monkey::Delayed(lhs, rhs, op)) => {
                let lhs = &expressions[lhs];
                let rhs = &expressions[rhs];
                if let (Expr::Literal(lhs), Expr::Literal(rhs)) = (lhs.as_ref(), rhs.as_ref()) {
                    Expr::Literal(op.eval(*lhs, *rhs))
                } else {
                    Expr::BinaryOperation(Rc::clone(lhs), Rc::clone(rhs), *op)
                }
            }
        };
        expressions.insert(name, Rc::new(expr));
    }
    Rc::clone(&expressions["root"])
}

pub(crate) fn solve_2(input: &str) -> isize {
    let expr = get_expression(input);
    println!("{expr}");
    expr.simplify().find_unknown(0)
}

#[cfg(test)]
mod tests {

    use itertools::Itertools;

    use super::*;

    const EXAMPLE: &str = "
        root: pppw + sjmn
        dbpl: 5
        cczh: sllz + lgvd
        zczc: 2
        ptdq: humn - dvpt
        dvpt: 3
        lfqf: 4
        humn: 5
        ljgn: 2
        sjmn: drzm * dbpl
        sllz: 4
        pppw: cczh / lfqf
        lgvd: ljgn * ptdq
        drzm: hmdt - zczc
        hmdt: 32
    ";

    #[test]
    fn test_parse() {
        let ops = parse(EXAMPLE).take(3).collect_vec();
        assert_eq!(
            ops,
            vec![
                (
                    "root".into(),
                    Monkey::Delayed("pppw".into(), "sjmn".into(), Op::Add)
                ),
                ("dbpl".into(), Monkey::Immediate(5)),
                (
                    "cczh".into(),
                    Monkey::Delayed("sllz".into(), "lgvd".into(), Op::Add)
                ),
            ]
        );
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(EXAMPLE), 152);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(EXAMPLE), 301);
    }
}
