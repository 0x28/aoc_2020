fn do_step(value: u64, subject_number: u64) -> u64 {
    (value * subject_number) % 20201227
}

fn find_loop_size(pkey: u64) -> u64 {
    let subject_number = 7;
    let mut value = 1;
    for size in 1.. {
        value = do_step(value, subject_number);
        if value == pkey {
            return size;
        }
    }

    unreachable!()
}

fn part1(pkey_door: u64, pkey_card: u64) -> u64 {
    let loop_size = find_loop_size(pkey_card);

    (0..loop_size).fold(1, |ekey, _| do_step(ekey, pkey_door))
}

fn main() {
    let pkey1 = 15113849;
    let pkey2 = 4206373;

    println!("part1 = {}", part1(pkey1, pkey2));
}

#[test]
fn test_day25() {
    assert_eq!(find_loop_size(5764801), 8);
    assert_eq!(find_loop_size(17807724), 11);
    assert_eq!(part1(17807724, 5764801), 14897079);
}
