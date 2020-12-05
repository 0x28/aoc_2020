use aoc_2020::input_file;
use std::fs;

fn decode_pass(pass: &str) -> usize {
    pass.chars().fold(0, |n, c| {
        n * 2
            + match c {
                'F' | 'L' => 0,
                'B' | 'R' => 1,
                _ => panic!("unknown pattern"),
            }
    })
}

fn part1(passes: &str) -> usize {
    passes.lines().map(decode_pass).max().unwrap()
}

fn part2(passes: &str) -> usize {
    let mut seats = [false; 1024]; // poor mans hash map :)

    for pass in passes.lines() {
        seats[decode_pass(pass)] = true;
    }

    for idx in 0..seats.len() {
        match (seats.get(idx), seats.get(idx + 1), seats.get(idx + 2)) {
            (Some(true), Some(false), Some(true)) => return idx + 1,
            _ => (),
        }
    }

    panic!("index not found!");
}

fn main() {
    let input =
        fs::read_to_string(input_file("day5.txt")).expect("file not found!");

    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day5() {
    assert_eq!(decode_pass("BFFFBBFRRR"), 567);
    assert_eq!(decode_pass("FFFBBBFRRR"), 119);
    assert_eq!(decode_pass("BBFFBBFRLL"), 820);
}
