use aoc_2020::input_file;
use std::fs;

type Notes = (i64, Vec<(usize, i64)>);

fn parse(input: &str) -> Notes {
    let mut lines = input.lines();
    let start = lines.next().unwrap().parse().unwrap();
    let ids = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .flat_map(|(n, v)| Some((n, v.parse::<i64>().ok()?)))
        .collect();

    (start, ids)
}

fn part1(notes: &Notes) -> i64 {
    let (start, ids) = notes;

    let (id, time) = ids
        .iter()
        .map(|t| t.1)
        .map(|id| (id, start + (id - start % id) % id))
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap();

    id * (time - start)
}

fn euclid_algorithm(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }

    let (d, s, t) = euclid_algorithm(b, a.rem_euclid(b));
    (d, t, s - (a.div_euclid(b)) * t)
}

fn part2(notes: &Notes) -> i64 {
    let m: i64 = notes.1.iter().map(|t| t.1).product();

    // chinese remainder theorem (∩ ^o^)⊃━☆゜.*
    m - notes
        .1
        .iter()
        .map(|(offset, id)| {
            let (_, _, s) = euclid_algorithm(*id, m / id);
            let e = s * (m / id);

            e * (*offset as i64)
        })
        .sum::<i64>()
        .rem_euclid(m)
}

fn main() {
    let notes = parse(&fs::read_to_string(input_file("day13.txt")).unwrap());
    println!("part1 = {}", part1(&notes));
    println!("part2 = {}", part2(&notes));
}

#[test]
fn test_day13() {
    let example1 = parse(
        "\
939
7,13,x,x,59,x,31,19",
    );

    assert_eq!(part1(&example1), 295);

    // assert_eq!(euclid_algorithm(3, 20), (0,0,0));

    let example2 = parse(
        "\
0
1789,37,47,1889
",
    );
    assert_eq!(part2(&example2), 1202161486);

    let example3 = parse(
        "\
0
17,x,13,19
",
    );
    assert_eq!(part2(&example3), 3417);

    let example4 = parse(
        "\
0
67,7,x,59,61
",
    );
    assert_eq!(part2(&example4), 1261476);
}
