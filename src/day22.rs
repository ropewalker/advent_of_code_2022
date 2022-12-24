use crate::day22::Direction::*;
use crate::day22::PathSegment::{MoveSteps, Turn};
use crate::day22::Tile::*;
use crate::day22::TurnDirection::*;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

type Coordinates = (i32, i32);
type Path = Vec<PathSegment>;

const SIDE_SIZE: i32 = 50;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl From<Direction> for Coordinates {
    fn from(direction: Direction) -> Self {
        match direction {
            Right => (1, 0),
            Down => (0, 1),
            Left => (-1, 0),
            Up => (0, -1),
        }
    }
}

impl Direction {
    fn turn(self, turn_direction: &TurnDirection) -> Self {
        match (self, turn_direction) {
            (Right, Clockwise) => Down,
            (Down, Clockwise) => Left,
            (Left, Clockwise) => Up,
            (Up, Clockwise) => Right,
            (Right, Counterclockwise) => Up,
            (Down, Counterclockwise) => Right,
            (Left, Counterclockwise) => Down,
            (Up, Counterclockwise) => Left,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum TurnDirection {
    Clockwise,
    Counterclockwise,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum PathSegment {
    MoveSteps(i32),
    Turn(TurnDirection),
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    OpenTile,
    SolidWall,
}

#[aoc_generator(day22)]
fn parse_input(input: &str) -> (HashMap<Coordinates, Tile>, Path) {
    use aoc_parse::{parser, prelude::*};

    let (map, path) = parser!(
        map:section(lines(string(tile:any_char+)))
        path:section(line(path_segment:{
            move_steps:i32 => MoveSteps(move_steps),
            turn_direction:{
                "R" => Clockwise,
                "L" => Counterclockwise
            } => Turn(turn_direction)
        }+)
    ))
    .parse(input)
    .unwrap();

    let map: HashMap<Coordinates, Tile> = map
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, tile)| *tile != ' ')
                .map(move |(column, tile)| match tile {
                    '.' => ((column as i32 + 1, row as i32 + 1), OpenTile),
                    '#' => ((column as i32 + 1, row as i32 + 1), SolidWall),
                    _ => unreachable!(),
                })
        })
        .collect();

    (map, path)
}

fn password(position: &Coordinates, direction: &Direction) -> usize {
    1_000 * position.1 as usize + 4 * position.0 as usize + *direction as usize
}

#[aoc(day22, part1)]
fn part1((map, path): &(HashMap<Coordinates, Tile>, Path)) -> usize {
    let mut current_position = map
        .keys()
        .filter(|(_, row)| *row == 1)
        .min_by(|coordinates1, coordinates2| coordinates1.0.cmp(&coordinates2.0))
        .unwrap()
        .to_owned();
    let mut direction = Right;

    for path_segment in path.iter() {
        match path_segment {
            MoveSteps(steps) => {
                let (x_step, y_step): Coordinates = direction.into();

                for _ in 0..*steps {
                    let mut new_position =
                        (current_position.0 + x_step, current_position.1 + y_step);

                    if map.get(&new_position).is_none() {
                        match direction {
                            Right => {
                                new_position = map
                                    .keys()
                                    .filter(|(_, row)| *row == new_position.1)
                                    .min_by(|coordinates1, coordinates2| {
                                        coordinates1.0.cmp(&coordinates2.0)
                                    })
                                    .unwrap()
                                    .to_owned()
                            }
                            Down => {
                                new_position = map
                                    .keys()
                                    .filter(|(column, _)| *column == new_position.0)
                                    .min_by(|coordinates1, coordinates2| {
                                        coordinates1.1.cmp(&coordinates2.1)
                                    })
                                    .unwrap()
                                    .to_owned()
                            }
                            Left => {
                                new_position = map
                                    .keys()
                                    .filter(|(_, row)| *row == new_position.1)
                                    .max_by(|coordinates1, coordinates2| {
                                        coordinates1.0.cmp(&coordinates2.0)
                                    })
                                    .unwrap()
                                    .to_owned()
                            }
                            Up => {
                                new_position = map
                                    .keys()
                                    .filter(|(column, _)| *column == new_position.0)
                                    .max_by(|coordinates1, coordinates2| {
                                        coordinates1.1.cmp(&coordinates2.1)
                                    })
                                    .unwrap()
                                    .to_owned()
                            }
                        };
                    }

                    if let Some(tile) = map.get(&new_position) {
                        match tile {
                            OpenTile => {
                                current_position = new_position;
                                continue;
                            }
                            SolidWall => {
                                break;
                            }
                        }
                    }
                }
            }
            Turn(turn_direction) => {
                direction = direction.turn(turn_direction);
            }
        }
    }

    password(&current_position, &direction)
}

#[aoc(day22, part2)]
fn part2_special_case((map, path): &(HashMap<Coordinates, Tile>, Path)) -> usize {
    let mut connections: HashMap<(Coordinates, Direction), (Coordinates, Direction)> =
        HashMap::with_capacity(14 * SIDE_SIZE as usize);

    for (tile1, tile2) in (1..=SIDE_SIZE)
        .map(|y| (SIDE_SIZE + 1, y))
        .zip((2 * SIDE_SIZE + 1..=3 * SIDE_SIZE).rev().map(|y| (1, y)))
    {
        connections.insert((tile1, Left), (tile2, Right));
        connections.insert((tile2, Left), (tile1, Right));
    }

    for (tile1, tile2) in (SIDE_SIZE + 1..=2 * SIDE_SIZE)
        .map(|x| (x, 1))
        .zip((3 * SIDE_SIZE + 1..=4 * SIDE_SIZE).map(|y| (1, y)))
    {
        connections.insert((tile1, Up), (tile2, Right));
        connections.insert((tile2, Left), (tile1, Down));
    }

    for (tile1, tile2) in (2 * SIDE_SIZE + 1..=3 * SIDE_SIZE)
        .map(|x| (x, 1))
        .zip((1..=SIDE_SIZE).map(|x| (x, 4 * SIDE_SIZE)))
    {
        connections.insert((tile1, Up), (tile2, Up));
        connections.insert((tile2, Down), (tile1, Down));
    }

    for (tile1, tile2) in (1..=SIDE_SIZE).map(|y| (3 * SIDE_SIZE, y)).zip(
        (2 * SIDE_SIZE + 1..=3 * SIDE_SIZE)
            .rev()
            .map(|y| (2 * SIDE_SIZE, y)),
    ) {
        connections.insert((tile1, Right), (tile2, Left));
        connections.insert((tile2, Right), (tile1, Left));
    }

    for (tile1, tile2) in (2 * SIDE_SIZE + 1..=3 * SIDE_SIZE)
        .map(|x| (x, SIDE_SIZE))
        .zip((SIDE_SIZE + 1..2 * SIDE_SIZE).map(|y| (2 * SIDE_SIZE, y)))
    {
        connections.insert((tile1, Down), (tile2, Left));
        connections.insert((tile2, Right), (tile1, Up));
    }

    for (tile1, tile2) in (SIDE_SIZE + 1..=2 * SIDE_SIZE)
        .map(|x| (x, 3 * SIDE_SIZE))
        .zip((3 * SIDE_SIZE + 1..=4 * SIDE_SIZE).map(|y| (SIDE_SIZE, y)))
    {
        connections.insert((tile1, Down), (tile2, Left));
        connections.insert((tile2, Right), (tile1, Up));
    }

    for (tile1, tile2) in (1..=SIDE_SIZE)
        .map(|x| (x, 2 * SIDE_SIZE + 1))
        .zip((SIDE_SIZE + 1..=2 * SIDE_SIZE).map(|y| (SIDE_SIZE + 1, y)))
    {
        connections.insert((tile1, Up), (tile2, Right));
        connections.insert((tile2, Left), (tile1, Down));
    }

    let mut current_position = map
        .keys()
        .filter(|(_, row)| *row == 1)
        .min_by(|coordinates1, coordinates2| coordinates1.0.cmp(&coordinates2.0))
        .unwrap()
        .to_owned();
    let mut direction = Right;

    for path_segment in path.iter() {
        match path_segment {
            MoveSteps(steps) => {
                for _ in 0..*steps {
                    let (x_step, y_step): Coordinates = direction.into();

                    let mut new_position =
                        (current_position.0 + x_step, current_position.1 + y_step);
                    let mut new_direction = direction;

                    if map.get(&new_position).is_none() {
                        let (connected_position, connected_direction) =
                            connections.get(&(current_position, direction)).unwrap();

                        new_position = *connected_position;
                        new_direction = *connected_direction;
                    }

                    if let Some(tile) = map.get(&new_position) {
                        match tile {
                            OpenTile => {
                                current_position = new_position;
                                direction = new_direction;
                                continue;
                            }
                            SolidWall => {
                                break;
                            }
                        }
                    }
                }
            }
            Turn(turn_direction) => {
                direction = direction.turn(turn_direction);
            }
        }
    }

    password(&current_position, &direction)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 6_032);
    }
}
