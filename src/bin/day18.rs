use aoc_2020::input_file;
use std::{fs, iter::Peekable, str::Chars};

fn eval_expr(stream: &mut Peekable<Chars>) -> u64 {
    let mut result = 0;
    let mut is_add = false;
    let mut is_mul = false;

    while let Some(c) = stream.peek() {
        match c {
            '0'..='9' => {
                let num = c.to_digit(10).unwrap() as u64;
                if is_add {
                    result += num;
                } else if is_mul {
                    result *= num;
                } else {
                    result = num;
                }
                stream.next();
            }
            '+' => {
                is_add = true;
                is_mul = false;
                stream.next();
            }
            '*' => {
                is_mul = true;
                is_add = false;
                stream.next();
            }
            '(' => {
                stream.next();
                let num = eval_expr(stream);

                if is_add {
                    result += num;
                } else if is_mul {
                    result *= num;
                } else {
                    result = num;
                }
            }
            ')' => {
                stream.next();
                return result;
            }
            _ => {
                stream.next();
            }
        }
    }

    result
}

fn eval(input: &str) -> u64 {
    eval_expr(&mut input.chars().peekable())
}

fn part1(input: &str) -> u64 {
    input.lines().map(eval).sum()
}

fn eval_add(stream: &mut Peekable<Chars>) -> u64 {
    let mut result = 0;

    while let Some(c) = stream.peek() {
        match c {
            '0'..='9' => {
                result += c.to_digit(10).unwrap() as u64;
                stream.next();
            }
            '(' => {
                stream.next();
                result += eval_mul(stream);
            }
            ')' | '*' => {
                break;
            }
            _ => {
                stream.next();
            }
        }
    }

    result
}

fn eval_mul(stream: &mut Peekable<Chars>) -> u64 {
    let mut result = eval_add(stream);

    while let Some(c) = stream.peek() {
        match c {
            '*' => {
                stream.next();
            }
            ')' => {
                stream.next();
                break;
            }
            _ => {
                result *= eval_add(stream);
            }
        }
    }

    result
}

fn eval_prec(input: &str) -> u64 {
    eval_mul(&mut input.chars().peekable())
}

fn part2(input: &str) -> u64 {
    input.lines().map(eval_prec).sum()
}

fn main() {
    let input = fs::read_to_string(input_file("day18.txt")).unwrap();
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day18() {
    assert_eq!(eval("2 * 3 + (4 * 5)"), 26);
    assert_eq!(eval("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
    assert_eq!(eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
    assert_eq!(
        eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        13632
    );

    assert_eq!(eval_prec("1 + (2 * 3) + (4 * (5 + 6))"), 51);
    assert_eq!(eval_prec("2 * 3 + (4 * 5)"), 46);
    assert_eq!(eval_prec("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
    assert_eq!(
        eval_prec("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
        669060
    );
    assert_eq!(
        eval_prec("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        23340
    );
}
