use crate::utils::string_from_file;

pub fn run() {
    let input = string_from_file("src/02input");

    println!("{}", solve(input));
}

struct PasswordPolicy2 {
    pos_1: usize,
    pos_2: usize,
    char_to_find: char,
    password: String,
}

impl PasswordPolicy2 {
    fn valid(&self) -> bool {
        let chars = self.password.chars().collect::<Vec<char>>();
        (chars[self.pos_1] == self.char_to_find && chars[self.pos_2] != self.char_to_find)
            || (chars[self.pos_1] != self.char_to_find && chars[self.pos_2] == self.char_to_find)
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

            PasswordPolicy2 {
                pos_1: range[0] - 1,
                pos_2: range[1] - 1,
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

    assert_eq!(solve(input.to_string()), 1);
}
