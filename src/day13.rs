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
            let mut packets = packet_pair.lines().map(parse_packet);
            (packets.next().unwrap(), packets.next().unwrap())
        })
        .collect()
}

fn is_right_order(left: &Packet, right: &Packet) -> bool {
    let mut left = left.to_owned();
    let mut right = right.to_owned();

    while !left.is_empty() && !right.is_empty() {
        let left_symbol = left.pop_front().unwrap();
        let right_symbol = right.pop_front().unwrap();

        match (left_symbol, right_symbol) {
            (left_symbol, right_symbol) if left_symbol == right_symbol => (),
            (_, RightBracket) => return false,
            (RightBracket, _) => return true,
            (Integer(left_integer), Integer(right_integer)) => {
                match left_integer.cmp(&right_integer) {
                    Less => return true,
                    Greater => return false,
                    _ => (),
                }
            }
            (LeftBracket, Integer(integer)) => {
                right.push_front(RightBracket);
                right.push_front(Integer(integer));
            }
            (Integer(integer), LeftBracket) => {
                left.push_front(RightBracket);
                left.push_front(Integer(integer));
            }
            _ => unreachable!(),
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

    let (num_of_packets_before_divider_1, num_of_packets_before_divider_2) = packets.iter().fold(
        (0, 0),
        |(mut num_of_packets_before_divider_1, mut num_of_packets_before_divider_2), packet| {
            if is_right_order(packet, &divider_packet_1) {
                num_of_packets_before_divider_1 += 1;
                num_of_packets_before_divider_2 += 1; //the order of [[2]] is less than the order of [[6]]
            } else if is_right_order(packet, &divider_packet_2) {
                num_of_packets_before_divider_2 += 1;
            }

            (
                num_of_packets_before_divider_1,
                num_of_packets_before_divider_2,
            )
        },
    );

    (num_of_packets_before_divider_1 + 1) * (num_of_packets_before_divider_2 + 2)
    //the order of [[2]] is less than the order of [[6]]
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
