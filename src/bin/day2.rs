use aoc_2020::input_file;

use std::fs;

type PasswordEntry = (usize, usize, char, String);

fn parse_line(line: &str) -> Option<PasswordEntry> {
    let mut token: Vec<&str> = line.split_whitespace().collect();
    let passwd = token.pop()?;
    let character = token.pop()?.chars().nth(0)?;
    let range: Vec<&str> = token.pop()?.split('-').collect();

    Some((
        range.get(0)?.parse().ok()?,
        range.get(1)?.parse().ok()?,
        character,
        passwd.to_owned(),
    ))
}

fn read_input_file() -> Vec<PasswordEntry> {
    let input = fs::read_to_string(input_file("day2.txt"));

    input
        .expect("couldn't read input file")
        .lines()
        .map(parse_line)
        .flatten()
        .collect()
}

fn password_valid(entry: &PasswordEntry) -> bool {
    let (min, max, character, passwd) = entry;
    let count = passwd.chars().filter(|c| c == character).count();

    count >= *min && count <= *max
}

fn part1() -> usize {
    read_input_file()
        .iter()
        .filter(|p| password_valid(p))
        .count()
}

fn password_valid2(entry: &PasswordEntry) -> bool {
    let (idx1, idx2, character, passwd) = entry;
    let first = passwd.chars().nth(idx1-1).unwrap_or_default();
    let second = passwd.chars().nth(idx2-1).unwrap_or_default();

    (first == *character) ^ (second == *character)
}

fn part2() -> usize {
    read_input_file()
        .iter()
        .filter(|p| password_valid2(p))
        .count()
}

fn main() {
    println!("part1 = {}", part1());
    println!("part2 = {}", part2());
}

#[test]
fn test_parse_line() {
    assert_eq!(password_valid(&parse_line("1-3 a: abcde").unwrap()), true);
    assert_eq!(password_valid(&parse_line("1-3 b: cdefg").unwrap()), false);
    assert_eq!(
        password_valid(&parse_line("2-9 c: ccccccccc").unwrap()),
        true
    );
}
