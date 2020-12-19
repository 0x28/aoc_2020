use aoc_2020::input_file;
use std::fs;
use std::{collections::HashMap, mem};

#[derive(Debug, PartialEq, Clone)]
enum Rule {
    Literal(String),
    Alternative(Vec<Vec<usize>>),
}

fn parse_rule(line: &str) -> Rule {
    let tokens = line.split_whitespace().collect::<Vec<_>>();
    let mut alternatives = Vec::new();
    let mut sequence = Vec::new();

    for token in tokens {
        if let Ok(idx) = token.parse::<usize>() {
            sequence.push(idx);
        } else if token.starts_with('"') {
            return Rule::Literal(
                token.chars().filter(|c| c.is_alphabetic()).collect(),
            );
        } else if token == "|" {
            let mut current = Vec::new();
            mem::swap(&mut current, &mut sequence);
            alternatives.push(current)
        } else {
            unreachable!()
        }
    }

    alternatives.push(sequence);

    Rule::Alternative(alternatives)
}

fn parse_rules(input: &str) -> Vec<Rule> {
    let mut rules = HashMap::new();

    for line in input.lines() {
        if let [idx, rule, ..] = line.split(':').collect::<Vec<_>>().as_slice()
        {
            let idx = idx.parse::<usize>().unwrap();

            rules.insert(idx, parse_rule(rule));
        }
    }

    let mut rule_vec =
        vec![Rule::Literal(String::from("")); *rules.keys().max().unwrap() + 1];
    let mut keys = rules.keys().copied().collect::<Vec<_>>();
    keys.sort_unstable();

    for key in keys {
        rule_vec[key] = rules.remove(&key).unwrap();
    }

    rule_vec
}

fn parse(input: &str) -> (Vec<Rule>, Vec<String>) {
    if let [rules, messages, ..] =
        input.split("\n\n").collect::<Vec<_>>().as_slice()
    {
        (
            parse_rules(rules),
            messages.lines().map(str::to_owned).collect(),
        )
    } else {
        unreachable!()
    }
}

fn valid(
    message: &str,
    pos: usize,
    current: &Rule,
    rules: &[Rule],
) -> Option<usize> {
    match current {
        Rule::Alternative(alternatives) => {
            for alternative in alternatives {
                let mut current_pos = pos;
                let mut failed = false;
                for idx in alternative {
                    if let Some(pos) =
                        valid(&message, current_pos, &rules[*idx], rules)
                    {
                        current_pos = pos;
                    } else {
                        failed = true;
                        break;
                    }
                }

                if !failed {
                    return Some(current_pos);
                }
            }

            None
        }
        Rule::Literal(lit) => {
            if message[pos..].starts_with(lit) {
                Some(pos + lit.len())
            } else {
                None
            }
        }
    }
}

fn valid_message(message: &str, rules: &[Rule]) -> bool {
    if let Some(pos) = valid(message, 0, &rules[0], rules) {
        pos == message.len()
    } else {
        false
    }
}

fn part1(messages: &[String], rules: &[Rule]) -> usize {
    messages
        .iter()
        .filter(|msg| valid_message(msg, rules))
        .count()
}

fn main() {
    let input1 = &fs::read_to_string(input_file("day19.txt")).unwrap();
    let (rules, messages) = parse(input1);
    println!("part1 = {}", part1(&messages, &rules));
}

#[test]
fn test_day19() {
    let example1 = "\
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb
";
    let (rules, messages) = parse(example1);

    assert_eq!(part1(&messages, &rules), 2);
}
