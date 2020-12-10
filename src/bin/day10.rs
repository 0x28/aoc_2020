use aoc_2020::input_file;
use std::{collections::HashMap, fs};

fn parse(input: &str) -> Vec<u64> {
    let mut vec: Vec<_> = input.lines().map(str::parse).flatten().collect();
    vec.push(0); // outlet
    vec.sort_unstable();
    vec
}

fn part1(nums: &[u64]) -> u64 {
    let (ones, _, threes) =
        nums.windows(2)
            .fold((0, 0, 1), |(ones, twos, threes), window| {
                match window[1] - window[0] {
                    1 => (ones + 1, twos, threes),
                    2 => (ones, twos + 1, threes),
                    3 => (ones, twos, threes + 1),
                    _ => panic!("invalid input!"),
                }
            });

    ones * threes
}

fn part2(nums: &[u64]) -> u64 {
    fn helper(memoize: &mut HashMap<u64, u64>, nums: &[u64]) -> u64 {
        if nums.len() == 1 {
            return 1;
        }

        if let Some(&result) = memoize.get(&nums[0]) {
            return result;
        }

        let sum = (1..=3)
            .map(|i| {
                if nums.get(i)? - nums.get(0)? <= 3 {
                    Some(helper(memoize, &nums[i..]))
                } else {
                    None
                }
            })
            .flatten()
            .sum();

        memoize.insert(nums[0], sum);

        sum
    }

    helper(&mut HashMap::new(), nums)
}

fn main() {
    let input = fs::read_to_string(input_file("day10.txt")).unwrap();
    let input = parse(&input);

    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day10() {
    let example1 = "\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
";
    let example1 = parse(example1);

    let example2 = "\
16
10
15
5
1
11
7
19
6
12
4
";
    let example2 = parse(example2);

    assert_eq!(part1(&example1), 220);
    assert_eq!(part2(&example1), 19208);
    assert_eq!(part2(&example2), 8);
}
