use aoc_2020::input_file;

use std::collections::HashMap;
use std::fs;

type Passports = Vec<HashMap<String, String>>;

fn parse_passports(batch: &str) -> Passports {
    let mut passports = vec![];

    for passport in batch.split("\n\n") {
        let mut map = HashMap::new();
        for attribute in passport.split_whitespace() {
            if let [key, value, ..] = attribute
                .split(':')
                .map(str::to_owned)
                .collect::<Vec<String>>()
                .as_slice()
            {
                map.insert(key.clone(), value.clone());
            }
        }
        passports.push(map);
    }

    passports
}

fn has_all_keys(passport: &HashMap<String, String>) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|key| passport.contains_key(*key))
}

fn part1(batch: &str) -> usize {
    parse_passports(batch)
        .iter()
        .filter(|p| has_all_keys(p))
        .count()
}

fn valid_passport(passport: &HashMap<String, String>) -> bool {
    if !has_all_keys(passport) {
        return false;
    }

    let byr: u32 = passport["byr"].parse().unwrap_or_default();
    let iyr: u32 = passport["iyr"].parse().unwrap_or_default();
    let eyr: u32 = passport["eyr"].parse().unwrap_or_default();
    let hgt = &passport["hgt"];

    let (height, unit) = hgt.split_at(hgt.len() - 2);
    let height = height.parse::<u32>();
    let hgt_ok = match (height, unit) {
        (Ok(value), "cm") => value >= 150 && value <= 193,
        (Ok(value), "in") => value >= 59 && value <= 76,
        _ => false,
    };

    let hcl = &passport["hcl"];
    let hcl_ok = hcl.starts_with('#')
        && hcl.len() == 7
        && hcl.chars().skip(1).all(|c| c.is_ascii_hexdigit());

    let ecl_ok = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .contains(&passport["ecl"].as_str());

    let pid_ok = passport["pid"].len() == 9
        && passport["pid"].chars().all(char::is_numeric);

    (byr >= 1920 && byr <= 2002)
        && (iyr >= 2010 && iyr <= 2020)
        && (eyr >= 2020 && eyr <= 2030)
        && (hgt_ok && hcl_ok && ecl_ok && pid_ok)
}

fn part2(batch: &str) -> usize {
    parse_passports(batch)
        .iter()
        .filter(|p| valid_passport(p))
        .count()
}

fn main() {
    let input =
        fs::read_to_string(input_file("day4.txt")).expect("file not found!");
    println!("part 1 = {}", part1(&input));
    println!("part 2 = {}", part2(&input));
}

#[test]
fn test_day4() {
    let example1 = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    assert_eq!(part1(example1), 2);

    let example2 = "\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
";

    assert_eq!(part2(example2), 4);
}
