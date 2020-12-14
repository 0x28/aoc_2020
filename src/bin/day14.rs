use aoc_2020::input_file;

use std::fs;
use std::{collections::HashMap, iter::Peekable, str::Chars};

#[derive(PartialEq, Debug)]
enum Expr {
    Mask(String),
    Mem(usize, u64),
}

fn parse_mask(mut input: Peekable<Chars>) -> Expr {
    let mut mask = String::new();

    while let Some(c) = input.peek() {
        match c {
            '0' | '1' | 'X' => mask.push(*c),
            _ => (),
        }
        input.next();
    }

    Expr::Mask(mask)
}

fn parse_mem(mut input: Peekable<Chars>) -> Expr {
    let mut index = 0;
    let mut value = 0;
    let mut in_index = false;

    while let Some(c) = input.peek() {
        match c {
            '0'..='9' => {
                let mut number = String::new();
                while let Some(c) = input.peek() {
                    if c.is_numeric() {
                        number.push(*c)
                    } else {
                        break;
                    }
                    input.next();
                }
                let number = number.parse::<u64>().unwrap();
                if in_index {
                    index = number;
                } else {
                    value = number;
                }
            }
            '[' => {
                in_index = true;
                input.next();
            }
            ']' => {
                in_index = false;
                input.next();
            }
            _ => {
                input.next();
            }
        }
    }

    Expr::Mem(index as usize, value)
}

fn parse_line(input: &str) -> Expr {
    let mut input = input.chars().peekable();

    if let Some(c) = input.peek() {
        match c {
            'a'..='z' => {
                let mut identifier = String::new();
                while let Some(c) = input.peek() {
                    if c.is_alphabetic() {
                        identifier.push(*c)
                    } else {
                        break;
                    }
                    input.next();
                }

                match identifier.as_str() {
                    "mask" => return parse_mask(input),
                    "mem" => return parse_mem(input),
                    ident => panic!("unknown identifier {}", ident),
                }
            }
            _ => panic!("syntax error!"),
        }
    }

    panic!("premature EOF!")
}

fn parse(input: &str) -> Vec<Expr> {
    input.lines().map(|l| parse_line(l)).collect()
}

fn part1(program: &[Expr]) -> u64 {
    let mut memory = HashMap::<usize, u64>::new();
    let mut or_mask = 0;
    let mut and_mask = 0;

    for expr in program {
        match expr {
            Expr::Mem(addr, value) => {
                memory.insert(*addr, (*value & and_mask) | or_mask);
            }
            Expr::Mask(value) => {
                and_mask =
                    u64::from_str_radix(&value.replace('X', "1"), 2).unwrap();
                or_mask =
                    u64::from_str_radix(&value.replace('X', "0"), 2).unwrap();
            }
        }
    }

    let mut sum = 0;
    for (_, v) in memory {
        sum += v;
    }
    sum
}

fn part2(program: &[Expr]) -> u64 {
    let mut memory = HashMap::<usize, u64>::new();
    let mut mask = String::new();

    for expr in program {
        match expr {
            Expr::Mem(addr, value) => {
                let xs = mask.chars().filter(|&c| c == 'X').count();
                let n = 1 << xs;

                for i in 0..n {
                    let mut x = i;
                    let mut mask_i = mask
                        .chars()
                        .zip(format!("{:036b}", *addr).chars())
                        .map(|(a, b)| match (a, b) {
                            ('0', v) => v,
                            ('1', _) => '1',
                            ('X', _) => 'X',
                            _ => unreachable!(),
                        })
                        .collect::<String>();
                    while mask_i.contains('X') {
                        mask_i = mask_i.replacen(
                            'X',
                            if x & 1 == 0 { "0" } else { "1" },
                            1,
                        );
                        x >>= 1;
                    }

                    memory.insert(
                        usize::from_str_radix(&mask_i, 2).unwrap(),
                        *value,
                    );
                }
            }
            Expr::Mask(value) => {
                mask = value.to_string();
            }
        }
    }

    let mut sum = 0;
    for (_, v) in memory {
        sum += v;
    }
    sum
}

fn main() {
    let program = parse(&fs::read_to_string(input_file("day14.txt")).unwrap());
    println!("part1 = {}", part1(&program));
    println!("part2 = {}", part2(&program));
}

#[test]
fn test_day14() {
    let example1 = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
";
    let example1 = parse(example1);

    assert_eq!(part1(&example1), 165);

    let example2 = "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
";
    let example2 = parse(example2);

    assert_eq!(part2(&example2), 208);
}
