use aoc_runner_derive::{aoc, aoc_generator};

type Crate = char;
type Stack = Vec<Crate>;

struct RearrangementStep {
    moved_count: usize,
    from: usize,
    to: usize,
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> (Vec<Stack>, Vec<RearrangementStep>) {
    let (stacks, rearrangement_procedure) = input.split_once("\n\n").unwrap();

    let layers: Vec<_> = stacks.lines().collect();

    let last_stack_number = layers[0].len() / 4 + 1;
    let mut stacks: Vec<Stack> = vec![Vec::with_capacity(layers.len() - 1); last_stack_number + 1];

    for layer_number in (0..layers.len() - 1).rev() {
        let layer = layers[layer_number];

        for (stack_number, stack) in stacks
            .iter_mut()
            .enumerate()
            .skip(1)
            .take(last_stack_number)
        {
            let supply_crate = layer.chars().nth((stack_number - 1) * 4 + 1).unwrap();

            if !supply_crate.is_ascii_whitespace() {
                stack.push(supply_crate);
            }
        }
    }

    let rearrangement_procedure = rearrangement_procedure
        .lines()
        .map(|rearrangement_step| {
            let mut numbers = rearrangement_step
                .split_ascii_whitespace()
                .filter_map(|token| token.parse().ok());

            RearrangementStep {
                moved_count: numbers.next().unwrap(),
                from: numbers.next().unwrap(),
                to: numbers.next().unwrap(),
            }
        })
        .collect();

    (stacks, rearrangement_procedure)
}

#[aoc(day5, part1)]
fn part1((stacks, rearrangement_procedure): &(Vec<Stack>, Vec<RearrangementStep>)) -> String {
    let mut stacks = stacks.to_owned();

    for rearrangement_step in rearrangement_procedure {
        for _ in 0..rearrangement_step.moved_count {
            let supply_crate = stacks[rearrangement_step.from].pop().unwrap();
            stacks[rearrangement_step.to].push(supply_crate);
        }
    }

    stacks
        .iter()
        .filter_map(|stack| stack.iter().last())
        .collect()
}

#[aoc(day5, part2)]
fn part2((stacks, rearrangement_procedure): &(Vec<Stack>, Vec<RearrangementStep>)) -> String {
    let mut stacks = stacks.to_owned();

    for rearrangement_step in rearrangement_procedure {
        let from_stack = &mut stacks[rearrangement_step.from];
        let mut moved_crates =
            from_stack.split_off(from_stack.len() - rearrangement_step.moved_count);

        stacks[rearrangement_step.to].append(&mut moved_crates);
    }

    stacks
        .iter()
        .filter_map(|stack| stack.iter().last())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), "CMZ".to_string());
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), "MCD".to_string());
    }
}
