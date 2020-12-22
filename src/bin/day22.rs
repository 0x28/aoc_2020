use aoc_2020::input_file;
use std::collections::VecDeque;
use std::fs;

fn parse_player(player: &str) -> Vec<u32> {
    player
        .lines()
        .filter(|l| l.chars().next().unwrap().is_numeric())
        .flat_map(str::parse)
        .collect()
}

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let players = input.split("\n\n").collect::<Vec<_>>();

    (parse_player(players[0]), parse_player(players[1]))
}

fn part1(player1: &[u32], player2: &[u32]) -> u32 {
    let mut deck1 = player1.iter().collect::<VecDeque<_>>();
    let mut deck2 = player2.iter().collect::<VecDeque<_>>();

    while !deck1.is_empty() && !deck2.is_empty() {
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();

        if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }

    if deck1.is_empty() {
        deck2
            .into_iter()
            .rev()
            .enumerate()
            .map(|(n, &card)| (n as u32 + 1) * card)
            .sum()
    } else {
        deck1
            .into_iter()
            .rev()
            .enumerate()
            .map(|(n, &card)| (n as u32 + 1) * card)
            .sum()
    }
}

fn main() {
    let input = fs::read_to_string(input_file("day22.txt")).unwrap();
    let player = parse(&input);
    println!("part1 = {}", part1(&player.0, &player.1));
}

#[test]
fn test_day22() {
    let example1 = "\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
    let example1 = parse(example1);

    assert_eq!(part1(&example1.0, &example1.1), 306);
}
