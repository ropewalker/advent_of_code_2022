use crate::day17::Direction::*;
use crate::day17::TileType::*;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

const SMALLER_NUMBER_OF_ROCKS: usize = 2_022;
const LARGER_NUMBER_OF_ROCKS: usize = 1_000_000_000_000;

const CHAMBER_WIDTH: usize = 7;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Shape {
    tiles: Vec<Vec<TileType>>,
    width: usize,
    height: usize,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
enum TileType {
    Rock,
    Empty,
}

struct Chamber {
    tiles: Vec<[TileType; CHAMBER_WIDTH]>,
    height: usize,
}

impl Debug for Chamber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.tiles.iter().rev().for_each(|row| {
            writeln!(
                f,
                "{}",
                row.iter().fold(String::new(), |mut acc, tile| {
                    acc.push(match tile {
                        Rock => '#',
                        Empty => '.',
                    });
                    acc
                })
            )
            .unwrap();
        });

        writeln!(f)?;

        Ok(())
    }
}

fn get_shapes() -> Vec<Shape> {
    const SHAPES: &str = r"####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##";

    SHAPES
        .split("\n\n")
        .map(|shape| {
            let rocks: Vec<Vec<TileType>> = shape
                .lines()
                .rev()
                .map(|row| {
                    row.chars()
                        .map(|char| match char {
                            '.' => Empty,
                            '#' => Rock,
                            _ => unreachable!(),
                        })
                        .collect()
                })
                .collect();

            let height = rocks.len();
            let width = rocks.iter().map(|row| row.len()).max().unwrap();

            Shape {
                tiles: rocks,
                width,
                height,
            }
        })
        .collect()
}

#[aoc_generator(day17)]
fn parse_input(input: &str) -> Vec<Direction> {
    input
        .chars()
        .map(|direction| match direction {
            '<' => Left,
            '>' => Right,
            _ => unreachable!(),
        })
        .collect()
}

fn starting_position(chamber: &Chamber) -> (usize, usize) {
    (2, chamber.tiles.len() + 3)
}

fn overlap(shape: &Shape, position: &(usize, usize), chamber: &Chamber) -> bool {
    for y in position.1..position.1 + shape.tiles.len() {
        if let Some(row) = chamber.tiles.get(y) {
            for (x, tile) in row.iter().enumerate().skip(position.0).take(shape.width) {
                if *tile == Rock && shape.tiles[y - position.1][x - position.0] == Rock {
                    return true;
                }
            }
        }
    }

    false
}

fn push_rock(
    shape: &Shape,
    position: &(usize, usize),
    direction: &Direction,
    chamber: &Chamber,
) -> Result<(usize, usize), ()> {
    let new_position = match direction {
        Left => (usize::max(position.0, 1) - 1, position.1),
        Right => (
            usize::min(position.0 + shape.width - 1, CHAMBER_WIDTH - 2) + 2 - shape.width,
            position.1,
        ),
    };

    if overlap(shape, &new_position, chamber) {
        return Err(());
    }

    Ok(new_position)
}

fn fall_down(
    shape: &Shape,
    position: &(usize, usize),
    chamber: &Chamber,
) -> Result<(usize, usize), ()> {
    if position.1 == 0 {
        return Err(());
    }

    let new_position = (position.0, position.1 - 1);

    if overlap(shape, &new_position, chamber) {
        return Err(());
    }

    Ok(new_position)
}

fn update_chamber(shape: &Shape, position: &(usize, usize), chamber: &mut Chamber) {
    for y in position.1..position.1 + shape.height {
        while chamber.tiles.len() < y + 1 {
            chamber.tiles.push([Empty; CHAMBER_WIDTH]);
            chamber.height += 1;
        }

        for (x, tile) in chamber.tiles[y]
            .iter_mut()
            .enumerate()
            .skip(position.0)
            .take(shape.width)
        {
            if shape.tiles[y - position.1][x - position.0] == Rock {
                *tile = Rock
            }
        }
    }
}

fn truncate_chamber(chamber: &mut Chamber) {
    for y in (0..chamber.tiles.len()).rev() {
        let mut accessible_tiles = [Rock; CHAMBER_WIDTH];

        if y == chamber.tiles.len() - 1 {
            accessible_tiles = chamber.tiles[y];
        } else {
            let previous_row = chamber.tiles[y + 1];
            let current_row = chamber.tiles[y];

            for x in 0..CHAMBER_WIDTH {
                if previous_row[x] == Empty && current_row[x] == Empty {
                    accessible_tiles[x] = Empty;

                    for neighbour_x in (0..x).rev() {
                        if accessible_tiles[neighbour_x] == Empty {
                            break;
                        }

                        if current_row[neighbour_x] == Empty {
                            accessible_tiles[neighbour_x] = Empty;
                        } else {
                            break;
                        }
                    }

                    for neighbour_x in x + 1..CHAMBER_WIDTH {
                        if accessible_tiles[neighbour_x] == Empty {
                            break;
                        }

                        if current_row[neighbour_x] == Empty {
                            accessible_tiles[neighbour_x] = Empty;
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        chamber.tiles[y] = accessible_tiles;

        if !accessible_tiles.contains(&Empty) {
            chamber.tiles.drain(0..=y);
            break;
        }
    }
}

#[aoc(day17, part1)]
fn part1(directions: &[Direction]) -> usize {
    let mut chamber = Chamber {
        tiles: Vec::new(),
        height: 0,
    };

    let shapes = get_shapes();
    let mut direction_index = 0;

    for rock_number in 0..SMALLER_NUMBER_OF_ROCKS {
        let shape_index = rock_number % shapes.len();
        let shape = shapes.get(shape_index).unwrap();
        let mut position = starting_position(&chamber);

        loop {
            let direction = directions.get(direction_index).unwrap();
            direction_index = (direction_index + 1) % directions.len();

            if let Ok(new_position) = push_rock(shape, &position, direction, &chamber) {
                position = new_position;
            }

            if let Ok(new_position) = fall_down(shape, &position, &chamber) {
                position = new_position;
            } else {
                update_chamber(shape, &position, &mut chamber);

                break;
            }
        }
    }

    chamber.height
}

#[derive(Eq, PartialEq, Hash)]
struct State {
    tiles: Vec<[TileType; CHAMBER_WIDTH]>,
    direction_index: usize,
    shape_index: usize,
}

#[aoc(day17, part2)]
fn part2(directions: &[Direction]) -> usize {
    let mut chamber = Chamber {
        tiles: Vec::new(),
        height: 0,
    };

    let shapes = get_shapes();
    let mut direction_index = 0;

    let mut states: HashMap<State, (usize, usize)> = HashMap::new();
    let mut rock_number = 0;

    let mut cycle_found = false;

    while rock_number < LARGER_NUMBER_OF_ROCKS {
        let shape_index = rock_number % shapes.len();
        let shape = shapes.get(shape_index).unwrap();
        let mut position = starting_position(&chamber);

        loop {
            let direction = directions.get(direction_index).unwrap();
            direction_index = (direction_index + 1) % directions.len();

            if let Ok(new_position) = push_rock(shape, &position, direction, &chamber) {
                position = new_position;
            }

            if let Ok(new_position) = fall_down(shape, &position, &chamber) {
                position = new_position;
            } else {
                update_chamber(shape, &position, &mut chamber);

                if !cycle_found {
                    truncate_chamber(&mut chamber);

                    let state = State {
                        tiles: chamber.tiles.clone(),
                        direction_index,
                        shape_index,
                    };

                    if let Some((prev_rock_number, prev_height)) = states.get(&state) {
                        let cycle_len = rock_number - prev_rock_number;
                        let height_delta = chamber.height - prev_height;

                        let remaining_rocks = LARGER_NUMBER_OF_ROCKS - 1 - rock_number;
                        let remaining_cycles = remaining_rocks / cycle_len;

                        chamber.height += remaining_cycles * height_delta;
                        rock_number += remaining_cycles * cycle_len;

                        cycle_found = true;
                    } else {
                        states.insert(state, (rock_number, chamber.height));
                    }
                }

                break;
            }
        }

        rock_number += 1;
    }

    chamber.height
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 3_068);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 1_514_285_714_288);
    }
}
