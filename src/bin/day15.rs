use std::collections::HashMap;

fn parse(input: &str) -> Vec<usize> {
    input.split(',').flat_map(str::parse).collect()
}

fn day15(nums: &[usize], n: usize) -> usize {
    let mut lastpos = HashMap::new();

    for (i, &n) in nums.iter().enumerate().take(nums.len() - 1) {
        lastpos.insert(n, i);
    }
    let mut last = *nums.last().unwrap();
    let mut current = 0;

    for i in nums.len()..n {
        current = if let Some(pos) = lastpos.get(&last) {
            i - 1 - pos
        } else {
            0
        };

        lastpos.insert(last, i - 1);
        last = current;
    }

    current
}

fn main() {
    let puzzle_input = "0,6,1,7,2,19,20";

    println!("part1 = {}", day15(&parse(puzzle_input), 2020));
    println!("part2 = {}", day15(&parse(puzzle_input), 30_000_000));
}

#[test]
fn test_day15() {
    assert_eq!(day15(&parse("0,3,6"), 2020), 436);
    assert_eq!(day15(&parse("1,3,2"), 2020), 1);
    assert_eq!(day15(&parse("2,1,3"), 2020), 10);
    assert_eq!(day15(&parse("1,2,3"), 2020), 27);
    assert_eq!(day15(&parse("2,3,1"), 2020), 78);
    assert_eq!(day15(&parse("3,2,1"), 2020), 438);
    assert_eq!(day15(&parse("3,1,2"), 2020), 1836);
}
