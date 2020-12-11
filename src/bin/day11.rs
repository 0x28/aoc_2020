use aoc_2020::input_file;
use std::usize;
use std::{fs, mem};

type Field = [Vec<char>];

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn neighbors(field: &Field, (pos_x, pos_y): (usize, usize)) -> usize {
    (pos_x.checked_sub(1).unwrap_or(pos_x)..=pos_x + 1)
        .flat_map(|x| {
            (pos_y.checked_sub(1).unwrap_or(pos_y)..=pos_y + 1).filter_map(
                move |y| {
                    if (pos_x, pos_y) != (x, y) {
                        Some(*field.get(y as usize)?.get(x as usize)?)
                    } else {
                        None
                    }
                },
            )
        })
        .filter(|&c| c == '#')
        .count()
}

fn part1(field: &Field) -> usize {
    let mut current = field.to_owned();
    let mut next = current.clone();

    loop {
        for y in 0..current.len() {
            for x in 0..current[y].len() {
                let n = neighbors(&current, (x, y));

                match current[y][x] {
                    'L' if n == 0 => next[y][x] = '#',
                    '#' if n >= 4 => next[y][x] = 'L',
                    c => next[y][x] = c,
                }
            }
        }

        if current == next {
            break;
        } else {
            mem::swap(&mut current, &mut next);
        }
    }

    current.iter().flatten().filter(|&&c| c == '#').count()
}

fn dir(
    field: &Field,
    (pos_x, pos_y): (usize, usize),
    x_step: isize,
    y_step: isize,
) -> bool {
    let mut x = pos_x as isize + x_step;
    let mut y = pos_y as isize + y_step;

    while (field.len() as isize) > y
        && (field[0].len() as isize) > x
        && x >= 0
        && y >= 0
    {
        match field[y as usize][x as usize] {
            '#' => return true,
            'L' => return false,
            _ => (),
        }

        x += x_step;
        y += y_step;
    }

    false
}

fn neighbors2(field: &Field, pos: (usize, usize)) -> usize {
    let dirs = [
        dir(&field, pos, -1, -1),
        dir(&field, pos, -1, 0),
        dir(&field, pos, -1, 1),
        dir(&field, pos, 0, -1),
        dir(&field, pos, 0, 1),
        dir(&field, pos, 1, -1),
        dir(&field, pos, 1, 0),
        dir(&field, pos, 1, 1),
    ];

    dirs.iter().filter(|b| **b).count()
}

fn part2(field: &Field) -> usize {
    let mut current = field.to_owned();
    let mut next = current.clone();

    loop {
        for y in 0..current.len() {
            for x in 0..current[y].len() {
                let n = neighbors2(&current, (x, y));

                match current[y][x] {
                    'L' if n == 0 => next[y][x] = '#',
                    '#' if n >= 5 => next[y][x] = 'L',
                    c => next[y][x] = c,
                }
            }
        }

        if current == next {
            break;
        } else {
            mem::swap(&mut current, &mut next);
        }
    }

    current.iter().flatten().filter(|&&c| c == '#').count()
}

fn main() {
    let input = fs::read_to_string(input_file("day11.txt")).unwrap();
    let input = parse(&input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day11() {
    let example = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";

    let example = parse(example);

    assert_eq!(part1(&example), 37);
    assert_eq!(part2(&example), 26);
}
