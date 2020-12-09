use crate::utils::string_from_file;

pub fn run() {
    let input = string_from_file("src/09input");

    println!("{}", solve(input, 25918798));
}

pub fn solve(input: String, target: usize) -> usize {
    let data = input
        .lines()
        .map(|l| l.trim().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut cursor = 0;
    loop {
        let mut window_size = 2;

        loop {
            let attempt = &data[cursor..cursor + window_size];
            let sum = attempt.iter().map(|x| x.clone()).sum::<usize>();

            if sum == target {
                println!("Values {:?} sum to {}", attempt, target);
                return attempt.iter().min().unwrap() + attempt.iter().max().unwrap();
            }

            if sum > target {
                break;
            }

            window_size += 1;
        }

        cursor += 1;
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

    assert_eq!(solve(input.to_string(), 127), 62);
}
