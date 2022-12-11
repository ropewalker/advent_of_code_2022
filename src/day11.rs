use crate::day11::Operand::*;
use crate::day11::Operator::*;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::vec_deque::VecDeque;

type WorryLevel = u64;
type MonkeyId = usize;

#[derive(Copy, Clone)]
enum Operand {
    Number(u64),
    Old,
}

#[derive(Copy, Clone)]
enum Operator {
    Add,
    Mul,
}

#[derive(Copy, Clone)]
struct Operation {
    lhs: Operand,
    operator: Operator,
    rhs: Operand,
}

impl Operation {
    fn execute(&self, old: u64) -> u64 {
        let lhs = match self.lhs {
            Number(operand) => operand,
            Old => old,
        };

        let rhs = match self.rhs {
            Number(operand) => operand,
            Old => old,
        };

        match self.operator {
            Add => lhs + rhs,
            Mul => lhs * rhs,
        }
    }
}

#[derive(Copy, Clone)]
struct Test {
    divisible_by: u64,
    if_true: MonkeyId,
    if_false: MonkeyId,
}

#[derive(Clone)]
struct Monkey {
    starting_items: VecDeque<WorryLevel>,
    operation: Operation,
    test: Test,
}

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey| {
            let mut arguments = monkey.lines().skip(1);

            Monkey {
                starting_items: arguments
                    .next()
                    .unwrap()
                    .split_once("  Starting items: ")
                    .unwrap()
                    .1
                    .split(", ")
                    .map(|token| token.parse().unwrap())
                    .collect(),
                operation: {
                    let mut tokens = arguments
                        .next()
                        .unwrap()
                        .split_once("  Operation: new = ")
                        .unwrap()
                        .1
                        .split_ascii_whitespace();

                    Operation {
                        lhs: if let Ok(worry_level) = tokens.next().unwrap().parse::<u64>() {
                            Number(worry_level)
                        } else {
                            Old
                        },
                        operator: match tokens.next().unwrap() {
                            "+" => Add,
                            "*" => Mul,
                            _ => unreachable!(),
                        },
                        rhs: if let Ok(worry_level) = tokens.next().unwrap().parse::<u64>() {
                            Number(worry_level)
                        } else {
                            Old
                        },
                    }
                },
                test: Test {
                    divisible_by: arguments
                        .next()
                        .unwrap()
                        .split_once("  Test: divisible by ")
                        .unwrap()
                        .1
                        .parse()
                        .unwrap(),
                    if_true: arguments
                        .next()
                        .unwrap()
                        .split_once("    If true: throw to monkey ")
                        .unwrap()
                        .1
                        .parse()
                        .unwrap(),
                    if_false: arguments
                        .next()
                        .unwrap()
                        .split_once("    If false: throw to monkey ")
                        .unwrap()
                        .1
                        .parse()
                        .unwrap(),
                },
            }
        })
        .collect()
}

fn monkey_business(monkeys: &[Monkey], worry_relief: bool, number_of_rounds: usize) -> usize {
    let mut inspected_items: Vec<usize> = vec![0; monkeys.len()];
    let mut monkeys: Vec<_> = monkeys.to_vec();

    let common_multiple: u64 = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible_by)
        .product();

    for _round in 0..number_of_rounds {
        for id in 0..monkeys.len() {
            let monkey = &mut monkeys[id];
            inspected_items[id] += monkey.starting_items.len();

            let if_true_id = monkey.test.if_true;
            let mut if_true_items = VecDeque::with_capacity(monkey.starting_items.len());

            let if_false_id = monkey.test.if_false;
            let mut if_false_items = VecDeque::with_capacity(monkey.starting_items.len());

            while !monkey.starting_items.is_empty() {
                let mut worry_level = monkey.starting_items.pop_front().unwrap();
                worry_level = monkey.operation.execute(worry_level);

                if worry_relief {
                    worry_level /= 3;
                } else {
                    worry_level %= common_multiple;
                }

                if worry_level % monkey.test.divisible_by == 0 {
                    if_true_items.push_back(worry_level);
                } else {
                    if_false_items.push_back(worry_level);
                }
            }

            monkeys[if_true_id]
                .starting_items
                .append(&mut if_true_items);
            monkeys[if_false_id]
                .starting_items
                .append(&mut if_false_items);
        }
    }

    inspected_items.sort_unstable();
    inspected_items.iter().rev().take(2).product()
}

#[aoc(day11, part1)]
fn part1(monkeys: &[Monkey]) -> usize {
    monkey_business(monkeys, true, 20)
}

#[aoc(day11, part2)]
fn part2(monkeys: &[Monkey]) -> usize {
    monkey_business(monkeys, false, 10_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 10_605);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 2_713_310_158);
    }
}
