use std::fs;
use std::{collections::HashSet, hash::Hash};

use aoc_2020::input_file;

fn parse(input: &str) -> HashSet<(isize, isize, isize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| match c {
                '#' => Some((x as isize, y as isize, 0)),
                _ => None,
            })
        })
        .flatten()
        .collect()
}

fn neighbors3(
    (pos_x, pos_y, pos_z): &(isize, isize, isize),
) -> HashSet<(isize, isize, isize)> {
    let mut neighbors = HashSet::new();
    for x in pos_x - 1..=pos_x + 1 {
        for y in pos_y - 1..=pos_y + 1 {
            for z in pos_z - 1..=pos_z + 1 {
                if (pos_x, pos_y, pos_z) != (&x, &y, &z) {
                    neighbors.insert((x, y, z));
                }
            }
        }
    }

    neighbors
}

fn neighbors4(
    (pos_x, pos_y, pos_z, pos_w): &(isize, isize, isize, isize),
) -> HashSet<(isize, isize, isize, isize)> {
    let mut neighbors = HashSet::new();
    for x in pos_x - 1..=pos_x + 1 {
        for y in pos_y - 1..=pos_y + 1 {
            for z in pos_z - 1..=pos_z + 1 {
                for w in pos_w - 1..=pos_w + 1 {
                    if (pos_x, pos_y, pos_z, pos_w) != (&x, &y, &z, &w) {
                        neighbors.insert((x, y, z, w));
                    }
                }
            }
        }
    }

    neighbors
}

fn conway<P, N>(field: &HashSet<P>, neighbors_fun: N) -> usize
where
    N: Fn(&P) -> HashSet<P> + Copy,
    P: Clone + Copy + Eq + Hash,
{
    let mut current = field.clone();

    for _ in 0..6 {
        let mut next = HashSet::new();
        for active_cube in current.iter() {
            let neighbors = neighbors_fun(&active_cube);
            if let 2 | 3 = neighbors.intersection(&current).count() {
                next.insert(active_cube);
            }
        }
        let extended_cubes = current
            .iter()
            .flat_map(neighbors_fun)
            .collect::<HashSet<_>>();
        for inactive_cube in extended_cubes.difference(&current) {
            let neighbors = neighbors_fun(inactive_cube);
            if let 3 = neighbors.intersection(&current).count() {
                next.insert(inactive_cube);
            }
        }

        current = next.into_iter().copied().collect();
    }

    current.len()
}

fn part1(field: &HashSet<(isize, isize, isize)>) -> usize {
    conway(&field, neighbors3)
}

fn part2(field: &HashSet<(isize, isize, isize, isize)>) -> usize {
    conway(&field, neighbors4)
}

fn main() {
    let field = parse(&fs::read_to_string(input_file("day17.txt")).unwrap());
    println!("part1 = {}", part1(&field));
    println!(
        "part2 = {}",
        part2(&field.iter().map(|&(x, y, z)| (x, y, z, 0)).collect(),)
    );
}

#[test]
fn test_day17() {
    let example1 = "\
.#.
..#
###
";
    assert_eq!(part1(&parse(example1)), 112);
}
