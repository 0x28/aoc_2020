use aoc_2020::input_file;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

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

fn part1(
    tuples: &[(Vec<String>, Vec<String>)],
) -> (u32, HashMap<String, HashSet<&String>>) {
    let mut allergens = HashSet::new();
    let mut ingredients = HashSet::new();
    let mut potentially_allergic = HashSet::new();
    let mut map = HashMap::<String, HashSet<_>>::new();

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
        let mut set = matches.next().unwrap().0.iter().collect::<HashSet<_>>();

        for m in matches {
            set = set.intersection(&m.0.iter().collect()).cloned().collect();
        }

        potentially_allergic =
            potentially_allergic.union(&set).cloned().collect();
        match map.get_mut(allergen) {
            Some(s) => {
                *s = s.union(&set).cloned().collect();
            }
            None => {
                map.insert(allergen.clone(), set.iter().cloned().collect());
            }
        }
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

    (num, map)
}

fn part2(dangerous: &HashMap<String, HashSet<&String>>) -> String {
    let mut real_evil = HashMap::new();
    let mut used = HashSet::new();
    while real_evil.len() != dangerous.len() {
        for (allergen, ing) in dangerous {
            let unused = ing.difference(&used).cloned().collect::<Vec<_>>();
            if unused.len() == 1 {
                let i = unused[0];
                real_evil.insert(allergen, i);
                used.insert(i);
            }
        }
    }

    let mut allergens = real_evil.keys().collect::<Vec<_>>();
    allergens.sort_unstable();
    let mut result = Vec::new();
    for allergen in allergens {
        result.push(real_evil[allergen].clone());
    }

    result.join(",")
}

fn main() {
    let input = parse(&fs::read_to_string(input_file("day21.txt")).unwrap());
    let (p1, dangerous) = part1(&input);
    println!("part1 = {}", p1);
    println!("part2 = {}", part2(&dangerous));
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

    assert_eq!(part1(&tuples).0, 5);
}
