use itertools::Itertools;
use regex::Regex;

use crate::utils::string_from_file;

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
    }).filter(|l| {
        passport_valid(l)
    }).count()
}

fn passport_valid(passport: &String) -> bool {
    let cm_re = Regex::new(r"^[0-9]+cm$").unwrap();
    let in_re = Regex::new(r"^[0-9]+in$").unwrap();
    let hair_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let eye_re = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
    let pid_re = Regex::new(r"^[0-9]{9}$").unwrap();

    passport.split(" ").map(|l| {
        l.split(':').collect_vec()
    }).all(|p| {
        let val = p[1];
        match p[0] {
            "byr" => {
                let year = val.parse::<usize>().unwrap();
                year >= 1920 && year <= 2002
            }
            "iyr" => {
                let year = val.parse::<usize>().unwrap();
                year >= 2010 && year <= 2020
            }
            "eyr" => {
                let year = val.parse::<usize>().unwrap();
                year >= 2020 && year <= 2030
            }
            "hgt" => {
                if cm_re.is_match(val) {
                    let height = val.replace("cm", "").parse::<usize>().unwrap();
                    height >= 150 && height <= 193
                } else if in_re.is_match(val) {
                    let height = val.replace("in", "").parse::<usize>().unwrap();
                    height >= 59 && height <= 76
                } else {
                    false
                }
            }
            "hcl" => {
                hair_re.is_match(val)
            }
            "ecl" => {
                eye_re.is_match(val)
            }
            "pid" => {
                pid_re.is_match(val)
            }
            "cid" => {
                true
            }
            _ => {
                panic!("unexpected passport entry: {}", p[0]);
            }
        }
    })
}

#[test]
fn example_1() {
    let input = "eyr:1972 cid:100
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

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    assert_eq!(solve(input.to_string()), 4);
}
