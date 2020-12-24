use aoc_2020::input_file;
use std::{collections::{HashMap, HashSet}, fs, hash::Hash};

#[derive(Debug, PartialEq)]
enum Dir {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

fn parse_line(input: &str) -> Vec<Dir> {
    let mut stream = input.chars().peekable();
    let mut dirs = vec![];

    while let Some(c) = stream.peek() {
        match c {
            's' => {
                stream.next();
                match stream.peek() {
                    Some('w') => dirs.push(Dir::SouthWest),
                    Some('e') => dirs.push(Dir::SouthEast),
                    _ => unreachable!(),
                }
            }
            'n' => {
                stream.next();
                match stream.peek() {
                    Some('w') => dirs.push(Dir::NorthWest),
                    Some('e') => dirs.push(Dir::NorthEast),
                    _ => unreachable!(),
                }
            }
            'e' => {
                dirs.push(Dir::East);
            }
            'w' => {
                dirs.push(Dir::West);
            }
            _ => unreachable!(),
        }
        stream.next();
    }

    dirs
}

fn dir_to_pos(dir: &Dir) -> (i64, i64) {
    match dir {
        Dir::East => (1, 0),
        Dir::SouthEast => (0, -1),
        Dir::SouthWest => (-1, -1),
        Dir::West => (-1, 0),
        Dir::NorthWest => (0, 1),
        Dir::NorthEast => (1, 1),
    }
}

fn parse(input: &str) -> Vec<Vec<Dir>> {
    input.lines().map(parse_line).collect()
}

fn find_black_tiles(dirs: &Vec<Vec<Dir>>) -> HashSet<(i64, i64)> {
    let mut flipped = HashMap::<(i64, i64), bool>::new();
    let ref_tile = (0, 0);

    for dir in dirs {
        let pos = dir.iter().fold(ref_tile, |(x, y), d| {
            let (rx, ry) = dir_to_pos(d);

            (x + rx, y + ry)
        });

        match flipped.get(&pos).copied() {
            Some(status) => {
                flipped.insert(pos, !status);
            }
            None => {
                flipped.insert(pos, true);
            }
        }
    }

    flipped
        .iter()
        .filter_map(|(p, status)| if *status { Some(*p) } else { None })
        .collect()
}

fn part1(dirs: &Vec<Vec<Dir>>) -> usize {
    find_black_tiles(&dirs).len()
}

fn conway<P, N>(field: &HashSet<P>, neighbors_fun: N) -> usize
where
    N: Fn(&P) -> HashSet<P> + Copy,
    P: Clone + Copy + Eq + Hash,
{
    let mut current = field.clone();

    for _ in 0..100 {
        let mut next = HashSet::new();
        for black_tile in current.iter() {
            let neighbors = neighbors_fun(&black_tile);
            if let 1 | 2 = neighbors.intersection(&current).count() {
                next.insert(black_tile);
            }
        }
        let extended_cubes = current
            .iter()
            .flat_map(neighbors_fun)
            .collect::<HashSet<_>>();
        for inactive_cube in extended_cubes.difference(&current) {
            let neighbors = neighbors_fun(inactive_cube);
            if let 2 = neighbors.intersection(&current).count() {
                next.insert(inactive_cube);
            }
        }

        current = next.into_iter().copied().collect();
    }

    current.len()
}

fn neighbors((pos_x, pos_y): &(i64, i64)) -> HashSet<(i64, i64)> {
    let dirs = vec![
        Dir::East,
        Dir::SouthEast,
        Dir::SouthWest,
        Dir::West,
        Dir::NorthWest,
        Dir::NorthEast,
    ];

    dirs.iter()
        .map(|d| {
            let (rx, ry) = dir_to_pos(d);
            (pos_x + rx, pos_y + ry)
        })
        .collect()
}

fn part2(dirs: &Vec<Vec<Dir>>) -> usize {
    let black_tiles = find_black_tiles(&dirs);

    conway(&black_tiles, neighbors)
}

fn main() {
    let dirs = parse(&fs::read_to_string(input_file("day24.txt")).unwrap());
    println!("part1 = {}", part1(&dirs));
    println!("part2 = {}", part2(&dirs));
}

#[test]
fn test_day24() {
    let example1 = "\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew
";

    assert_eq!(part1(parse(example1)), 10);
}
