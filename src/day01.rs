use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|inventory| {
            inventory
                .lines()
                .map(|item| item.parse().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day1, part1)]
fn part1(calories: &[Vec<u32>]) -> u32 {
    calories
        .iter()
        .map(|inventory| inventory.iter().sum())
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
fn part2(calories: &[Vec<u32>]) -> u32 {
    calories
        .iter()
        .map(|inventory| inventory.iter().sum())
        .fold(vec![0, 0, 0], |mut top_three, calories| {
            if calories > top_three[0] {
                top_three[0] = calories;
                top_three.sort_unstable();
            }

            top_three
        })
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 24_000);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 45_000);
    }
}
