use aoc_2020::input_file;
use std::collections::HashSet;
use std::fs;

fn part1(groups: &str) -> usize {
    groups
        .split("\n\n")
        .map(|g| {
            g.chars()
                .filter(|c| c.is_alphabetic())
                .collect::<HashSet<char>>()
                .len()
        })
        .sum()
}

fn part2(groups: &str) -> usize {
    groups
        .split("\n\n")
        .map(|g| {
            g.lines()
                .fold(('a'..='z').collect(), |set: HashSet<char>, line| {
                    set.intersection(&line.chars().collect()).copied().collect()
                })
                .len()
        })
        .sum()
}

fn main() {
    let input =
        fs::read_to_string(input_file("day6.txt")).expect("file not found!");
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day6() {
    let example = "\
abc

a
b
c

ab
ac

a
a
a
a

b";

    assert_eq!(part1(&example), 11);
    assert_eq!(part2(example), 6);
}
