use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet, VecDeque};

type Coordinates = (i32, i32, i32);

#[aoc_generator(day18)]
fn parse_input(input: &str) -> Vec<Coordinates> {
    use aoc_parse::{parser, prelude::*};

    parser!(lines((i32 "," i32 "," i32))).parse(input).unwrap()
}

fn manhattan_distance(start: &Coordinates, end: &Coordinates) -> i32 {
    (start.0 - end.0).abs() + (start.1 - end.1).abs() + (start.2 - end.2).abs()
}

#[aoc(day18, part1)]
fn part1(cubes: &[Coordinates]) -> usize {
    let mut covered_sides: HashMap<Coordinates, usize> = HashMap::new();

    for first_cube in cubes {
        for second_cube in cubes {
            if first_cube != second_cube && manhattan_distance(first_cube, second_cube) == 1 {
                covered_sides
                    .entry(*first_cube)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
    }

    cubes.len() * 6 - covered_sides.values().sum::<usize>()
}

#[aoc(day18, part2)]
fn part2(cubes: &[Coordinates]) -> usize {
    let (min_x, max_x, min_y, max_y, min_z, max_z) = cubes.iter().fold(
        (
            cubes[0].0 - 1,
            cubes[0].0 + 1,
            cubes[0].1 - 1,
            cubes[0].1 + 1,
            cubes[0].2 - 1,
            cubes[0].2 + 1,
        ),
        |(min_x, max_x, min_y, max_y, min_z, max_z), cube| {
            (
                i32::min(min_x, cube.0 - 1),
                i32::max(max_x, cube.0 + 1),
                i32::min(min_y, cube.1 - 1),
                i32::max(max_y, cube.1 + 1),
                i32::min(min_z, cube.2 - 1),
                i32::max(max_z, cube.2 + 1),
            )
        },
    );

    let mut queue = VecDeque::from(vec![(min_x, min_y, min_z)]);
    let mut visited: HashSet<Coordinates> = HashSet::from([(min_x, min_y, min_z)]);

    let mut external_sides_count = 0;

    while !queue.is_empty() {
        let current_coordinates = queue.pop_front().unwrap();

        for (x, y, z) in [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ]
        .iter()
        .map(|(x, y, z)| {
            (
                x + current_coordinates.0,
                y + current_coordinates.1,
                z + current_coordinates.2,
            )
        })
        .filter(|(x, y, z)| {
            *x >= min_x && *x <= max_x && *y >= min_y && *y <= max_y && *z >= min_z && *z <= max_z
        }) {
            if cubes.contains(&(x, y, z)) {
                external_sides_count += 1;
            } else if !visited.contains(&(x, y, z)) {
                queue.push_back((x, y, z));
                visited.insert((x, y, z));
            }
        }
    }

    external_sides_count
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 64);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 58);
    }
}
