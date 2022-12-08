use crate::day08::Axis::*;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

type Coordinates = (usize, usize);
type TreeHeight = u32;

struct Grid {
    tree_heights: Vec<Vec<TreeHeight>>,
    width: usize,
    depth: usize,
}

enum Axis {
    X,
    Y,
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Grid {
    Grid {
        tree_heights: input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|tree| tree.to_digit(10).unwrap())
                    .collect()
            })
            .collect(),
        width: input.lines().next().unwrap().len(),
        depth: input.lines().count(),
    }
}

fn trees_visible_along_axis(grid: &Grid, axis: Axis) -> Vec<Coordinates> {
    let mut visible_trees = Vec::with_capacity(grid.depth * grid.width);

    let (len_a, len_b) = match axis {
        X => (grid.depth, grid.width),
        Y => (grid.width, grid.depth),
    };

    for coordinate_a in 0..len_a {
        let (x, y) = match axis {
            X => (0, coordinate_a),
            Y => (coordinate_a, 0),
        };

        visible_trees.push((x, y));
        let mut top_height = grid.tree_heights[y][x];
        let mut previous_tree_height = top_height;
        let mut maybe_visible_trees = Vec::with_capacity(len_b);

        for coordinate_b in 1..len_b {
            let (x, y) = match axis {
                X => (coordinate_b, coordinate_a),
                Y => (coordinate_a, coordinate_b),
            };

            let tree_height = grid.tree_heights[y][x];

            if tree_height >= top_height {
                maybe_visible_trees.clear();
            } else if tree_height < previous_tree_height {
                previous_tree_height = tree_height;
            } else {
                while let Some((_, height)) = maybe_visible_trees.last() {
                    if *height > tree_height {
                        break;
                    } else {
                        maybe_visible_trees.pop();
                    }
                }
            }

            maybe_visible_trees.push(((x, y), tree_height));

            if tree_height > top_height {
                visible_trees.push((x, y));
                top_height = tree_height;
            }
        }

        visible_trees.extend(
            maybe_visible_trees
                .into_iter()
                .map(|(coordinates, _)| coordinates),
        );
    }

    visible_trees
}

#[aoc(day8, part1)]
fn part1(grid: &Grid) -> usize {
    let trees_visible_along_x = trees_visible_along_axis(grid, X);
    let trees_visible_along_y = trees_visible_along_axis(grid, Y);

    trees_visible_along_x
        .into_iter()
        .chain(trees_visible_along_y.into_iter())
        .collect::<HashSet<_>>()
        .len()
}

#[aoc(day8, part2, clever)]
fn part2_clever(grid: &Grid) -> usize {
    let mut scenic_scores = vec![vec![0; grid.width]; grid.depth];
    let mut stack = Vec::with_capacity(usize::max(grid.width - 2, grid.depth - 2));

    for (y, x_scores) in scenic_scores
        .iter_mut()
        .enumerate()
        .take(grid.depth - 1)
        .skip(1)
    {
        'outer: for (x, scenic_score) in
            x_scores.iter_mut().enumerate().take(grid.width - 1).skip(1)
        {
            while !stack.is_empty() {
                let last = *stack.last().unwrap();

                if grid.tree_heights[y][last] < grid.tree_heights[y][x] {
                    stack.pop();
                } else {
                    *scenic_score = x - last;
                    stack.push(x);
                    continue 'outer;
                }
            }

            *scenic_score = x;
            stack.push(x);
        }

        stack.clear();
    }

    for (y, x_scores) in scenic_scores
        .iter_mut()
        .enumerate()
        .take(grid.depth - 1)
        .skip(1)
    {
        'outer: for x in (1..grid.width - 1).rev() {
            while !stack.is_empty() {
                let last = *stack.last().unwrap();

                if grid.tree_heights[y][last] < grid.tree_heights[y][x] {
                    stack.pop();
                } else {
                    x_scores[x] *= last - x;
                    stack.push(x);
                    continue 'outer;
                }
            }

            x_scores[x] *= grid.width - 1 - x;
            stack.push(x);
        }

        stack.clear();
    }

    for x in 1..grid.width - 1 {
        'outer: for (y, x_scores) in scenic_scores
            .iter_mut()
            .enumerate()
            .take(grid.depth - 1)
            .skip(1)
        {
            while !stack.is_empty() {
                let last: usize = *stack.last().unwrap();

                if grid.tree_heights[last][x] < grid.tree_heights[y][x] {
                    stack.pop();
                } else {
                    x_scores[x] *= y - last;

                    stack.push(y);
                    continue 'outer;
                }
            }

            x_scores[x] *= y;
            stack.push(y);
        }

        stack.clear();
    }

    for x in 1..grid.width - 1 {
        'outer: for y in (1..grid.depth - 1).rev() {
            while !stack.is_empty() {
                let last: usize = *stack.last().unwrap();

                if grid.tree_heights[last][x] < grid.tree_heights[y][x] {
                    stack.pop();
                } else {
                    scenic_scores[y][x] *= last - y;
                    stack.push(y);
                    continue 'outer;
                }
            }

            scenic_scores[y][x] *= grid.depth - 1 - y;
            stack.push(y);
        }

        stack.clear();
    }

    *scenic_scores.iter().flatten().max().unwrap()
}

#[aoc(day8, part2, naive)]
fn part2_naive(grid: &Grid) -> usize {
    let mut scenic_scores = Vec::with_capacity(grid.width * grid.depth);

    for tree_x in 0..grid.width {
        for tree_y in 0..grid.depth {
            let mut scenic_score = 1;
            let tree_height = grid.tree_heights[tree_y][tree_x];

            let mut visible_trees_count = 0;

            for x in (0..tree_x).rev() {
                visible_trees_count += 1;

                if grid.tree_heights[tree_y][x] >= tree_height {
                    break;
                }
            }

            scenic_score *= visible_trees_count;
            visible_trees_count = 0;

            for x in tree_x + 1..grid.width {
                visible_trees_count += 1;

                if grid.tree_heights[tree_y][x] >= tree_height {
                    break;
                }
            }

            scenic_score *= visible_trees_count;
            visible_trees_count = 0;

            for y in (0..tree_y).rev() {
                visible_trees_count += 1;

                if grid.tree_heights[y][tree_x] >= tree_height {
                    break;
                }
            }

            scenic_score *= visible_trees_count;
            visible_trees_count = 0;

            for y in tree_y + 1..grid.depth {
                visible_trees_count += 1;

                if grid.tree_heights[y][tree_x] >= tree_height {
                    break;
                }
            }

            scenic_score *= visible_trees_count;
            scenic_scores.push(scenic_score);
        }
    }

    *scenic_scores.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"30373
25512
65332
33549
35390";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 21);
    }

    #[test]
    fn part2_clever_example() {
        assert_eq!(part2_clever(&parse_input(TEST_INPUT)), 8);
    }

    #[test]
    fn part2_naive_example() {
        assert_eq!(part2_naive(&parse_input(TEST_INPUT)), 8);
    }
}
