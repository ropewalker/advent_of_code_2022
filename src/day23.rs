use crate::day23::CardinalDirection::*;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::hash_map::Entry::Vacant;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Coordinates(i32, i32);

impl Coordinates {
    fn all_adjacent_positions(&self) -> [Coordinates; 8] {
        let mut positions = [Coordinates(0, 0); 8];

        for (index, (x, y)) in [
            (-1, -1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
        ]
        .into_iter()
        .enumerate()
        {
            positions[index] = Coordinates(self.0 + x, self.1 + y);
        }

        positions
    }

    fn adjacent_positions_in_direction(&self, direction: &CardinalDirection) -> [Coordinates; 3] {
        match direction {
            N => [
                Coordinates(self.0 - 1, self.1 - 1),
                Coordinates(self.0, self.1 - 1),
                Coordinates(self.0 + 1, self.1 - 1),
            ],
            S => [
                Coordinates(self.0 - 1, self.1 + 1),
                Coordinates(self.0, self.1 + 1),
                Coordinates(self.0 + 1, self.1 + 1),
            ],
            W => [
                Coordinates(self.0 - 1, self.1 - 1),
                Coordinates(self.0 - 1, self.1),
                Coordinates(self.0 - 1, self.1 + 1),
            ],
            E => [
                Coordinates(self.0 + 1, self.1 - 1),
                Coordinates(self.0 + 1, self.1),
                Coordinates(self.0 + 1, self.1 + 1),
            ],
        }
    }

    fn move_one_step(&self, direction: &CardinalDirection) -> Coordinates {
        match direction {
            N => Coordinates(self.0, self.1 - 1),
            S => Coordinates(self.0, self.1 + 1),
            W => Coordinates(self.0 - 1, self.1),
            E => Coordinates(self.0 + 1, self.1),
        }
    }
}

type Map = HashSet<Coordinates>;

#[aoc_generator(day23)]
fn parse_input(input: &str) -> Map {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, tile)| *tile == '#')
                .map(move |(x, _)| Coordinates(x as i32, y as i32))
        })
        .collect()
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum CardinalDirection {
    N,
    S,
    W,
    E,
}

fn simulate_one_round(map: &mut Map, directions: &mut VecDeque<CardinalDirection>) -> usize {
    let mut proposed_moves: HashMap<Coordinates, Coordinates> = HashMap::new();

    'next_elf: for elf_position in map.iter() {
        if !elf_position
            .all_adjacent_positions()
            .iter()
            .any(|adjacent_position| map.contains(adjacent_position))
        {
            continue 'next_elf;
        }

        'next_direction: for direction in directions.iter() {
            for adjacent_position in elf_position.adjacent_positions_in_direction(direction) {
                if map.contains(&adjacent_position) {
                    continue 'next_direction;
                }
            }

            let new_position = elf_position.move_one_step(direction);

            if let Vacant(entry) = proposed_moves.entry(new_position) {
                entry.insert(*elf_position);
            } else {
                proposed_moves.remove(&new_position);
            }

            continue 'next_elf;
        }
    }

    for (new_position, old_position) in proposed_moves.iter() {
        map.remove(old_position);
        map.insert(*new_position);
    }

    let first_direction = directions.pop_front().unwrap();
    directions.push_back(first_direction);

    proposed_moves.len()
}

fn simulate_rounds(map: &mut Map, rounds: usize) {
    let mut directions: VecDeque<CardinalDirection> = VecDeque::from([N, S, W, E]);

    for _round in 1..=rounds {
        if simulate_one_round(map, &mut directions) == 0 {
            break;
        }
    }
}

fn simulate(map: &mut Map) -> usize {
    let mut directions: VecDeque<CardinalDirection> = VecDeque::from([N, S, W, E]);

    let mut rounds = 1;

    loop {
        if simulate_one_round(map, &mut directions) == 0 {
            return rounds;
        }

        rounds += 1;
    }
}

#[aoc(day23, part1)]
fn part1(map: &Map) -> usize {
    let mut map = map.clone();
    simulate_rounds(&mut map, 10);

    let min_x = map.iter().map(|coordinates| coordinates.0).min().unwrap();
    let max_x = map.iter().map(|coordinates| coordinates.0).max().unwrap();
    let min_y = map.iter().map(|coordinates| coordinates.1).min().unwrap();
    let max_y = map.iter().map(|coordinates| coordinates.1).max().unwrap();

    ((max_x - min_x + 1) * (max_y - min_y + 1)) as usize - map.len()
}

#[aoc(day23, part2)]
fn part2(map: &Map) -> usize {
    let mut map = map.clone();
    simulate(&mut map)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 110);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 20);
    }
}
