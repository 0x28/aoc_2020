use aoc_2020::input_file;
use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::RangeInclusive,
};

struct TicketDB {
    attributes: HashMap<String, Vec<RangeInclusive<u64>>>,
    my_ticket: Vec<u64>,
    tickets: Vec<Vec<u64>>,
}

fn parse_ticket(line: &str) -> Vec<u64> {
    line.split(',').flat_map(str::parse).collect()
}

fn parse_attribute(line: &str) -> (String, Vec<RangeInclusive<u64>>) {
    let mut split = line.split(':');
    let key = split.next().unwrap();
    let ranges = split
        .next()
        .unwrap()
        .chars()
        .filter(|c| c.is_numeric() || c.is_whitespace() || *c == '-')
        .collect::<String>()
        .split_whitespace()
        .flat_map(|r| {
            let r = r.split('-').collect::<Vec<_>>();

            Some(r[0].parse().ok()?..=r[1].parse().ok()?)
        })
        .collect();

    (key.to_string(), ranges)
}

fn parse(input: &str) -> TicketDB {
    let groups = input.split("\n\n").collect::<Vec<_>>();

    let attributes = groups[0].lines().map(parse_attribute).collect();
    let my_ticket = groups[1].lines().nth(1).map(parse_ticket).unwrap();
    let tickets = groups[2]
        .lines()
        .map(parse_ticket)
        .filter(|t| !t.is_empty())
        .collect();

    TicketDB {
        attributes,
        my_ticket,
        tickets,
    }
}

fn part1(ticket_db: &TicketDB) -> u64 {
    ticket_db
        .tickets
        .iter()
        .flat_map(|ticket| {
            ticket.iter().filter(|attribute| {
                !ticket_db.attributes.iter().any(|(_, ranges)| {
                    ranges.iter().any(|r| r.contains(*attribute))
                })
            })
        })
        .sum()
}

fn part2(ticket_db: &TicketDB) -> u64 {
    let filtered = ticket_db
        .tickets
        .iter()
        .filter(|ticket| {
            ticket.iter().all(|attribute| {
                ticket_db.attributes.iter().any(|(_, ranges)| {
                    ranges.iter().any(|range| range.contains(attribute))
                })
            })
        })
        .collect::<Vec<_>>();

    let mut possible_attributes: Vec<_> =
        vec![HashSet::<String>::new(); ticket_db.attributes.len()];

    for attribute in &mut possible_attributes {
        *attribute = ticket_db.attributes.keys().cloned().collect();
    }

    for ticket in filtered {
        for i in 0..possible_attributes.len() {
            let value = ticket[i];
            possible_attributes[i] = possible_attributes[i]
                .iter()
                .filter(|field| {
                    ticket_db
                        .attributes
                        .get::<String>(field)
                        .unwrap()
                        .iter()
                        .any(|range| range.contains(&value))
                })
                .cloned()
                .collect();
        }
    }

    let mut real_attributes = vec!["".to_owned(); possible_attributes.len()];
    for i in 1..=possible_attributes.len() {
        for (pos, f) in possible_attributes
            .iter()
            .enumerate()
            .filter(|(_, row_set)| row_set.len() == i)
        {
            for k in f {
                if !real_attributes.contains(k) {
                    real_attributes[pos] = k.to_string()
                }
            }
        }
    }

    real_attributes
        .iter()
        .enumerate()
        .filter_map(|(idx, attribute)| {
            if attribute.starts_with("departure") {
                Some(ticket_db.my_ticket[idx])
            } else {
                None
            }
        })
        .product()
}

fn main() {
    let input = parse(&fs::read_to_string(input_file("day16.txt")).unwrap());
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day16() {
    let example1 = "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";
    let example1 = parse(example1);
    assert_eq!(part1(&example1), 71);
}
