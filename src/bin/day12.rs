use aoc_2020::input_file;
use std::fs;

#[derive(Debug)]
enum Action {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

fn parse(input: &str) -> Vec<Action> {
    input
        .lines()
        .map(|line| {
            let begin = line.chars().next()?;
            match begin {
                'N' => Some(Action::North(line[1..].parse().ok()?)),
                'S' => Some(Action::South(line[1..].parse().ok()?)),
                'E' => Some(Action::East(line[1..].parse().ok()?)),
                'W' => Some(Action::West(line[1..].parse().ok()?)),
                'L' => Some(Action::Left(line[1..].parse().ok()?)),
                'R' => Some(Action::Right(line[1..].parse().ok()?)),
                'F' => Some(Action::Forward(line[1..].parse().ok()?)),
                _ => None,
            }
        })
        .flatten()
        .collect()
}

fn part1(actions: &[Action]) -> i32 {
    let mut pos = (0, 0);
    let mut degree = 90; // east

    for action in actions {
        match action {
            Action::North(v) => pos.1 += v,
            Action::South(v) => pos.1 -= v,
            Action::East(v) => pos.0 += v,
            Action::West(v) => pos.0 -= v,
            Action::Left(d) => degree = (degree - *d + 360) % 360,
            Action::Right(d) => degree = (degree + *d) % 360,
            Action::Forward(v) if degree == 0 => pos.1 += v,
            Action::Forward(v) if degree == 90 => pos.0 += v,
            Action::Forward(v) if degree == 180 => pos.1 -= v,
            Action::Forward(v) if degree == 270 => pos.0 -= v,
            v => panic!("Error {:?} {}", v, degree),
        }
    }

    i32::abs(pos.0) + i32::abs(pos.1)
}

fn rotate(point: (i32, i32), mut degree: i32) -> (i32, i32) {
    let mut res = point;
    while degree > 0 {
        res = (res.1, -res.0);
        degree -= 90;
    }

    res
}

fn part2(actions: &[Action]) -> i32 {
    let mut pos = (0, 0);
    let mut waypoint = (10, 1);

    for action in actions {
        match action {
            Action::North(v) => waypoint.1 += v,
            Action::South(v) => waypoint.1 -= v,
            Action::East(v) => waypoint.0 += v,
            Action::West(v) => waypoint.0 -= v,
            Action::Left(v) => waypoint = rotate(waypoint, 360 - *v),
            Action::Right(v) => waypoint = rotate(waypoint, *v),
            Action::Forward(v) => {
                pos.0 += waypoint.0 * v;
                pos.1 += waypoint.1 * v;
            }
        }
    }

    i32::abs(pos.0) + i32::abs(pos.1)
}

fn main() {
    let input = parse(&fs::read_to_string(input_file("day12.txt")).unwrap());
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day12() {
    let example = parse(
        "\
F10
N3
F7
R90
F11",
    );

    assert_eq!(part1(&example), 25);
    assert_eq!(part2(&example), 286);
}
