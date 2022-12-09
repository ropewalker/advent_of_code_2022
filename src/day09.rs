use crate::day09::Direction::*;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Motion {
    direction: Direction,
    steps: usize,
}

type Coordinates = (i32, i32);

struct Rope {
    knots: Vec<Coordinates>,
}

impl Rope {
    fn tail(&self) -> Coordinates {
        *self.knots.last().unwrap()
    }

    fn pull_tail(&mut self) {
        let mut previous_knot = *self.knots.first().unwrap();

        for knot in self.knots.iter_mut().skip(1) {
            let (distance_x, distance_y) = (previous_knot.0 - knot.0, previous_knot.1 - knot.1);

            if distance_x.abs() > 1 || distance_y.abs() > 1 {
                knot.0 += distance_x.signum();
                knot.1 += distance_y.signum();
            }

            previous_knot = *knot;
        }
    }

    fn make_step(&mut self, direction: &Direction) {
        let mut head = self.knots.first_mut().unwrap();

        match direction {
            Left => head.0 -= 1,
            Right => head.0 += 1,
            Up => head.1 += 1,
            Down => head.1 -= 1,
        }

        self.pull_tail();
    }
}

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<Motion> {
    input
        .lines()
        .map(|motion| {
            let mut tokens = motion.split_ascii_whitespace();

            Motion {
                direction: match tokens.next().unwrap() {
                    "L" => Left,
                    "R" => Right,
                    "U" => Up,
                    "D" => Down,
                    _ => unreachable!(),
                },
                steps: tokens.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(series_of_motions: &[Motion]) -> usize {
    let mut rope = Rope {
        knots: vec![(0, 0); 2],
    };

    let mut tail_visited = HashSet::with_capacity(series_of_motions.len());
    tail_visited.insert((0, 0));

    for motion in series_of_motions {
        for _ in 0..motion.steps {
            rope.make_step(&motion.direction);
            tail_visited.insert(rope.tail());
        }
    }

    tail_visited.len()
}

#[aoc(day9, part2)]
fn part2(series_of_motions: &[Motion]) -> usize {
    let mut rope = Rope {
        knots: vec![(0, 0); 10],
    };

    let mut tail_visited = HashSet::with_capacity(series_of_motions.len());
    tail_visited.insert((0, 0));

    for motion in series_of_motions {
        for _ in 0..motion.steps {
            rope.make_step(&motion.direction);
            tail_visited.insert(rope.tail());
        }
    }

    tail_visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    static TEST_INPUT_2: &str = r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT_1)), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT_1)), 1);
        assert_eq!(part2(&parse_input(TEST_INPUT_2)), 36);
    }
}
