use aoc_2020::input_file;
use std::fs;

type Field = Vec<Vec<char>>;

fn collisions<F>(field: &Field, predicate: F) -> usize
where
    F: Fn(&(usize, &Vec<char>)) -> bool,
{
    field.iter().enumerate().filter(predicate).count()
}

fn main() {
    let field = fs::read_to_string(input_file("day3.txt"))
        .expect("file not found!")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let s1 = collisions(&field, |(y, line)| line[y * 3 % line.len()] == '#');
    let s2 = collisions(&field, |(y, line)| line[y % line.len()] == '#');
    let s3 = collisions(&field, |(y, line)| line[y * 5 % line.len()] == '#');
    let s4 = collisions(&field, |(y, line)| line[y * 7 % line.len()] == '#');
    let s5 = collisions(&field, |(y, line)| {
        y % 2 == 0 && line[y / 2 % line.len()] == '#'
    });

    println!("part1 = {}", s1);
    println!("part2 = {}", s1 * s2 * s3 * s4 * s5);
}
