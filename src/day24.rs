use crate::day24::Direction::*;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashSet, VecDeque};

type Coordinates = (i32, i32);

#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Map {
    blizzards: [Vec<Vec<bool>>; 4],
    rows: usize,
    columns: usize,
}

impl Map {
    fn is_blizzard(&self, (x, y): Coordinates, time_passed: usize) -> bool {
        let rows = self.rows as i32;
        let columns = self.columns as i32;
        let minutes_passed = time_passed as i32;

        for direction in 0..4 {
            let (xt, yt) = match direction {
                direction if direction == Up as usize => (x, (y + minutes_passed) % rows),
                direction if direction == Down as usize => {
                    (x, ((y - minutes_passed) % rows + rows) % rows)
                }
                direction if direction == Left as usize => ((x + minutes_passed) % columns, y),
                direction if direction == Right as usize => {
                    (((x - minutes_passed) % columns + columns) % columns, y)
                }
                _ => unreachable!(),
            };

            if self.blizzards[direction][yt as usize][xt as usize] {
                return true;
            }
        }

        false
    }
}

#[aoc_generator(day24)]
fn parse_input(input: &str) -> Map {
    let rows = input.lines().count() - 2;
    let columns = input.lines().next().unwrap().len() - 2;

    let mut blizzards: [Vec<Vec<bool>>; 4] = Default::default();

    input.lines().skip(1).take(rows).for_each(|tiles| {
        let mut row_blizzards: [Vec<bool>; 4] = Default::default();

        tiles.chars().skip(1).take(columns).for_each(|tile| {
            let blizzard = match tile {
                '.' => None,
                '^' => Some(Up),
                'v' => Some(Down),
                '<' => Some(Left),
                '>' => Some(Right),
                _ => unreachable!(),
            };

            for (index, direction_blizzards) in row_blizzards.iter_mut().enumerate() {
                match blizzard {
                    Some(direction) if direction as usize == index => {
                        direction_blizzards.push(true)
                    }
                    _ => direction_blizzards.push(false),
                };
            }
        });

        for index in 0..4 {
            blizzards[index].push(row_blizzards[index].to_owned());
        }
    });

    Map {
        blizzards,
        rows,
        columns,
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct State {
    current_position: Coordinates,
    iteration_x: usize,
    iteration_y: usize,
}

#[derive(Clone, Eq, PartialEq)]
struct Node {
    current_position: Coordinates,
    minutes_passed: usize,
}

fn time_to_reach_goal(map: &Map, forgot_snacks: bool) -> Option<usize> {
    let map = map.to_owned();
    let starting_position = (0, -1);
    let final_position = (map.columns as i32 - 1, map.rows as i32);

    let mut goals = if forgot_snacks {
        vec![final_position, starting_position, final_position]
    } else {
        vec![final_position]
    };

    let initial_state = State {
        current_position: (0, 0),
        iteration_x: 0,
        iteration_y: 0,
    };

    let mut queue = VecDeque::from([Node {
        current_position: (0, 0),
        minutes_passed: 0,
    }]);

    let mut visited: HashSet<State> = HashSet::from([initial_state]);

    'next_goal: while !goals.is_empty() {
        let goal = goals.pop().unwrap();

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            let minutes_passed = node.minutes_passed + 1;

            for step in [(0, 0), (0, -1), (0, 1), (-1, 0), (1, 0)].into_iter() {
                let new_position = (
                    node.current_position.0 + step.0,
                    node.current_position.1 + step.1,
                );

                if new_position == goal {
                    if goals.is_empty() {
                        return Some(minutes_passed);
                    }

                    let initial_state = State {
                        current_position: new_position,
                        iteration_x: minutes_passed % map.columns,
                        iteration_y: minutes_passed % map.rows,
                    };

                    queue = VecDeque::from([Node {
                        current_position: new_position,
                        minutes_passed,
                    }]);

                    visited = HashSet::from([initial_state]);

                    continue 'next_goal;
                }

                if new_position == starting_position || new_position == final_position {
                    let state = State {
                        current_position: new_position,
                        iteration_x: minutes_passed % map.columns,
                        iteration_y: minutes_passed % map.rows,
                    };

                    if !visited.contains(&state) {
                        queue.push_back(Node {
                            current_position: new_position,
                            minutes_passed,
                        });
                    }
                }

                if new_position.0 >= 0
                    && new_position.0 < map.columns as i32
                    && new_position.1 >= 0
                    && new_position.1 < map.rows as i32
                    && !map.is_blizzard(new_position, minutes_passed)
                {
                    let state = State {
                        current_position: new_position,
                        iteration_x: minutes_passed % map.columns,
                        iteration_y: minutes_passed % map.rows,
                    };

                    if !visited.contains(&state) {
                        visited.insert(state);

                        queue.push_back(Node {
                            current_position: new_position,
                            minutes_passed,
                        });
                    }
                }
            }
        }
    }

    None
}

#[aoc(day24, part1)]
fn part1(map: &Map) -> Option<usize> {
    time_to_reach_goal(map, false)
}

#[aoc(day24, part2)]
fn part2(map: &Map) -> Option<usize> {
    time_to_reach_goal(map, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), Some(18));
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), Some(54));
    }
}
