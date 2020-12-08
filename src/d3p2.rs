
use crate::d3p1::slope;
use crate::utils::string_from_file;

pub fn run() {
    let input = string_from_file("src/03input");

    println!("{}", solve(input));
}

fn solve(input: String) -> usize {
    slope(input.clone(), 1, 1) *
    slope(input.clone(), 3, 1) *
    slope(input.clone(), 5, 1) *
    slope(input.clone(), 7, 1) *
    slope(input.clone(), 1, 2)
}

#[test]
fn example_1() {
    let input = "..##.......
                         #...#...#..
                         .#....#..#.
                         ..#.#...#.#
                         .#...##..#.
                         ..#.##.....
                         .#.#.#....#
                         .#........#
                         #.##...#...
                         #...##....#
                         .#..#...#.#";

    assert_eq!(slope(input.to_string(), 1, 1), 2);
    assert_eq!(slope(input.to_string(), 3, 1), 7);
    assert_eq!(slope(input.to_string(), 5, 1), 3);
    assert_eq!(slope(input.to_string(), 7, 1), 4);
    assert_eq!(slope(input.to_string(), 1, 2), 2);
}
