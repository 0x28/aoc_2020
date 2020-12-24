use aoc_2020::input_file;
use std::{collections::HashMap, fs};

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

fn part1(dirs: Vec<Vec<Dir>>) -> usize {
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

    flipped.values().filter(|status| **status).count()
}

fn main() {
    let dirs = parse(&fs::read_to_string(input_file("day24.txt")).unwrap());
    println!("part1 = {}", part1(dirs));
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
