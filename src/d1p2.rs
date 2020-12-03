use crate::{d1p1::solve, utils::string_from_file};

pub fn run() {
    let input = string_from_file("src/01input");

    solve(input, 3);
}

#[test]
fn example_2() {
    let input = "1721
    979
    366
    299
    675
    1456";

    assert_eq!(solve(input.to_string(), 3), 241861950);
}
