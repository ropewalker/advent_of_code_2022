use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{BTreeSet, HashMap};

#[derive(Debug)]
struct Valve {
    label: String,
    flow_rate: u32,
    connected_valves: Vec<String>,
}

#[aoc_generator(day16)]
fn parse_input(input: &str) -> Vec<Valve> {
    use aoc_parse::{parser, prelude::*};

    let valves_parser = parser!(repeat_sep(string(upper+), ", "));

    let parser = parser!(lines(
        "Valve " label:string(upper+)
        " has flow rate=" flow_rate:u32
        "; " {"tunnels lead to valves ", "tunnel leads to valve "} connected_valves:valves_parser =>
            Valve {
                label,
                flow_rate,
                connected_valves
            }
    ));
    parser.parse(input).unwrap()
}

fn floyd_warshall(distances: &mut Vec<Vec<Option<u32>>>) {
    for k in 0..distances.len() {
        for i in 0..distances.len() {
            for j in 0..distances.len() {
                if distances[i][k].is_some() && distances[k][j].is_some() {
                    let new_distance = distances[i][k].unwrap() + distances[k][j].unwrap();

                    match distances[i][j] {
                        None => {
                            distances[i][j] = Some(new_distance);
                        }
                        Some(distance) if distance > new_distance => {
                            distances[i][j] = Some(new_distance);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
struct Node {
    current_index: usize,
    total_released_pressure: u32,
    closed_functioning_valves: BTreeSet<usize>,
    time_left: u32,
}

struct Setup {
    functioning_valves: BTreeSet<usize>,
    distances: Vec<Vec<Option<u32>>>,
    flow_rates: Vec<u32>,
    time_to_eruption: u32,
    starting_valve_index: usize,
}

impl Setup {
    fn new(valves: &[Valve], starting_valve_label: String, time_to_eruption: u32) -> Self {
        let mut distances: Vec<Vec<Option<u32>>> = vec![vec![None; valves.len()]; valves.len()];
        let mut functioning_valves: BTreeSet<usize> = BTreeSet::new();

        let dictionary: HashMap<_, _> = valves
            .iter()
            .enumerate()
            .map(|(index, valve)| (valve.label.to_owned(), index))
            .collect();
        let mut flow_rates = Vec::with_capacity(valves.len());
        let mut starting_valve_index = 0;

        for (index, valve) in valves.iter().enumerate() {
            if valve.label == starting_valve_label {
                starting_valve_index = index;
            }

            if valve.flow_rate > 0 {
                functioning_valves.insert(index);
            }

            flow_rates.push(valve.flow_rate);

            distances[index][index] = Some(0);

            for connected_valve_label in valve.connected_valves.iter() {
                let other_index = *dictionary.get(connected_valve_label).unwrap();

                distances[index][other_index] = Some(1);
            }
        }

        floyd_warshall(&mut distances);

        Self {
            functioning_valves,
            distances,
            flow_rates,
            time_to_eruption,
            starting_valve_index,
        }
    }
}

#[aoc(day16, part1)]
fn part1(valves: &[Valve]) -> u32 {
    const STARTING_VALVE_LABEL: &str = "AA";
    const TIME_TO_ERUPTION: u32 = 30;

    let setup = Setup::new(valves, STARTING_VALVE_LABEL.to_string(), TIME_TO_ERUPTION);

    let mut nodes = Vec::new();

    for functioning_valve_index in setup.functioning_valves.iter() {
        let distance =
            setup.distances[setup.starting_valve_index][*functioning_valve_index].unwrap();

        if distance < setup.time_to_eruption {
            let time_left = setup.time_to_eruption - distance - 1;
            let mut closed_functioning_valves = setup.functioning_valves.clone();
            closed_functioning_valves.remove(functioning_valve_index);

            nodes.push(Node {
                current_index: *functioning_valve_index,
                total_released_pressure: setup.flow_rates[*functioning_valve_index] * time_left,
                closed_functioning_valves,
                time_left,
            });
        }
    }

    let mut max_total_released_pressure = 0;

    while !nodes.is_empty() {
        let node = nodes.pop().unwrap();

        let mut more_closed_valves_in_reach = false;

        for closed_valve_index in node.closed_functioning_valves.iter() {
            let distance = setup.distances[node.current_index][*closed_valve_index].unwrap();

            if distance < node.time_left {
                more_closed_valves_in_reach = true;

                let time_left = node.time_left - distance - 1;
                let mut closed_functioning_valves = node.closed_functioning_valves.clone();
                closed_functioning_valves.remove(closed_valve_index);

                nodes.push(Node {
                    current_index: *closed_valve_index,
                    total_released_pressure: node.total_released_pressure
                        + setup.flow_rates[*closed_valve_index] * time_left,
                    closed_functioning_valves,
                    time_left,
                });
            }
        }

        if !more_closed_valves_in_reach {
            max_total_released_pressure =
                u32::max(max_total_released_pressure, node.total_released_pressure);
        }
    }

    max_total_released_pressure
}

#[aoc(day16, part2)]
fn part2(valves: &[Valve]) -> u32 {
    const STARTING_VALVE_LABEL: &str = "AA";
    const TIME_TO_ERUPTION: u32 = 26;

    let setup = Setup::new(valves, STARTING_VALVE_LABEL.to_string(), TIME_TO_ERUPTION);

    let mut nodes = Vec::new();

    for functioning_valve_index in setup.functioning_valves.iter() {
        let distance =
            setup.distances[setup.starting_valve_index][*functioning_valve_index].unwrap();

        if distance < setup.time_to_eruption {
            let time_left = setup.time_to_eruption - distance - 1;
            let mut closed_functioning_valves = setup.functioning_valves.clone();
            closed_functioning_valves.remove(functioning_valve_index);

            nodes.push(Node {
                current_index: *functioning_valve_index,
                total_released_pressure: setup.flow_rates[*functioning_valve_index] * time_left,
                closed_functioning_valves,
                time_left,
            });
        }
    }

    let mut max_total_pressures: HashMap<BTreeSet<usize>, u32> = HashMap::new();

    while !nodes.is_empty() {
        let node = nodes.pop().unwrap();

        let max_total_pressure = max_total_pressures
            .entry(
                setup
                    .functioning_valves
                    .difference(&node.closed_functioning_valves.clone())
                    .copied()
                    .collect::<BTreeSet<_>>(),
            )
            .or_insert(0);
        if *max_total_pressure < node.total_released_pressure {
            *max_total_pressure = node.total_released_pressure;
        }

        for closed_valve_index in node.closed_functioning_valves.iter() {
            let distance = setup.distances[node.current_index][*closed_valve_index].unwrap();

            if distance < node.time_left {
                let time_left = node.time_left - distance - 1;
                let mut closed_functioning_valves = node.closed_functioning_valves.clone();
                closed_functioning_valves.remove(closed_valve_index);

                nodes.push(Node {
                    current_index: *closed_valve_index,
                    total_released_pressure: node.total_released_pressure
                        + setup.flow_rates[*closed_valve_index] * time_left,
                    closed_functioning_valves,
                    time_left,
                });
            }
        }
    }

    let mut max_total_released_pressure = 0;

    for (human_valves, human_total_pressure) in max_total_pressures.iter() {
        for (elephant_valves, elephant_total_pressure) in max_total_pressures.iter() {
            if human_valves.intersection(elephant_valves).count() == 0 {
                max_total_released_pressure = u32::max(
                    max_total_released_pressure,
                    *human_total_pressure + *elephant_total_pressure,
                );
            }
        }
    }

    max_total_released_pressure
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 1_651);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 1_707);
    }
}
