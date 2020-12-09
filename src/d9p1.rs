use crate::utils::string_from_file;
use itertools::Itertools;

pub fn run() {
    let input = string_from_file("src/09input");

    println!("{}", solve(input, 25));
}

pub fn solve(input: String, window: usize) -> usize {
    let data = input
        .lines()
        .map(|l| l.trim().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut cursor = 0;
    loop {
        let target = data[cursor + window];
        let previous_n = &data[cursor..cursor + window];

        if previous_n
            .iter()
            .combinations(2)
            .any(|c| c.iter().map(|x| x.clone()).to_owned().sum::<usize>() == target)
        {
            cursor += 1;
            continue;
        }

        return target;
    }
}

#[test]
fn example_1() {
    let input = "35
    20
    15
    25
    47
    40
    62
    55
    65
    95
    102
    117
    150
    182
    127
    219
    299
    277
    309
    576";

    assert_eq!(solve(input.to_string(), 5), 127);
}
