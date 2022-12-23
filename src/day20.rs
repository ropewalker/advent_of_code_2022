use aoc_runner_derive::{aoc, aoc_generator};

const FIRST_INDEX: usize = 1_000;
const SECOND_INDEX: usize = 2_000;
const THIRD_INDEX: usize = 3_000;
const DECRYPTION_KEY: i64 = 811_589_153;

#[aoc_generator(day20)]
fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|number| number.parse().unwrap())
        .collect()
}

fn wrap_to_next(index: i64, vec_len: usize) -> usize {
    let vec_len = vec_len as i64;
    let index = index % vec_len;

    (((index - 1) % vec_len + vec_len) % vec_len + 1) as usize
}

fn wrap(index: i64, vec_len: usize) -> usize {
    let vec_len = vec_len as i64;
    ((index % vec_len + vec_len) % vec_len) as usize
}

#[aoc(day20, part1)]
fn part1(original_order: &[i64]) -> i64 {
    let mut order = original_order.to_vec();
    let mut indexes: Vec<_> = original_order
        .iter()
        .enumerate()
        .map(|(index, _)| index)
        .collect();

    for i in 0..original_order.len() {
        let index = indexes.iter().position(|&element| element == i).unwrap();
        let number = order[index];

        order.remove(index);
        indexes.remove(index);

        let new_index = wrap_to_next(index as i64 + number, order.len());

        order.insert(new_index, number);
        indexes.insert(new_index, i);
    }

    let index_0 = order.iter().position(|&element| element == 0).unwrap();

    order[wrap((index_0 + FIRST_INDEX) as i64, order.len())]
        + order[wrap((index_0 + SECOND_INDEX) as i64, order.len())]
        + order[wrap((index_0 + THIRD_INDEX) as i64, order.len())]
}

#[aoc(day20, part2)]
fn part2(original_order: &[i64]) -> i64 {
    let mut order: Vec<_> = original_order
        .iter()
        .map(|element| *element * DECRYPTION_KEY)
        .collect();

    let mut indexes: Vec<_> = original_order
        .iter()
        .enumerate()
        .map(|(index, _)| index)
        .collect();

    for _ in 0..10 {
        for i in 0..original_order.len() {
            let index = indexes.iter().position(|&element| element == i).unwrap();
            let number = order[index];

            order.remove(index);
            indexes.remove(index);

            let new_index = wrap_to_next(index as i64 + number, order.len());

            order.insert(new_index, number);
            indexes.insert(new_index, i);
        }
    }

    let index_0 = order.iter().position(|&element| element == 0).unwrap();

    order[wrap((index_0 + FIRST_INDEX) as i64, order.len())]
        + order[wrap((index_0 + SECOND_INDEX) as i64, order.len())]
        + order[wrap((index_0 + THIRD_INDEX) as i64, order.len())]
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"1
2
-3
3
-2
0
4";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 1_623_178_306);
    }
}
