use aoc_runner_derive::aoc;
use std::collections::HashMap;

const START_OF_PACKET_MARKER_LEN: usize = 4;
const START_OF_MESSAGE_MARKER_LEN: usize = 14;

fn find_marker(datastream_buffer: &str, marker_len: usize) -> Option<usize> {
    let mut character_frequencies: HashMap<_, _> = HashMap::with_capacity(marker_len);

    for index in 0..marker_len {
        character_frequencies
            .entry(&datastream_buffer.as_bytes()[index])
            .and_modify(|frequency| *frequency += 1)
            .or_insert(1usize);
    }
    let mut index = marker_len;

    while character_frequencies.len() < marker_len {
        if index >= datastream_buffer.len() {
            return None;
        }

        let removed_character = &datastream_buffer.as_bytes()[index - marker_len];
        let frequency = character_frequencies
            .entry(removed_character)
            .and_modify(|frequency| *frequency -= 1)
            .or_insert(0usize);

        if *frequency == 0 {
            character_frequencies.remove(removed_character);
        }

        let added_character = &datastream_buffer.as_bytes()[index];
        character_frequencies
            .entry(added_character)
            .and_modify(|frequency| *frequency += 1)
            .or_insert(1usize);

        index += 1;
    }

    Some(index)
}

#[aoc(day6, part1)]
fn part1(datastream_buffer: &str) -> Option<usize> {
    find_marker(datastream_buffer, START_OF_PACKET_MARKER_LEN)
}

#[aoc(day6, part2)]
fn part2(datastream_buffer: &str) -> Option<usize> {
    find_marker(datastream_buffer, START_OF_MESSAGE_MARKER_LEN)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    static TEST_INPUT_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    static TEST_INPUT_3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    static TEST_INPUT_4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    static TEST_INPUT_5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn part1_example() {
        assert_eq!(part1(TEST_INPUT_1), Some(7));
        assert_eq!(part1(TEST_INPUT_2), Some(5));
        assert_eq!(part1(TEST_INPUT_3), Some(6));
        assert_eq!(part1(TEST_INPUT_4), Some(10));
        assert_eq!(part1(TEST_INPUT_5), Some(11));
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(TEST_INPUT_1), Some(19));
        assert_eq!(part2(TEST_INPUT_2), Some(23));
        assert_eq!(part2(TEST_INPUT_3), Some(23));
        assert_eq!(part2(TEST_INPUT_4), Some(29));
        assert_eq!(part2(TEST_INPUT_5), Some(26));
    }
}
