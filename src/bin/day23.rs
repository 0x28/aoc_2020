#[derive(Debug, Clone, PartialEq)]
struct Cup {
    value: i64,
    next: usize,
}

fn parse(input: &str) -> Vec<Cup> {
    let mut cups = vec![];

    for (idx, c) in input.chars().enumerate() {
        let value = c.to_digit(10).unwrap() as i64;
        let next = (idx + 1) % input.len();

        cups.push(Cup { value, next })
    }

    cups
}

fn part1(numbers: &[Cup]) -> String {
    let mut cups = numbers.to_vec();
    let mut current_cup_idx = 0;

    for _ in 0..100 {
        let crab1_idx = cups[current_cup_idx].next;
        let crab2_idx = cups[crab1_idx].next;
        let crab3_idx = cups[crab2_idx].next;

        let mut dest_idx = cups[crab3_idx].next;
        let mut search_value = cups[current_cup_idx].value - 1;

        loop {
            let search_cup = &cups[dest_idx];

            if search_cup == &cups[current_cup_idx] {
                // wrap
                search_value = if search_value <= 1 {
                    numbers.len() as i64
                } else {
                    search_value - 1
                };
            } else if search_cup.value == search_value
                && search_cup != &cups[crab1_idx]
                && search_cup != &cups[crab2_idx]
                && search_cup != &cups[crab3_idx]
            {
                break;
            }

            dest_idx = search_cup.next;
        }

        let after_crab3 = cups[crab3_idx].next;
        cups[crab3_idx].next = cups[dest_idx].next;
        cups[dest_idx].next = cups[current_cup_idx].next; // index of crab3
        cups[current_cup_idx].next = after_crab3;

        current_cup_idx = cups[current_cup_idx].next;
    }

    let one_idx = cups.iter().position(|c| c.value == 1).unwrap();
    let mut result = String::new();
    let mut idx = cups[one_idx].next;

    while cups[idx].value != 1 {
        result.push(std::char::from_digit(cups[idx].value as u32, 10).unwrap());
        idx = cups[idx].next;
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
