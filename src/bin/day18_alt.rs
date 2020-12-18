use aoc_2020::input_file;
use std::fs;

peg::parser!( grammar arithmetic() for str {
    rule number() -> u64
        = _ n:$(['0'..='9']+) _ { n.parse().unwrap() }

    pub rule calculate() -> u64 = precedence!{
        x:(@) "*" y:@ { x * y }
        x:(@) "+" y:@ { x + y }
        --
        _ "(" v:calculate() ")" _ { v }
        n:number() {n}
    }

    pub rule calculate_prec() -> u64 = precedence!{
        x:(@) "*" y:@ { x * y }
        --
        x:(@) "+" y:@ { x + y }
        --
        _ "(" v:calculate_prec() ")" _ { v }
        n:number() {n}
    }

    rule _() = quiet!{[c if c.is_whitespace()]*}
});

fn part1(input: &str) -> u64 {
    input.lines().flat_map(arithmetic::calculate).sum()
}

fn part2(input: &str) -> u64 {
    input.lines().flat_map(arithmetic::calculate_prec).sum()
}

fn main() {
    let input = &fs::read_to_string(input_file("day18.txt")).unwrap();
    println!("part1 = {}", part1(input));
    println!("part2 = {}", part2(input));
}
