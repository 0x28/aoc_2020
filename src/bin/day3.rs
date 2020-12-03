use aoc_2020::input_file;
use std::fs;

fn main() {
    let (s1, s2, s3, s4, s5) = fs::read_to_string(input_file("day3.txt"))
        .expect("file not found!")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .enumerate()
        .fold((0, 0, 0, 0, 0), |(s1, s2, s3, s4, s5), (y, line)| {
            (
                (line[y * 3 % line.len()] == '#') as u32 + s1,
                (line[y % line.len()] == '#') as u32 + s2,
                (line[y * 5 % line.len()] == '#') as u32 + s3,
                (line[y * 7 % line.len()] == '#') as u32 + s4,
                (y % 2 == 0 && line[y / 2 % line.len()] == '#') as u32 + s5,
            )
        });

    println!("part 1 = {}", s1);
    println!("part 2 = {}", s1 * s2 * s3 * s4 * s5);
}
