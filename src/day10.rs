use crate::day10::Instruction::*;
use crate::day10::PixelState::*;
use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt::{Display, Formatter};

const X_STARTING_VALUE: i32 = 1;
const SIGNAL_STRENGTH_MEASURE_CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];
const MAX_INSTRUCTION_DURATION: usize = 2;

const CRT_WIDTH: usize = 40;
const CRT_HEIGHT: usize = 6;

enum Instruction {
    AddX(i32),
    NoOp,
}

impl Instruction {
    fn duration(&self) -> usize {
        match self {
            AddX(_) => 2,
            NoOp => 1,
        }
    }
}

#[derive(Clone, Copy)]
enum PixelState {
    Lit,
    Dark,
}

impl Display for PixelState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Lit => write!(f, "#"),
            Dark => write!(f, "."),
        }
    }
}

struct Crt([PixelState; CRT_WIDTH * CRT_HEIGHT]);

impl Display for Crt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..CRT_HEIGHT {
            for x in 0..CRT_WIDTH {
                write!(f, "{}", self.0[y * CRT_WIDTH + x])?
            }

            if y < CRT_HEIGHT - 1 {
                writeln!(f)?
            }
        }

        Ok(())
    }
}

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|instruction| {
            let mut tokens = instruction.split_ascii_whitespace();

            match tokens.next().unwrap() {
                "addx" => AddX(tokens.next().unwrap().parse().unwrap()),
                "noop" => NoOp,
                _ => unreachable!(),
            }
        })
        .collect()
}

#[aoc(day10, part1)]
fn part1(program: &[Instruction]) -> i32 {
    let mut cycles_passed = 0;
    let mut x_value = X_STARTING_VALUE;
    let mut signal_strengths_sum = 0;
    let mut measurements_done = 0;

    for instruction in program {
        if measurements_done < SIGNAL_STRENGTH_MEASURE_CYCLES.len()
            && cycles_passed
                >= SIGNAL_STRENGTH_MEASURE_CYCLES[measurements_done] - MAX_INSTRUCTION_DURATION
        {
            signal_strengths_sum +=
                x_value * SIGNAL_STRENGTH_MEASURE_CYCLES[measurements_done] as i32;
            measurements_done += 1;
        }

        cycles_passed += instruction.duration();

        if let AddX(value) = instruction {
            x_value += value;
        }
    }

    signal_strengths_sum
}

#[aoc(day10, part2)]
fn part2(program: &[Instruction]) -> String {
    let mut cycles_passed = 0;
    let mut x_value = X_STARTING_VALUE;
    let mut crt = Crt([Dark; CRT_WIDTH * CRT_HEIGHT]);

    for instruction in program {
        let sprite_pixels_horizontal_positions = [x_value - 1, x_value, x_value + 1];
        let mut drawn_pixels = Vec::with_capacity(instruction.duration());

        for cycle in 0..instruction.duration() {
            drawn_pixels.push(cycles_passed + cycle)
        }

        for pixel_position in drawn_pixels {
            if sprite_pixels_horizontal_positions.contains(&((pixel_position % CRT_WIDTH) as i32)) {
                crt.0[pixel_position] = Lit;
            }
        }

        cycles_passed += instruction.duration();

        if let AddX(value) = instruction {
            x_value += value;
        }
    }

    crt.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 13_140);
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse_input(TEST_INPUT)),
            r"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
                .to_string()
        );
    }
}
