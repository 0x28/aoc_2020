use aoc_2020::input_file;
use std::collections::{HashSet, VecDeque};
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

fn score_deck(deck: &[u32]) -> u32 {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(n, &card)| (n as u32 + 1) * card)
        .sum()
}

fn part1(player1: &[u32], player2: &[u32]) -> u32 {
    let mut deck1 = player1.iter().copied().collect::<VecDeque<_>>();
    let mut deck2 = player2.iter().copied().collect::<VecDeque<_>>();

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
        score_deck(deck2.make_contiguous())
    } else {
        score_deck(deck1.make_contiguous())
    }
}

fn recursive_combat(player1: &[u32], player2: &[u32]) -> (u8, VecDeque<u32>) {
    let mut deck1 = player1.iter().copied().collect::<VecDeque<_>>();
    let mut deck2 = player2.iter().copied().collect::<VecDeque<_>>();
    let mut dejavu1 = HashSet::<VecDeque<u32>>::new();
    let mut dejavu2 = HashSet::<VecDeque<u32>>::new();

    while !deck1.is_empty() && !deck2.is_empty() {
        if dejavu1.contains(&deck1) || dejavu2.contains(&deck2) {
            return (1, deck1);
        } else {
            dejavu1.insert(deck1.clone());
            dejavu2.insert(deck2.clone());
        }

        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();

        if card1 as usize <= deck1.len() && card2 as usize <= deck2.len() {
            let rdeck1 = deck1
                .iter()
                .take(card1 as usize)
                .cloned()
                .collect::<Vec<u32>>();
            let rdeck2 = deck2
                .iter()
                .take(card2 as usize)
                .cloned()
                .collect::<Vec<u32>>();

            match recursive_combat(&rdeck1, &rdeck2) {
                (1, _) => {
                    deck1.push_back(card1);
                    deck1.push_back(card2);
                }
                (2, _) => {
                    deck2.push_back(card2);
                    deck2.push_back(card1);
                }
                _ => unreachable!(),
            }
        } else if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }

    if deck2.is_empty() {
        (1, deck1)
    } else {
        (2, deck2)
    }
}

fn part2(player1: &[u32], player2: &[u32]) -> u32 {
    let (_, mut deck) = recursive_combat(player1, player2);

    score_deck(deck.make_contiguous())
}

fn main() {
    let input = fs::read_to_string(input_file("day22.txt")).unwrap();
    let player = parse(&input);
    println!("part1 = {}", part1(&player.0, &player.1));
    println!("part2 = {}", part2(&player.0, &player.1));
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
    assert_eq!(part2(&example1.0, &example1.1), 291);
}
