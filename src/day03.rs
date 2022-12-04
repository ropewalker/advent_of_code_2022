use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

type ItemType = char;
struct Rucksack(Vec<ItemType>);

const LOWERCASE_A_PRIORITY: u32 = 1;
const UPPERCASE_A_PRIORITY: u32 = 27;

fn priority(item_type: &ItemType) -> u32 {
    if item_type.is_ascii_lowercase() {
        LOWERCASE_A_PRIORITY + *item_type as u32 - 'a' as u32
    } else {
        UPPERCASE_A_PRIORITY + *item_type as u32 - 'A' as u32
    }
}

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|rucksack| Rucksack(rucksack.chars().collect()))
        .collect()
}

#[aoc(day3, part1)]
fn part1(rucksacks: &[Rucksack]) -> u32 {
    rucksacks
        .iter()
        .map(|rucksack| {
            let first_compartment_item_types: HashSet<_> =
                rucksack.0[..rucksack.0.len() / 2].iter().copied().collect();
            let second_compartment_item_types: HashSet<_> =
                rucksack.0[rucksack.0.len() / 2..].iter().copied().collect();

            priority(
                first_compartment_item_types
                    .intersection(&second_compartment_item_types)
                    .next()
                    .unwrap(),
            )
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2(rucksacks: &[Rucksack]) -> u32 {
    rucksacks
        .chunks_exact(3)
        .map(|group| {
            priority(
                group
                    .iter()
                    .map(|rucksack| rucksack.0.iter().copied().collect::<HashSet<_>>())
                    .reduce(|common_item_types, rucksack_item_types| {
                        common_item_types
                            .intersection(&rucksack_item_types)
                            .copied()
                            .collect()
                    })
                    .unwrap()
                    .iter()
                    .next()
                    .unwrap(),
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 157);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 70);
    }
}
