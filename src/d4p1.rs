use crate::utils::string_from_file;
use itertools::Itertools;

pub fn run() {
    let input = string_from_file("src/04input");

    println!("{}", solve(input));
}

pub fn solve(input: String) -> usize {
    input.split("\n\n").map(|p| {
        let l = p.lines().join(" ");
        println!("{}", l);
        l
    }).filter(|l| {
        l.contains("byr:") &&
        l.contains("iyr:") &&
        l.contains("eyr:") &&
        l.contains("hgt:") &&
        l.contains("hcl:") &&
        l.contains("ecl:") &&
        l.contains("pid:")
    }).count()
}

#[test]
fn example_1() {
    let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    assert_eq!(solve(input.to_string()), 2);
}
