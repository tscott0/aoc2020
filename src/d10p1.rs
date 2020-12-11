use crate::utils::string_from_file;

pub fn run() {
    let input = string_from_file("src/10input");

    println!("{}", solve(input));
}

pub fn solve(input: String) -> usize {
    let mut one_jolt_count = 0;
    let mut three_jolt_count = 1;

    let mut adapters = input
        .lines()
        .map(|l| l.trim())
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    adapters.sort();

    let mut previous = 0;
    adapters.iter().for_each(|j| {
        let diff = j - previous;
        println!("{}=>{} ({} - {} = {})",previous, j, j, previous, diff);
        match diff {
            1 => one_jolt_count += 1,
            3 => three_jolt_count += 1,
            _ => {}
        }
        previous = *j;
    });


    println!("Ones: {} Threes: {}", one_jolt_count, three_jolt_count);
    one_jolt_count * three_jolt_count
}

#[test]
fn example_1() {
    let input = "16
    10
    15
    5
    1
    11
    7
    19
    6
    12
    4";

    // 7 * 5 = 35
    assert_eq!(solve(input.to_string()), 35);
}

#[test]
fn example_2() {
    let input = "28
    33
    18
    42
    31
    14
    46
    20
    48
    47
    24
    23
    49
    45
    19
    38
    39
    11
    1
    32
    25
    35
    8
    17
    7
    9
    4
    2
    34
    10
    3";

    // 22 * 10 = 220
    assert_eq!(solve(input.to_string()), 220);
}
