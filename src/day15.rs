use aoc_runner_derive::*;
use std::collections::{HashMap, HashSet};

type Coordinates = (i64, i64);

fn manhattan_distance(start: &Coordinates, end: &Coordinates) -> i64 {
    (start.0 - end.0).abs() + (start.1 - end.1).abs()
}

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Vec<(Coordinates, Coordinates)> {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(lines(
        "Sensor at x=" sensor_x:i64
        ", y=" sensor_y:i64
        ": closest beacon is at x=" beacon_x:i64
        ", y=" beacon_y:i64 =>
            ((sensor_x, sensor_y), (beacon_x, beacon_y))
    ));
    parser.parse(input).unwrap()
}

fn non_beacon_positions(coordinates: &[(Coordinates, Coordinates)], row: i64) -> usize {
    let mut segments: Vec<_> = coordinates
        .iter()
        .map(|(sensor_coordinates, beacon_coordinates)| {
            (
                *sensor_coordinates,
                manhattan_distance(sensor_coordinates, beacon_coordinates),
            )
        })
        .filter(|(sensor_coordinates, radius)| *radius >= (sensor_coordinates.1 - row).abs())
        .map(|(sensor_coordinates, radius)| {
            let distance = radius - (sensor_coordinates.1 - row).abs();

            (
                sensor_coordinates.0 - distance,
                sensor_coordinates.0 + distance,
            )
        })
        .collect();

    let mut non_overlapping_segments = Vec::with_capacity(segments.len());

    while !segments.is_empty() {
        let mut segment = segments.pop().unwrap();
        let mut still_non_overlapping_segments =
            Vec::with_capacity(non_overlapping_segments.len() + 1);

        while !non_overlapping_segments.is_empty() {
            let non_overlapping_segment: (i64, i64) = non_overlapping_segments.pop().unwrap();

            if non_overlapping_segment.1 < segment.0 || non_overlapping_segment.0 > segment.1 {
                still_non_overlapping_segments.push(non_overlapping_segment);
            } else {
                segment = (
                    i64::min(non_overlapping_segment.0, segment.0),
                    i64::max(non_overlapping_segment.1, segment.1),
                );
            }
        }

        still_non_overlapping_segments.push(segment);
        non_overlapping_segments = still_non_overlapping_segments;
    }

    let scanned_positions_count: usize = non_overlapping_segments
        .iter()
        .map(|(x_min, x_max)| (x_max - x_min + 1) as usize)
        .sum();

    let beacons_count = coordinates
        .iter()
        .filter(|(_, beacon_coordinates)| beacon_coordinates.1 == row)
        .map(|(_, beacon_coordinates)| beacon_coordinates)
        .collect::<HashSet<_>>()
        .len();

    scanned_positions_count - beacons_count
}

#[aoc(day15, part1)]
fn part1(coordinates: &[(Coordinates, Coordinates)]) -> usize {
    non_beacon_positions(coordinates, 2_000_000)
}

fn tuning_frequency(
    coordinates: &[(Coordinates, Coordinates)],
    max_search_space_coordinate: i64,
) -> Option<i64> {
    let sensor_radiuses: HashMap<_, _> = coordinates
        .iter()
        .map(|(sensor_coordinates, beacon_coordinates)| {
            (
                *sensor_coordinates,
                manhattan_distance(sensor_coordinates, beacon_coordinates),
            )
        })
        .collect();

    for y in 0..=max_search_space_coordinate {
        let mut segments: Vec<_> = sensor_radiuses
            .iter()
            .filter(|(&sensor_coordinates, &radius)| radius >= (sensor_coordinates.1 - y).abs())
            .map(|(&sensor_coordinates, &radius)| {
                let distance = radius - (sensor_coordinates.1 - y).abs();

                (
                    sensor_coordinates.0 - distance,
                    sensor_coordinates.0 + distance,
                )
            })
            .collect();

        let mut non_overlapping_segments = Vec::with_capacity(segments.len());

        while !segments.is_empty() {
            let mut segment = segments.pop().unwrap();
            let mut processed_segments = Vec::with_capacity(non_overlapping_segments.len() + 1);

            if segment.1 < 0 || segment.0 > max_search_space_coordinate {
                continue;
            } else {
                segment = (
                    i64::max(0, segment.0),
                    i64::min(segment.1, max_search_space_coordinate),
                );
            }

            while !non_overlapping_segments.is_empty() {
                let previous_segment: (i64, i64) = non_overlapping_segments.pop().unwrap();

                if previous_segment.1 < segment.0 || previous_segment.0 > segment.1 {
                    processed_segments.push(previous_segment);
                } else {
                    segment = (
                        i64::min(previous_segment.0, segment.0),
                        i64::max(previous_segment.1, segment.1),
                    );
                }
            }

            processed_segments.push(segment);
            non_overlapping_segments = processed_segments;
        }

        if non_overlapping_segments.len() == 2 {
            let x = i64::min(non_overlapping_segments[0].1, non_overlapping_segments[1].1) + 1;
            return Some(x * 4_000_000 + y);
        }
    }

    None
}

#[aoc(day15, part2)]
fn part2(coordinates: &[(Coordinates, Coordinates)]) -> Option<i64> {
    tuning_frequency(coordinates, 4_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn part1_example() {
        assert_eq!(non_beacon_positions(&parse_input(TEST_INPUT), 10), 26);
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            tuning_frequency(&parse_input(TEST_INPUT), 20),
            Some(56_000_011)
        );
    }
}
