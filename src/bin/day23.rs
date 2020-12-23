fn parse(input: &str) -> Vec<u32> {
    input.chars().flat_map(|c| c.to_digit(10)).collect()
}

fn part1(numbers: &[u32]) -> String {
    let mut cups = numbers.iter().copied().collect::<Vec<_>>();
    let mut current_cup = *cups.first().unwrap();

    for _ in 0..100 {
        let mut crab_cups = vec![];

        for _ in 0..3 {
            let next_index =
                (cups.iter().position(|&c| c == current_cup).unwrap() + 1)
                    % cups.len();
            crab_cups.push(cups.remove(next_index));
        }

        let mut dest = if current_cup == 1 {
            numbers.len() as u32
        } else {
            current_cup - 1
        };
        let dest_idx;
        loop {
            if let Some(idx) = cups.iter().position(|&c| c == dest) {
                dest_idx = idx;
                break;
            } else {
                dest = if dest == 1 {
                    numbers.len() as u32
                } else {
                    dest - 1
                };
            }
        }

        for _ in 0..3 {
            cups.insert(dest_idx + 1, crab_cups.pop().unwrap());
        }

        let next_index = (cups.iter().position(|&c| c == current_cup).unwrap()
            + 1)
            % cups.len();
        current_cup = cups[next_index];
    }

    let one_idx = cups.iter().position(|&c| c == 1).unwrap();
    let mut result = String::new();
    let mut idx = one_idx + 1;

    while cups[idx % cups.len()] != 1 {
        result.push(std::char::from_digit(cups[idx % cups.len()], 10).unwrap());
        idx += 1;
    }

    result
}

fn main() {
    let numbers = parse("137826495");
    println!("part1 = {}", part1(&numbers))
}

#[test]
fn test_day23() {
    assert_eq!(part1(&parse("389125467")), "67384529");
}
