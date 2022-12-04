use aoc_runner_derive::{aoc, aoc_generator};

struct Assignment {
    start_section_id: u32,
    end_section_id: u32,
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<(Assignment, Assignment)> {
    input
        .lines()
        .map(|assignment_pair| {
            let assignment_pair = assignment_pair.split_once(',').unwrap();
            let first_assignment = assignment_pair.0.split_once('-').unwrap();
            let second_assignment = assignment_pair.1.split_once('-').unwrap();

            (
                Assignment {
                    start_section_id: first_assignment.0.parse().unwrap(),
                    end_section_id: first_assignment.1.parse().unwrap(),
                },
                Assignment {
                    start_section_id: second_assignment.0.parse().unwrap(),
                    end_section_id: second_assignment.1.parse().unwrap(),
                },
            )
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(assignment_pairs: &[(Assignment, Assignment)]) -> usize {
    assignment_pairs
        .iter()
        .filter(|&assignment_pair| {
            assignment_pair.0.start_section_id <= assignment_pair.1.start_section_id
                && assignment_pair.0.end_section_id >= assignment_pair.1.end_section_id
                || assignment_pair.1.start_section_id <= assignment_pair.0.start_section_id
                    && assignment_pair.1.end_section_id >= assignment_pair.0.end_section_id
        })
        .count()
}

#[aoc(day4, part2)]
fn part2(assignment_pairs: &[(Assignment, Assignment)]) -> usize {
    assignment_pairs
        .iter()
        .filter(|&assignment_pair| {
            assignment_pair.0.start_section_id <= assignment_pair.1.end_section_id
                && assignment_pair.0.end_section_id >= assignment_pair.1.start_section_id
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 4);
    }
}
