use aoc_2020::input_file;
use std::fs;
use std::{collections::HashMap, num::ParseIntError};

type Bags = HashMap<String, Vec<(u32, String)>>;

fn parse(input: &str) -> Bags {
    let mut bags = HashMap::new();

    for line in input.lines() {
        if let [first, second, rest @ ..] =
            line.split_whitespace().collect::<Vec<&str>>().as_slice()
        {
            let inner_bags = rest
                .windows(3)
                .map::<Result<(u32, String), ParseIntError>, _>(|slice| {
                    Ok((
                        slice[0].parse::<u32>()?,
                        format!("{} {}", slice[1], slice[2]),
                    ))
                })
                .flatten()
                .collect();

            bags.insert(format!("{} {}", first, second), inner_bags);
        }
    }

    bags
}

fn part1(bags: &Bags) -> usize {
    fn has_gold(bag: &str, bags: &Bags) -> bool {
        bags[bag]
            .iter()
            .any(|(_, b)| b == "shiny gold" || has_gold(b, bags))
    }

    bags.keys().filter(|b| has_gold(b, bags)).count()
}

fn part2(bags: &Bags) -> u32 {
    fn count_bags(bag: &str, bags: &Bags) -> u32 {
        bags[bag]
            .iter()
            .map(|(n, b)| n * count_bags(b, bags))
            .sum::<u32>()
            + 1
    }

    count_bags("shiny gold", bags) - 1
}

fn main() {
    let bags = parse(&fs::read_to_string(input_file("day7.txt")).unwrap());
    println!("part1 = {}", part1(&bags));
    println!("part2 = {}", part2(&bags));
}

#[test]
fn test_day7() {
    let example1 = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";
    let example1 = parse(example1);

    let example2 = "\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
";
    let example2 = parse(example2);

    assert_eq!(part1(&example1), 4);
    assert_eq!(part2(&example1), 32);
    assert_eq!(part2(&example2), 126);
}
