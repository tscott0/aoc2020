use crate::utils::string_from_file;
use std::collections::HashSet;

pub fn run() {
    let input = string_from_file("src/08input");

    println!("{}", solve(input));
}

enum Operation {
    NoOp,
    Accumulate(isize),
    Jump(isize),
}

pub fn solve(input: String) -> isize {
    let instructions: Vec<Operation> = input
        .lines()
        .map(|l| {
            let tokens: Vec<&str> = l.trim().split(" ").collect();

            let magnitude = tokens[1].parse::<isize>().unwrap();
            match tokens[0] {
                "nop" => Operation::NoOp,
                "acc" => Operation::Accumulate(magnitude),
                "jmp" => Operation::Jump(magnitude),
                _ => {
                    panic!("unexpected operation: {}", tokens[0]);
                }
            }
        })
        .collect();

    let mut visited_instructions: HashSet<usize> = HashSet::new();
    let mut cursor: usize = 0;
    let mut accumulator: isize = 0;

    loop {
        if visited_instructions.contains(&cursor) {
            return accumulator;
        }
        visited_instructions.insert(cursor.clone());

        match &instructions[cursor] {
            Operation::NoOp => {}
            Operation::Accumulate(m) => {
                accumulator += m;
            }
            Operation::Jump(m) => {
                cursor = (cursor as isize + m) as usize;
                continue;
            }
        }

        cursor += 1;
    }
}

#[test]
fn example_1() {
    let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    assert_eq!(solve(input.to_string()), 5);
}
