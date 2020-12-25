use aoc_2020::input_file;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Tile {
    id: u64,
    top: u64,
    bottom: u64,
    left: u64,
    right: u64,
    pixel: Vec<Vec<char>>,
}

fn border_to_number(border: &str) -> u64 {
    u64::from_str_radix(&border.replace('#', "1").replace('.', "0"), 2).unwrap()
}

fn parse_tile(block: &str) -> Tile {
    let lines = block.lines().collect::<Vec<_>>();

    if let [first, rest @ ..] = lines.as_slice() {
        let id = first
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse()
            .unwrap();
        let top = border_to_number(rest[0]);
        let bottom = border_to_number(rest.last().unwrap());
        let right = border_to_number(
            &rest
                .iter()
                .flat_map(|line| line.chars().last())
                .collect::<String>(),
        );
        let left = border_to_number(
            &rest
                .iter()
                .flat_map(|line| line.chars().next())
                .collect::<String>(),
        );

        Tile {
            id,
            top,
            bottom,
            left,
            right,
            pixel: rest.iter().map(|l| l.chars().collect()).collect(),
        }
    } else {
        unreachable!()
    }
}

fn parse(input: &str) -> Vec<Tile> {
    input.split("\n\n").map(parse_tile).collect()
}

fn rotate_pixel(block: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut rblock = vec![vec![' '; block.len()]; block[0].len()];

    for y in 0..rblock.len() {
        for x in 0..rblock[y].len() {
            rblock[y][x] = block[rblock[y].len() - x - 1][y];
        }
    }

    rblock
}

fn flip_pixel(block: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut rblock = block.to_vec();
    for y in 0..rblock.len() {
        for x in 0..rblock[y].len() {
            rblock[y][x] = block[y][rblock[y].len() - x - 1];
        }
    }

    rblock
}

fn rotate(tile: &Tile) -> Tile {
    let pixel = rotate_pixel(&tile.pixel);

    Tile {
        id: tile.id,
        top: reverse_border(tile.left),
        right: tile.top,
        bottom: reverse_border(tile.right),
        left: tile.bottom,
        pixel,
    }
}

fn flip(tile: &Tile) -> Tile {
    let pixel = flip_pixel(&tile.pixel);

    Tile {
        id: tile.id,
        top: reverse_border(tile.top),
        bottom: reverse_border(tile.bottom),
        left: tile.right,
        right: tile.left,
        pixel,
    }
}

fn reverse_border(border: u64) -> u64 {
    let mut result = 0;
    for b in 0..10 {
        result <<= 1;
        result |= (border >> b) & 1;
    }

    result
}

fn orientations(tile: &Tile) -> Vec<Tile> {
    let tile_r1 = rotate(tile);
    let tile_r2 = rotate(&tile_r1);
    let tile_r3 = rotate(&tile_r2);

    vec![
        flip(tile),
        flip(&tile_r1),
        flip(&tile_r2),
        flip(&tile_r3),
        tile_r1,
        tile_r2,
        tile_r3,
        tile.clone(),
    ]
}

fn solve_jigsaw(tiles: &[Tile]) -> HashMap<(i32, i32), Tile> {
    let mut unused = tiles.iter().skip(1).cloned().collect::<HashSet<_>>();
    let mut unexpanded = HashSet::new();
    unexpanded.insert(((0, 0), tiles.iter().next().unwrap().clone()));
    let mut expanded = HashMap::new();

    while let Some((pos, current_tile)) = unexpanded.iter().cloned().next() {
        for adjacent_tile in unused.clone().iter() {
            for orientation in orientations(&adjacent_tile) {
                if orientation.left == current_tile.right {
                    unused.remove(adjacent_tile);
                    unexpanded.insert(((pos.0 + 1, pos.1), orientation));
                    break;
                } else if orientation.right == current_tile.left {
                    unused.remove(adjacent_tile);
                    unexpanded.insert(((pos.0 - 1, pos.1), orientation));
                    break;
                } else if orientation.bottom == current_tile.top {
                    unexpanded.insert(((pos.0, pos.1 - 1), orientation));
                    unused.remove(adjacent_tile);
                    break;
                } else if orientation.top == current_tile.bottom {
                    unexpanded.insert(((pos.0, pos.1 + 1), orientation));
                    unused.remove(adjacent_tile);
                    break;
                }
            }
        }

        unexpanded.remove(&(pos, current_tile.clone()));
        expanded.insert(pos, current_tile);
    }

    expanded
}

fn part1(puzzle: &HashMap<(i32, i32), Tile>) -> u64 {
    let max_x = puzzle.iter().max_by_key(|(pos, _)| pos.0).unwrap().0 .0;
    let max_y = puzzle.iter().max_by_key(|(pos, _)| pos.1).unwrap().0 .1;
    let min_x = puzzle.iter().min_by_key(|(pos, _)| pos.0).unwrap().0 .0;
    let min_y = puzzle.iter().min_by_key(|(pos, _)| pos.1).unwrap().0 .1;

    puzzle[&(max_x, max_y)].id
        * puzzle[&(max_x, min_y)].id
        * puzzle[&(min_x, max_y)].id
        * puzzle[&(min_x, min_y)].id
}

fn combine(puzzle: &HashMap<(i32, i32), Tile>) -> Vec<Vec<char>> {
    let max_x = puzzle.iter().max_by_key(|(pos, _)| pos.0).unwrap().0 .0;
    let max_y = puzzle.iter().max_by_key(|(pos, _)| pos.1).unwrap().0 .1;
    let min_x = puzzle.iter().min_by_key(|(pos, _)| pos.0).unwrap().0 .0;
    let min_y = puzzle.iter().min_by_key(|(pos, _)| pos.1).unwrap().0 .1;
    let size: i32 = 8;

    let mut picture = vec![
        vec![' '; (size * (max_x - min_x + 1)) as usize];
        (size * (max_y - min_y + 1)) as usize
    ];

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let tile = &puzzle[&(x, y)];

            for t_y in 1..tile.pixel.len() - 1 {
                for t_x in 1..tile.pixel[t_y].len() - 1 {
                    picture[(size * (y - min_y) + (t_y - 1) as i32) as usize]
                        [(size * (x - min_x) + (t_x - 1) as i32) as usize] =
                        tile.pixel[t_y][t_x];
                }
            }
        }
    }

    picture
}

fn main() {
    let input = fs::read_to_string(input_file("day20.txt")).unwrap();
    let tiles = parse(&input);
    let tiles = solve_jigsaw(&tiles);
    println!("part1 = {}", part1(&tiles));
}

#[test]
fn test_day20() {
    let tile1 = "\
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###
";
    let tile = parse_tile(tile1);
    assert_eq!(
        tile,
        Tile {
            id: 2311,
            left: 0b0111110010,
            right: 0b0001011001,
            bottom: 0b0011100111,
            top: 0b0011010010,
            pixel: vec![
                "..##.#..#.".chars().collect(),
                "##..#.....".chars().collect(),
                "#...##..#.".chars().collect(),
                "####.#...#".chars().collect(),
                "##.##.###.".chars().collect(),
                "##...#.###".chars().collect(),
                ".#.#.#..##".chars().collect(),
                "..#....#..".chars().collect(),
                "###...#.#.".chars().collect(),
                "..###..###".chars().collect(),
            ]
        }
    );

    assert_eq!(
        rotate(&tile),
        Tile {
            id: 2311,
            top: reverse_border(tile.left),
            right: tile.top,
            bottom: reverse_border(tile.right),
            left: tile.bottom,
            pixel: vec![
                ".#..#####.".chars().collect(),
                ".#.####.#.".chars().collect(),
                "###...#..#".chars().collect(),
                "#..#.##..#".chars().collect(),
                "#....#.##.".chars().collect(),
                "...##.##.#".chars().collect(),
                ".#...#....".chars().collect(),
                "#.#.##....".chars().collect(),
                "##.###.#.#".chars().collect(),
                "#..##.#...".chars().collect(),
            ]
        }
    );

    let n: u64 = 0b1000100011;

    assert_eq!(reverse_border(n), 0b1100010001);

    let example1 = "\
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
";

    let tiles1 = parse(example1);
    let tiles1 = solve_jigsaw(&tiles1);
    assert_eq!(part1(&tiles1), 20899048083289);
}
