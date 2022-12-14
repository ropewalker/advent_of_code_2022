use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

type Coordinates = (i32, i32);
type Path = Vec<Coordinates>;
const SAND_POURING_POINT: Coordinates = (500, 0);

struct Cave {
    rock_coordinates: HashSet<Coordinates>,
    depth: i32,
}

impl From<&[Path]> for Cave {
    fn from(paths: &[Path]) -> Self {
        let mut rock_coordinates = HashSet::new();
        let mut depth = SAND_POURING_POINT.1;

        for path in paths {
            for line in path.windows(2) {
                if line[0].0 == line[1].0 {
                    let x = line[0].0;

                    let y_start = i32::min(line[0].1, line[1].1);
                    let y_end = i32::max(line[0].1, line[1].1);

                    if y_end > depth {
                        depth = y_end;
                    }

                    for y in y_start..=y_end {
                        rock_coordinates.insert((x, y));
                    }
                } else {
                    let y = line[0].1;

                    if y > depth {
                        depth = y;
                    }

                    let x_start = i32::min(line[0].0, line[1].0);
                    let x_end = i32::max(line[0].0, line[1].0);

                    for x in x_start..=x_end {
                        rock_coordinates.insert((x, y));
                    }
                }
            }
        }

        Self {
            rock_coordinates,
            depth,
        }
    }
}

#[aoc_generator(day14)]
fn parse_input(input: &str) -> Vec<Path> {
    input
        .lines()
        .map(|path| {
            path.split(" -> ")
                .map(|coordinates| {
                    let mut coordinates = coordinates.split(',');
                    (
                        coordinates.next().unwrap().parse().unwrap(),
                        coordinates.next().unwrap().parse().unwrap(),
                    )
                })
                .collect()
        })
        .collect()
}

#[aoc(day14, part1)]
fn part1(paths: &[Path]) -> usize {
    let mut cave: Cave = paths.into();
    let mut sand_units_count = 0;
    let mut visited = vec![(SAND_POURING_POINT, false, false, false)];

    while !visited.is_empty() {
        let (sand_unit_coordinates, been_down, been_down_left, been_down_right) =
            visited.pop().unwrap();

        if sand_unit_coordinates.1 > cave.depth {
            return sand_units_count;
        }

        let down = (sand_unit_coordinates.0, sand_unit_coordinates.1 + 1);

        if !been_down && !cave.rock_coordinates.contains(&down) {
            visited.push((sand_unit_coordinates, true, been_down_left, been_down_right));
            visited.push((down, false, false, false));
            continue;
        }

        let down_left = (sand_unit_coordinates.0 - 1, sand_unit_coordinates.1 + 1);

        if !been_down_left && !cave.rock_coordinates.contains(&down_left) {
            visited.push((sand_unit_coordinates, been_down, true, been_down_right));
            visited.push((down_left, false, false, false));
            continue;
        }

        let down_right = (sand_unit_coordinates.0 + 1, sand_unit_coordinates.1 + 1);

        if !been_down_right && !cave.rock_coordinates.contains(&down_right) {
            visited.push((
                sand_unit_coordinates,
                been_down,
                been_down_left,
                been_down_right,
            ));
            visited.push((down_right, false, false, false));
            continue;
        }

        cave.rock_coordinates.insert(sand_unit_coordinates);
        sand_units_count += 1;
    }

    sand_units_count
}

#[aoc(day14, part2)]
fn part2(paths: &[Path]) -> usize {
    let mut cave: Cave = paths.into();
    let mut sand_units_count = 0;
    let floor_depth = cave.depth + 2;

    let mut x_min = SAND_POURING_POINT.0;
    let mut x_max = SAND_POURING_POINT.0;

    for y in SAND_POURING_POINT.1..floor_depth {
        let mut rock_count = 2;

        for x in x_min..=x_max {
            if cave.rock_coordinates.contains(&(x, y)) {
                rock_count += 1;

                if rock_count >= 3 {
                    cave.rock_coordinates.insert((x - 1, y + 1));
                }

                if x == x_max {
                    cave.rock_coordinates.insert((x, y + 1));
                    cave.rock_coordinates.insert((x + 1, y + 1));
                }
            } else {
                sand_units_count += 1;
                rock_count = 0;
            }
        }

        x_min -= 1;
        x_max += 1;
    }

    sand_units_count
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 24);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 93);
    }
}
