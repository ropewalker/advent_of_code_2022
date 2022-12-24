use crate::day21::Operator::*;
use crate::day21::Yell::*;
use aoc_runner_derive::{aoc, aoc_generator};
use num_rational::Rational64;
use num_traits::identities::*;
use std::collections::HashMap;

const ROOT_MONKEY_NAME: &str = "root";
const MY_NAME: &str = "humn";

#[derive(Clone, Hash, Eq, PartialEq)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, Hash, Eq, PartialEq)]
enum Yell {
    Number(i64),
    Operation(String, Operator, String),
}

#[derive(Clone, Eq, PartialEq)]
struct LinearPolynomial(Rational64, Rational64);

#[aoc_generator(day21)]
fn parse_input(input: &str) -> HashMap<String, Yell> {
    use aoc_parse::{parser, prelude::*};

    parser!(lines(
        name:string(lower+) ": "
        yell:{
            lhs:string(lower+) " "
            operator:{
                "+" => Add,
                "-" => Sub,
                "*" => Mul,
                "/" => Div,
            } " "
            rhs:string(lower+) =>
                Operation(lhs, operator, rhs),
            number:i64 => Number(number),
        } =>
            (name, yell)
    ))
    .parse(input)
    .unwrap()
    .into_iter()
    .collect()
}

fn solve_first_riddle(monkey_name: &String, monkey_jobs: &HashMap<String, Yell>) -> i64 {
    let yell = monkey_jobs.get(monkey_name).unwrap();

    match yell {
        Number(number) => *number,
        Operation(lhs, operator, rhs) => {
            let lhs = solve_first_riddle(lhs, monkey_jobs);
            let rhs = solve_first_riddle(rhs, monkey_jobs);
            match operator {
                Add => lhs + rhs,
                Sub => lhs - rhs,
                Mul => lhs * rhs,
                Div => lhs / rhs,
            }
        }
    }
}

#[aoc(day21, part1)]
fn part1(monkey_jobs: &HashMap<String, Yell>) -> i64 {
    solve_first_riddle(&ROOT_MONKEY_NAME.to_string(), monkey_jobs)
}

fn solve_second_riddle(monkey_jobs: &HashMap<String, Yell>) -> i64 {
    let yell = monkey_jobs.get(ROOT_MONKEY_NAME).unwrap();

    let (lhs, rhs) = match yell {
        Operation(lhs, _, rhs) => (
            solve_riddle_with_polynomials(lhs, monkey_jobs),
            solve_riddle_with_polynomials(rhs, monkey_jobs),
        ),
        Number(_) => unreachable!(),
    };

    if lhs.0 == rhs.0 {
        unreachable!()
    }

    ((rhs.1 - lhs.1) / (lhs.0 - rhs.0)).to_integer()
}

fn solve_riddle_with_polynomials(
    monkey_name: &String,
    monkey_jobs: &HashMap<String, Yell>,
) -> LinearPolynomial {
    let yell = monkey_jobs.get(monkey_name).unwrap();
    let zero = Rational64::zero();
    let one = Rational64::one();

    match yell {
        Number(number) => {
            if monkey_name == MY_NAME {
                LinearPolynomial(one, zero)
            } else {
                LinearPolynomial(zero, Rational64::from_integer(*number))
            }
        }
        Operation(lhs, operator, rhs) => {
            let lhs = solve_riddle_with_polynomials(lhs, monkey_jobs);
            let rhs = solve_riddle_with_polynomials(rhs, monkey_jobs);
            match operator {
                Add => LinearPolynomial(lhs.0 + rhs.0, lhs.1 + rhs.1),
                Sub => LinearPolynomial(lhs.0 - rhs.0, lhs.1 - rhs.1),
                Mul => {
                    if rhs.0 * lhs.0 != zero {
                        unreachable!()
                    }

                    LinearPolynomial(rhs.0 * lhs.1 + lhs.0 * rhs.1, rhs.1 * lhs.1)
                }
                Div => {
                    if rhs.0 != zero || rhs.1 == zero {
                        unreachable!()
                    }

                    LinearPolynomial(lhs.0 / rhs.1, lhs.1 / rhs.1)
                }
            }
        }
    }
}

#[aoc(day21, part2)]
fn part2(monkey_jobs: &HashMap<String, Yell>) -> i64 {
    solve_second_riddle(monkey_jobs)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"root: pppw + sjmn
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
hmdt: 32";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 152);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 301);
    }
}
