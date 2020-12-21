use aoc_2020::input_file;
use std::{collections::HashSet, fs};

fn parse(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let mut tuples = vec![];

    for line in input.lines() {
        let mut ingredients = Vec::new();
        let mut allergens = Vec::new();
        let mut word = String::new();
        let mut in_parens = false;
        for c in line.chars() {
            match c {
                'a'..='z' => word.push(c),
                ' ' | ')' => {
                    if word != "contains" {
                        if in_parens {
                            allergens.push(word.clone());
                        } else {
                            ingredients.push(word.clone());
                        }
                    }
                    word.clear();
                }
                '(' => {
                    in_parens = true;
                }
                _ => (),
            }
        }

        tuples.push((ingredients, allergens));
    }

    tuples
}

fn part1(tuples: &[(Vec<String>, Vec<String>)]) -> u32 {
    let mut allergens = HashSet::new();
    let mut ingredients = HashSet::new();
    let mut potentially_allergic = HashSet::new();

    for (i, a) in tuples {
        for allergen in a {
            allergens.insert(allergen);
        }
        for ingredient in i {
            ingredients.insert(ingredient);
        }
    }

    for allergen in allergens {
        let mut matches = tuples.iter().filter(|(_, a)| a.contains(allergen));
        let mut set = matches
            .next()
            .unwrap()
            .0
            .iter()
            .collect::<HashSet<_>>();

        for m in matches {
            set = set.intersection(&m.0.iter().collect()).cloned().collect();
        }

        potentially_allergic =
            potentially_allergic.union(&set).cloned().collect();
    }

    let safe_ingredients = ingredients
        .difference(&potentially_allergic)
        .collect::<HashSet<_>>();

    let mut num = 0;
    for (i, _) in tuples {
        for ingredient in i {
            if safe_ingredients.contains(&ingredient) {
                num += 1;
            }
        }
    }

    num
}

fn main() {
    let input = parse(&fs::read_to_string(input_file("day21.txt")).unwrap());
    println!("part1 = {}", part1(&input));
}

#[test]
fn test_day21() {
    let example1 = "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
";
    let tuples = parse(example1);

    assert_eq!(part1(&tuples), 5);
}
