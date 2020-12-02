use crate::utils::string_from_file;
use itertools::Itertools;

pub fn run() {
    let input = string_from_file("src/01input");

    solve(input, 2);
}

pub fn solve(input: String, k: usize) -> usize {
    let answers = input
        .lines()
        .map(|x| x.trim().parse::<usize>().unwrap())
        .combinations(k)
        .filter(|c| {
            // println!("{:?}", c);
            c.iter().sum::<usize>() == 2020
        })
        .collect::<Vec<Vec<usize>>>();

    if answers.len() == 0 {
        println!("No solutions found found");
        std::process::exit(1)
    }
    let product = answers[0].iter().product::<usize>();

    println!("Answer: product({:?}) = {:?}", answers[0], product);

    product
}

#[test]
fn example_1() {
    let input = "1721
    979
    366
    299
    675
    1456";

    assert_eq!(solve(input.to_string(), 2), 514579);
}
