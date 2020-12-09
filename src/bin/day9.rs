use aoc_2020::input_file;

use cmp::Ordering;
use std::fs;
use std::i32;
use std::{cmp, collections::HashSet};

fn parse(input: &str) -> Vec<i32> {
    input.lines().map(str::parse).flatten().collect()
}

fn find_sum(nums: &[i32], sum: i32) -> Option<(i32, i32)> {
    let mut lookup = HashSet::new();

    for &n in nums {
        match lookup.get(&(sum - n)) {
            Some(&v) => return Some((v, n)),
            None => lookup.insert(n),
        };
    }

    None
}

fn part1(nums: &[i32], preamble: usize) -> i32 {
    nums.windows(preamble + 1)
        .map(|window| {
            if let [start @ .., sum] = window {
                match find_sum(start, *sum) {
                    Some(_) => None,
                    None => Some(*sum),
                }
            } else {
                None
            }
        })
        .flatten()
        .next()
        .unwrap()
}

fn part2(nums: &[i32], preamble: usize) -> i32 {
    let mut begin = 0;
    let mut end = 0;
    let mut sum = 0;
    let weakness = part1(nums, preamble);

    loop {
        match sum.cmp(&weakness) {
            Ordering::Greater => {
                sum -= nums[begin];
                begin += 1;
            }
            Ordering::Equal => break,
            Ordering::Less => {
                sum += nums[end];
                end += 1;
            }
        }
    }

    let (min, max) = nums[begin..end]
        .iter()
        .fold((i32::MAX, i32::MIN), |(min, max), &v| {
            (cmp::min(v, min), cmp::max(v, max))
        });

    min + max
}

fn main() {
    let numbers = fs::read_to_string(input_file("day9.txt")).unwrap();
    let numbers = parse(&numbers);

    println!("part1 = {}", part1(&numbers, 25));
    println!("part2 = {}", part2(&numbers, 25));
}

#[test]
fn test_day9() {
    let example1 = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
";

    assert_eq!(part1(&parse(example1), 5), 127);
    assert_eq!(part2(&parse(example1), 5), 62);
}
