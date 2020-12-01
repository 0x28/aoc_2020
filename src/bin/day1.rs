use aoc_2020::input_file;

use std::fs;

fn combinations(mut nums: &[u32]) -> Vec<(u32, u32)> {
    let mut result = vec![];
    loop {
        match nums.split_first() {
            Some((first, rest)) => {
                for pair in rest.iter().map(|elem| (*first, *elem)) {
                    result.push(pair)
                }
                nums = rest;
            }
            None => break,
        }
    }

    result
}

fn solution(nums: &[u32]) -> Option<u32> {
    for result in combinations(nums)
        .into_iter()
        .filter(|(a, b)| a + b == 2020)
        .map(|(a, b)| a * b)
    {
        return Some(result);
    }

    None
}

fn read_input_file() -> Vec<u32> {
    let input = fs::read_to_string(input_file("day1.txt"));

    input
        .expect("couldn't read input file")
        .split("\n")
        .map(str::parse)
        .flatten() // throw away Err values
        .collect()
}

fn main() {
    if let Some(result) = solution(&read_input_file()) {
        println!("{}", result);
    } else {
        eprintln!("no solution found!");
    }
}

#[test]
fn test_day1() {
    assert_eq!(solution(&[2019, 1]), Some(2019));
    assert_eq!(solution(&[1721, 979, 366, 299, 675, 1465]), Some(514579));
    assert_eq!(solution(&read_input_file()), Some(651651));
}
