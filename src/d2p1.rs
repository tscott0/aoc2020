use crate::utils::string_from_file;
use itertools::Itertools;

pub fn run() {
    let input = string_from_file("src/02input");

    println!("{}", solve(input));
}

struct PasswordPolicy {
    min: usize,
    max: usize,
    char_to_find: char,
    password: String,
}

impl PasswordPolicy {
    fn valid(&self) -> bool {
        let occurrences = self
            .password
            .chars()
            .filter(|c| *c == self.char_to_find)
            .count();

        occurrences >= self.min && occurrences <= self.max
    }
}

fn solve(input: String) -> usize {
    input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.trim().split(" ").collect();
            if parts.len() != 3 {
                panic!("Each row must have 3 parts")
            }
            let password = parts[2].to_string();

            let range: Vec<usize> = parts[0]
                .split('-')
                .map(|r| r.parse::<usize>().unwrap())
                .collect();

            let char_to_find = parts[1].chars().next().unwrap();

            PasswordPolicy {
                min: range[0],
                max: range[1],
                char_to_find,
                password,
            }
        })
        .filter(|p| p.valid())
        .count()
}

#[test]
fn example_1() {
    let input = "1-3 a: abcde
    1-3 b: cdefg
    2-9 c: ccccccccc";

    assert_eq!(solve(input.to_string()), 2);
}
