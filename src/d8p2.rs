use crate::utils::string_from_file;
use std::collections::HashSet;

pub fn run() {
    let input = string_from_file("src/08input");

    println!("{}", solve(input));
}

#[derive(Copy, Clone)]
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

    (0..instructions.len())
        .find_map(|i| {
            let mut visited_instructions: HashSet<usize> = HashSet::new();
            let mut cursor: usize = 0;
            let mut accumulator: isize = 0;

            let mut attempt = instructions.clone();
            let op = attempt[i];

            // If operation is jmp or nop, try to swap
            match op {
                Operation::NoOp => {
                    attempt[i] = Operation::Jump(0);
                }
                Operation::Jump(_) => {
                    attempt[i] = Operation::NoOp;
                }
                Operation::Accumulate(_) => return None,
            }

            loop {
                // Prevent infinite loops
                if visited_instructions.contains(&cursor) {
                    break;
                }
                // Yield accumulator if the cursor is "immediately after
                // the last instruction in the file"
                if cursor == instructions.len() {
                    return Some(accumulator);
                }
                visited_instructions.insert(cursor.clone());

                match &attempt[cursor] {
                    Operation::Accumulate(m) => {
                        accumulator += m;
                    }
                    Operation::Jump(m) => {
                        cursor = (cursor as isize + m) as usize;
                        continue;
                    }
                    Operation::NoOp => {}
                }

                cursor += 1;
            }

            None
        })
        .unwrap()
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

    assert_eq!(solve(input.to_string()), 8);
}
