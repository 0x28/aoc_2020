use aoc_2020::input_file;
use std::{cmp, fs};
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

fn parse_rules(input: &str) -> HashMap<usize, Rule> {
    let mut rules = HashMap::new();

    for line in input.lines() {
        if let [idx, rule, ..] = line.split(':').collect::<Vec<_>>().as_slice()
        {
            let idx = idx.parse::<usize>().unwrap();

            rules.insert(idx, parse_rule(rule));
        }
    }

    rules
}

fn parse(input: &str) -> (HashMap<usize, Rule>, Vec<String>) {
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
    rules: &HashMap<usize, Rule>,
) -> Option<usize> {
    match current {
        Rule::Alternative(alternatives) => {
            let mut max_pos = pos;
            for sequence in alternatives {
                let mut current_pos = pos;
                let mut failed = false;
                for idx in sequence {
                    if let Some(pos) =
                        valid(&message, current_pos, &rules[idx], rules)
                    {
                        current_pos = pos;
                    } else {
                        failed = true;
                        break;
                    }
                }

                if !failed {
                    max_pos = cmp::max(current_pos, max_pos);
                }
            }

            if max_pos > pos {
                Some(max_pos)
            } else {
                None
            }
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

fn valid_message(message: &str, rules: &HashMap<usize, Rule>) -> bool {
    if !rules.contains_key(&8) || !rules.contains_key(&11) {
        if let Some(pos) = valid(message, 0, &rules[&0], rules) {
            return pos == message.len();
        } else {
            return false;
        }
    }

    for i in 0..message.len() {
        let (left, right) = message.split_at(i);

        match (
            valid(left, 0, &rules[&8], rules),
            valid(right, 0, &rules[&11], rules),
        ) {
            (Some(lpos), Some(rpos))
                if lpos == left.len() && rpos == right.len() =>
            {
                return true;
            }
            _ => (),
        }
    }

    false
}

fn solve(messages: &[String], rules: &HashMap<usize, Rule>) -> usize {
    messages
        .iter()
        .filter(|msg| valid_message(msg, rules))
        .count()
}

fn main() {
    let input1 = &fs::read_to_string(input_file("day19.txt")).unwrap();
    let (rules, messages) = parse(input1);
    println!("part1 = {}", solve(&messages, &rules));

    let input2 = input1
        .replace("8: 42", "8: 42 | 42 8")
        .replace("11: 42 31", "11: 42 31 | 42 11 31");
    let (rules, messages) = parse(&input2);
    println!("part2 = {}", solve(&messages, &rules));
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

    assert_eq!(solve(&messages, &rules), 2);

    let example2 = "\
0: 8 11
8: 42 | 42 8
11: 42 31 | 42 11 31
42: \"a\"
31: \"b\"

aaabb
aab
aaaab
aaaaaaaaaaabb
abb
";

    let (rules, messages) = parse(example2);

    assert_eq!(solve(&messages, &rules), 4);

    let example3 = "\
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31 | 42 11 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42 | 42 8
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
";

    let (rules, messages) = parse(example3);

    assert_eq!(solve(&messages, &rules), 12);
}
