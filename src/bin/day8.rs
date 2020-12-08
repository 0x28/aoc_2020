use aoc_2020::input_file;
use std::fs;

#[derive(Clone)]
enum Instruction {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            if let [operator, operand, ..] =
                line.split_whitespace().collect::<Vec<&str>>().as_slice()
            {
                let operand =
                    operand.parse().expect("syntax error: not a number");
                match *operator {
                    "nop" => Instruction::Nop(operand),
                    "acc" => Instruction::Acc(operand),
                    "jmp" => Instruction::Jmp(operand),
                    v => panic!("unknown operator: \"{}\"", v),
                }
            } else {
                panic!("missing operands")
            }
        })
        .collect()
}

fn part1(prog: &Vec<Instruction>) -> (bool, i64) {
    let mut pos_lookup = vec![false; prog.len()];
    let mut acc = 0i64;
    let mut pc = 0i64;

    while (pc as usize) < prog.len() {
        let idx = pc as usize;

        if pos_lookup[idx] {
            return (false, acc);
        } else {
            pos_lookup[idx] = true;
        }

        match prog[idx] {
            Instruction::Nop(_) => (),
            Instruction::Acc(v) => acc += v,
            Instruction::Jmp(v) => pc += v - 1,
        }

        pc += 1;
    }

    (true, acc)
}

fn part2(prog: &mut Vec<Instruction>) -> i64 {
    for i in 0..prog.len() {
        let old = prog[i].clone();

        prog[i] = match old {
            Instruction::Jmp(v) => Instruction::Nop(v),
            Instruction::Nop(v) => Instruction::Jmp(v),
            _ => continue,
        };

        if let (true, value) = part1(prog) {
            return value;
        }

        prog[i] = old;
    }

    panic!("no solution found!")
}

fn main() {
    let input =
        &fs::read_to_string(input_file("day8.txt")).expect("file not found!");
    let mut input = parse(input);

    println!("part1 = {}", part1(&input).1);
    println!("part2 = {}", part2(&mut input));
}

#[test]
fn test_day8() {
    let example1 = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";

    assert_eq!(part1(&parse(example1)), (false, 5));
}
