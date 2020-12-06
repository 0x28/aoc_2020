use aoc_2020::input_file;
use std::collections::HashSet;
use std::fs;

type Groups = Vec<HashSet<char>>;

fn parse(groups: &str) -> Groups {
    groups
        .split("\n\n")
        .map(|g| g.chars().filter(|c| c.is_alphabetic()).collect())
        .collect()
}

fn part1(groups: &[HashSet<char>]) -> usize {
    groups.iter().map(HashSet::len).sum()
}

fn part2(groups: &str) -> usize {
    groups
        .split("\n\n")
        .map(|g| {
            ('a'..='z')
                .filter(|&c| {
                    // check if every line contains the character c
                    g.chars().filter(|&x| x == c).count() == g.lines().count()
                })
                .count()
        })
        .sum()
}

fn main() {
    let input =
        fs::read_to_string(input_file("day6.txt")).expect("file not found!");
    let groups = parse(&input);
    println!("part1 = {}", part1(&groups));
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

    assert_eq!(part1(&parse(example)), 11);
    assert_eq!(part2(example), 6);
}
