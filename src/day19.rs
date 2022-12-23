use crate::day19::ResourceType::*;
use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::cmp::Ordering::*;
use std::ops::SubAssign;

#[derive(Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd, Debug)]
enum ResourceType {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

impl TryFrom<usize> for ResourceType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Ore),
            1 => Ok(Clay),
            2 => Ok(Obsidian),
            3 => Ok(Geode),
            _ => Err(()),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug, Default)]
struct ResourcesCombination([usize; 4]);

impl PartialOrd for ResourcesCombination {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Equal)
        } else if self.0.iter().zip(other.0.iter()).all(|(x, y)| x <= y) {
            Some(Less)
        } else if self.0.iter().zip(other.0.iter()).all(|(x, y)| x >= y) {
            Some(Greater)
        } else {
            None
        }
    }
}

impl SubAssign<&ResourcesCombination> for ResourcesCombination {
    fn sub_assign(&mut self, rhs: &Self) {
        if self.0.partial_cmp(&rhs.0) == Some(Less) || self.0.partial_cmp(&rhs.0).is_none() {
            panic!();
        }

        self.0 = [
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
            self.0[3] - rhs.0[3],
        ];
    }
}

impl ResourcesCombination {
    fn resource_quantity(&self, resource_type: &ResourceType) -> usize {
        self.0[*resource_type as usize]
    }

    fn add_resource(&mut self, resource_type: ResourceType, quantity: usize) {
        self.0[resource_type as usize] += quantity;
    }
}

#[derive(Clone)]
struct State {
    time: usize,
    robots: [usize; 4],
    resources: ResourcesCombination,
}

struct Blueprint {
    id: usize,
    robot_costs: [ResourcesCombination; 4],
}

impl Blueprint {
    fn cost(&self, robot: &ResourceType) -> &ResourcesCombination {
        &self.robot_costs[*robot as usize]
    }

    fn highest_cost(&self, resource_type: &ResourceType) -> usize {
        self.robot_costs
            .iter()
            .map(|cost| cost.resource_quantity(resource_type))
            .max()
            .unwrap()
    }

    fn quality_level(&self) -> usize {
        self.max_geodes(24) * self.id
    }

    fn max_geodes(&self, time_left: usize) -> usize {
        let mut states = Vec::new();

        states.push(State {
            time: 0,
            robots: [1, 0, 0, 0],
            resources: Default::default(),
        });

        let mut max_geodes_quantity = 0;

        while !states.is_empty() {
            let state = states.pop().unwrap();

            'outer: for next_robot in 0usize..4 {
                let next_robot: ResourceType = next_robot.try_into().unwrap();
                let cost = self.cost(&next_robot);

                if next_robot != Geode
                    && state.robots[next_robot as usize] * (time_left - state.time)
                        + state.resources.resource_quantity(&next_robot)
                        >= self.highest_cost(&next_robot) * (time_left - state.time)
                {
                    continue;
                }

                let mut time = state.time;
                let mut resources = state.resources;

                while resources.partial_cmp(cost) != Some(Greater)
                    && resources.partial_cmp(cost) != Some(Equal)
                {
                    time += 1;
                    for resource_type in 0..3 {
                        resources.add_resource(
                            resource_type.try_into().unwrap(),
                            state.robots[resource_type],
                        );
                    }

                    if time >= time_left - 1 {
                        continue 'outer;
                    }
                }

                resources -= cost;
                let mut robots = state.robots;

                for (resource_type, quantity) in robots.iter().enumerate().take(3) {
                    resources.add_resource(resource_type.try_into().unwrap(), *quantity);
                }

                time += 1;

                if next_robot == Geode {
                    resources.add_resource(Geode, time_left - time);
                    max_geodes_quantity =
                        usize::max(max_geodes_quantity, resources.resource_quantity(&Geode));
                }

                robots[next_robot as usize] += 1;

                if time < time_left {
                    states.push(State {
                        time,
                        robots,
                        resources,
                    });
                }
            }
        }

        max_geodes_quantity
    }
}

#[aoc_generator(day19)]
fn parse_input(input: &str) -> Vec<Blueprint> {
    use aoc_parse::{parser, prelude::*};

    parser!(lines(
        "Blueprint " id:usize ": "
        "Each ore robot costs " ore_robot_ore_cost:usize " ore. "
        "Each clay robot costs " clay_robot_ore_cost:usize " ore. "
        "Each obsidian robot costs "
            obsidian_robot_ore_cost:usize " ore and "
            obsidian_robot_clay_cost:usize " clay. "
        "Each geode robot costs "
            geode_robot_ore_cost:usize " ore and "
            geode_robot_obsidian_cost:usize " obsidian." =>
            Blueprint {
                id,
                robot_costs: [
                    ResourcesCombination([ore_robot_ore_cost, 0, 0, 0]),
                    ResourcesCombination([clay_robot_ore_cost, 0, 0, 0]),
                    ResourcesCombination([obsidian_robot_ore_cost, obsidian_robot_clay_cost, 0, 0]),
                    ResourcesCombination([geode_robot_ore_cost, 0, geode_robot_obsidian_cost, 0]),
                ]
            }
    ))
    .parse(input)
    .unwrap()
}

#[aoc(day19, part1)]
fn part1(blueprints: &[Blueprint]) -> usize {
    blueprints
        .iter()
        .map(|blueprint| blueprint.quality_level())
        .sum()
}

#[aoc(day19, part2)]
fn part2(blueprints: &[Blueprint]) -> usize {
    blueprints
        .iter()
        .take(3)
        .map(|blueprint| blueprint.max_geodes(32))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 33);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(parse_input(TEST_INPUT)[0].max_geodes(32), 56);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(parse_input(TEST_INPUT)[1].max_geodes(32), 62);
    }
}
