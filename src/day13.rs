use crate::day13::PacketSymbol::*;
use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering::*;
use std::collections::VecDeque;

#[derive(Eq, PartialEq, Clone, Copy)]
enum PacketSymbol {
    LeftBracket,
    RightBracket,
    Integer(u32),
}

type Packet = VecDeque<PacketSymbol>;

static DIVIDER_PACKET_1: &str = "[[2]]";
static DIVIDER_PACKET_2: &str = "[[6]]";

fn parse_packet(packet: &str) -> Packet {
    let mut packet_symbols = VecDeque::with_capacity(packet.len());
    let mut integer_string = String::with_capacity(2);

    for symbol in packet.chars() {
        if !symbol.is_ascii_digit() {
            if !integer_string.is_empty() {
                packet_symbols.push_back(Integer(integer_string.parse().unwrap()));
                integer_string.clear();
            }

            match symbol {
                '[' => packet_symbols.push_back(LeftBracket),
                ']' => packet_symbols.push_back(RightBracket),
                ',' => (),
                _ => unreachable!(),
            }
        } else {
            integer_string.push(symbol);
        }
    }

    packet_symbols
}

#[aoc_generator(day13)]
fn parse_input(input: &str) -> Vec<(Packet, Packet)> {
    input
        .split("\n\n")
        .map(|packet_pair| {
            let mut packets = packet_pair.lines().map(|packet| parse_packet(packet));
            (packets.next().unwrap(), packets.next().unwrap())
        })
        .collect()
}

fn is_right_order(left: &Packet, right: &Packet) -> bool {
    let mut left = left.to_owned();
    let mut right = right.to_owned();

    while !left.is_empty() && !right.is_empty() {
        match left.pop_front().unwrap() {
            LeftBracket => match right.pop_front().unwrap() {
                LeftBracket => (),
                RightBracket => return false,
                Integer(right_integer) => {
                    right.push_front(RightBracket);
                    right.push_front(Integer(right_integer));
                }
            },
            RightBracket => match right.pop_front().unwrap() {
                LeftBracket | Integer(_) => return true,
                RightBracket => (),
            },
            Integer(left_integer) => match right.pop_front().unwrap() {
                LeftBracket => {
                    left.push_front(RightBracket);
                    left.push_front(Integer(left_integer));
                }
                RightBracket => return false,
                Integer(right_integer) => match left_integer.cmp(&right_integer) {
                    Less => return true,
                    Greater => return false,
                    Equal => (),
                },
            },
        }
    }

    true
}

#[aoc(day13, part1)]
fn part1(packet_pairs: &[(Packet, Packet)]) -> usize {
    packet_pairs
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| is_right_order(left, right))
        .map(|(index, _)| index + 1)
        .sum()
}

#[aoc(day13, part2)]
fn part2(packet_pairs: &[(Packet, Packet)]) -> usize {
    let packets: Vec<_> = packet_pairs
        .iter()
        .flat_map(|(left, right)| [left, right])
        .collect();

    let divider_packet_1 = parse_packet(DIVIDER_PACKET_1);
    let divider_packet_2 = parse_packet(DIVIDER_PACKET_2);

    (packets
        .iter()
        .filter(|packet| is_right_order(packet, &divider_packet_1))
        .count()
        + 1)
        * (packets
            .iter()
            .filter(|packet| is_right_order(packet, &divider_packet_2))
            .count()
            + 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 140);
    }
}
