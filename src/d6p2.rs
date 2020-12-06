use crate::utils::string_from_file;
use std::collections::HashSet;

pub fn run() {
    let input = string_from_file("src/06input");

    println!("{}", solve(input));
}

pub fn solve(input: String) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let mut distinct_questions: HashSet<char> = HashSet::new();
            group.lines().for_each(|l| {
                // println!("line: {}", l);

                l.trim().chars().for_each(|c| {
                    distinct_questions.insert(c);
                })
            });

            let mut all_yes = distinct_questions.clone();
            group.lines().for_each(|l| {
                // println!("line: {}", l);

                let individual_answers = l.trim().chars().collect::<Vec<char>>();
                distinct_questions.iter().for_each(|q| {
                    if !individual_answers.contains(q) {
                        all_yes.remove(q);
                    }
                })
            });
            all_yes.len()
        })
        .sum()
}

#[test]
fn example_1() {
    let input = "abc

a
b
c

ab
ac

a
a
a
a

b";

    assert_eq!(solve(input.to_string()), 6);
}
