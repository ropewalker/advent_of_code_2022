use crate::day07::CdArgument::*;
use crate::day07::Command::*;
use crate::day07::FileSystemObject::*;
use crate::day07::OutputLine::*;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

type FileSystemName = String;
type Path = Vec<FileSystemName>;

static ROOT: &str = "/";
const SMALL_DIR_SIZE_LIMIT: u32 = 100_000;
const TOTAL_DISK_SPACE: u32 = 70_000_000;
const UNUSED_DISK_SPACE_REQUIRED: u32 = 30_000_000;

#[derive(Clone)]
enum FileSystemObject {
    Dir,
    File { size: u32 },
}

enum CdArgument {
    In(FileSystemName),
    Out,
}

enum Command {
    List,
    ChangeDirectory(CdArgument),
}

enum OutputLine {
    ExecutedCommand(Command),
    ResultLine(FileSystemObject),
}

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<OutputLine> {
    input
        .lines()
        .map(|output_line| {
            let mut tokens = output_line.split_ascii_whitespace();
            match tokens.next().unwrap() {
                "$" => {
                    let command = tokens.next().unwrap();

                    match command {
                        "cd" => {
                            let argument = tokens.next().unwrap();
                            match argument {
                                ".." => ExecutedCommand(ChangeDirectory(Out)),
                                dir_name => {
                                    ExecutedCommand(ChangeDirectory(In(dir_name.to_string())))
                                }
                            }
                        }
                        "ls" => ExecutedCommand(List),
                        _ => unreachable!(),
                    }
                }
                "dir" => ResultLine(Dir),
                size => ResultLine(File {
                    size: size.parse().unwrap(),
                }),
            }
        })
        .collect()
}

fn dir_sizes(terminal_output: &[OutputLine]) -> HashMap<Path, u32> {
    let mut full_path: Path = Vec::new();
    let mut dir_sizes: HashMap<Path, u32> = HashMap::new();

    for output_line in terminal_output {
        match output_line {
            ExecutedCommand(command) => {
                if let ChangeDirectory(short_path) = command {
                    match short_path {
                        Out => {
                            full_path.pop();
                        }
                        In(dir_name) => {
                            full_path.push(dir_name.to_owned());
                        }
                    }
                }
            }
            ResultLine(result_line) => {
                if let File { size } = result_line {
                    for index in 0..full_path.len() {
                        dir_sizes
                            .entry(full_path[..=index].to_vec())
                            .and_modify(|curr_size| *curr_size += *size)
                            .or_insert(*size);
                    }
                }
            }
        };
    }

    dir_sizes
}

#[aoc(day7, part1)]
fn part1(terminal_output: &[OutputLine]) -> u32 {
    dir_sizes(terminal_output)
        .values()
        .filter(|&size| *size <= SMALL_DIR_SIZE_LIMIT)
        .sum()
}

#[aoc(day7, part2)]
fn part2(terminal_output: &[OutputLine]) -> u32 {
    let dir_sizes = dir_sizes(terminal_output);
    let root_size = *dir_sizes.get(&vec![ROOT.to_string()]).unwrap();
    let disk_space_available = TOTAL_DISK_SPACE - root_size;

    *dir_sizes
        .values()
        .filter(|dir_size| disk_space_available + **dir_size >= UNUSED_DISK_SPACE_REQUIRED)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 95_437);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 24_933_642);
    }
}
