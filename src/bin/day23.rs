#[derive(Debug, Clone, PartialEq)]
struct Cup {
    value: i64,
    next: usize,
}

fn parse(input: &str) -> (usize, usize, Vec<Cup>) {
    let mut cups = vec![Cup { value: 0, next: 0 }; input.len()];
    let numbers = input
        .chars()
        .flat_map(|c| c.to_digit(10))
        .map(|n| n as usize)
        .collect::<Vec<_>>();

    let mut last_idx = numbers.last().unwrap() - 1;

    for c in input.chars() {
        let value = c.to_digit(10).unwrap() as i64;

        cups[last_idx].next = (value - 1) as usize;

        cups[(value - 1) as usize] = Cup { value, next: 0 };
        last_idx = (value - 1) as usize;
    }

    cups[last_idx].next = numbers.first().unwrap() - 1;

    (
        numbers.first().unwrap() - 1,
        numbers.last().unwrap() - 1,
        cups,
    )
}

fn solve((start, numbers): &(usize, &[Cup]), steps: usize) -> Vec<Cup> {
    let mut cups = numbers.to_vec();
    let mut current_cup_idx = *start;

    for _ in 0..steps {
        let crab1_idx = cups[current_cup_idx].next;
        let crab2_idx = cups[crab1_idx].next;
        let crab3_idx = cups[crab2_idx].next;

        let mut dest_idx;
        let mut search_value = if cups[current_cup_idx].value <= 1 {
            numbers.len() as i64
        } else {
            cups[current_cup_idx].value - 1
        };

        let mut i = 0;
        loop {
            dest_idx = (search_value - 1) as usize;
            let search_cup = &cups[dest_idx];

            if search_cup == &cups[crab1_idx]
                || search_cup == &cups[crab2_idx]
                || search_cup == &cups[crab3_idx]
            {
                search_value = if search_value <= 1 {
                    numbers.len() as i64
                } else {
                    search_value - 1
                };
            } else {
                break;
            }

            i += 1;
            assert!(i < 100);
        }

        let after_crab3 = cups[crab3_idx].next;
        cups[crab3_idx].next = cups[dest_idx].next;
        cups[dest_idx].next = cups[current_cup_idx].next; // index of crab3
        cups[current_cup_idx].next = after_crab3;

        current_cup_idx = cups[current_cup_idx].next;
    }

    cups
}

fn part1((start, numbers): &(usize, &[Cup])) -> String {
    let cups = solve(&(*start, numbers), 100);
    let mut idx = cups[0].next;
    let mut result = String::new();
    while cups[idx].value != 1 {
        result.push(std::char::from_digit(cups[idx].value as u32, 10).unwrap());
        idx = cups[idx].next;
    }

    result
}

fn part2((start, numbers): &(usize, &[Cup])) -> i64 {
    let cups = solve(&(*start, numbers), 10_000_000);

    cups[cups[0].next].value * cups[cups[cups[0].next].next].value
}

fn create_list(
    mut cups: Vec<Cup>,
    start: usize,
    end: usize,
    n: usize,
) -> Vec<Cup> {
    cups.reserve(n);
    cups[end].next = cups.len();

    for i in cups.len()..n {
        cups.push(Cup {
            value: (i + 1) as i64,
            next: (i + 1) % n,
        })
    }

    cups.last_mut().unwrap().next = start;

    assert_eq!(cups.len(), n);

    cups
}

fn main() {
    let (start, end, cups) = parse("137826495");
    println!("part1 = {}", part1(&(start, &cups)));
    let cups = create_list(cups, start, end, 1_000_000);
    println!("part2 = {}", part2(&(start, &cups)));
}

#[test]
fn test_day23() {
    let (start, _end, cups) = parse("389125467");
    assert_eq!(part1(&(start, &cups)), "67384529");
    // let cups = &create_list(cups, start, _end, 1_000_000);
    // assert_eq!(part2(&(start, cups)), 149245887792);
}
