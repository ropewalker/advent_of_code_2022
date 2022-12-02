use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Copy, Clone)]
enum FirstColumnRecord {
    A,
    B,
    C,
}

impl From<char> for FirstColumnRecord {
    fn from(first_column_record: char) -> Self {
        use FirstColumnRecord::*;

        match first_column_record {
            'A' => A,
            'B' => B,
            'C' => C,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone)]
enum SecondColumnRecord {
    X,
    Y,
    Z,
}

impl From<char> for SecondColumnRecord {
    fn from(second_column_record: char) -> Self {
        use SecondColumnRecord::*;

        match second_column_record {
            'X' => X,
            'Y' => Y,
            'Z' => Z,
            _ => unreachable!(),
        }
    }
}

struct RoundRecord(FirstColumnRecord, SecondColumnRecord);

enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl From<FirstColumnRecord> for Shape {
    fn from(first_column_record: FirstColumnRecord) -> Self {
        use FirstColumnRecord::*;
        use Shape::*;

        match first_column_record {
            A => Rock,
            B => Paper,
            C => Scissors,
        }
    }
}

impl From<SecondColumnRecord> for Shape {
    fn from(second_column_record: SecondColumnRecord) -> Self {
        use SecondColumnRecord::*;
        use Shape::*;

        match second_column_record {
            X => Rock,
            Y => Paper,
            Z => Scissors,
        }
    }
}

impl Shape {
    fn score(&self) -> u32 {
        use Shape::*;

        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

enum RoundOutcome {
    Loss,
    Draw,
    Win,
}

impl From<SecondColumnRecord> for RoundOutcome {
    fn from(second_column_record: SecondColumnRecord) -> Self {
        use RoundOutcome::*;
        use SecondColumnRecord::*;

        match second_column_record {
            X => Loss,
            Y => Draw,
            Z => Win,
        }
    }
}

impl RoundOutcome {
    fn score(&self) -> u32 {
        use RoundOutcome::*;

        match self {
            Loss => 0,
            Draw => 3,
            Win => 6,
        }
    }
}

fn round_outcome(opponent_shape: &Shape, your_shape: &Shape) -> RoundOutcome {
    use RoundOutcome::*;
    use Shape::*;

    match your_shape {
        Rock => match opponent_shape {
            Rock => Draw,
            Paper => Loss,
            Scissors => Win,
        },
        Paper => match opponent_shape {
            Rock => Win,
            Paper => Draw,
            Scissors => Loss,
        },
        Scissors => match opponent_shape {
            Rock => Loss,
            Paper => Win,
            Scissors => Draw,
        },
    }
}

fn your_shape(opponent_shape: &Shape, round_outcome: &RoundOutcome) -> Shape {
    use RoundOutcome::*;
    use Shape::*;

    match opponent_shape {
        Rock => match round_outcome {
            Loss => Scissors,
            Draw => Rock,
            Win => Paper,
        },
        Paper => match round_outcome {
            Loss => Rock,
            Draw => Paper,
            Win => Scissors,
        },
        Scissors => match round_outcome {
            Loss => Paper,
            Draw => Scissors,
            Win => Rock,
        },
    }
}

fn total_score(your_shape: &Shape, round_outcome: &RoundOutcome) -> u32 {
    your_shape.score() + round_outcome.score()
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<RoundRecord> {
    input
        .lines()
        .map(|round_record| {
            RoundRecord(
                round_record.chars().next().unwrap().into(),
                round_record.chars().nth(2).unwrap().into(),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(strategy_guide: &[RoundRecord]) -> u32 {
    strategy_guide
        .iter()
        .map(|round_record| {
            let opponent_shape: Shape = round_record.0.into();
            let your_shape: Shape = round_record.1.into();

            total_score(&your_shape, &round_outcome(&opponent_shape, &your_shape))
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(strategy_guide: &[RoundRecord]) -> u32 {
    strategy_guide
        .iter()
        .map(|round_record| {
            let opponent_shape: Shape = round_record.0.into();
            let round_outcome: RoundOutcome = round_record.1.into();

            total_score(&your_shape(&opponent_shape, &round_outcome), &round_outcome)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"A Y
B X
C Z";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 15);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 12);
    }
}
