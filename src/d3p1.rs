use crate::utils::string_from_file;
use itertools::Itertools;

pub fn run() {
    let input = string_from_file("src/03input");

    println!("{}", slope(input, 3, 1));
}

pub fn slope(input: String, right: usize, down: usize) -> usize {
    let width = input.lines().next().unwrap().len();

    let grid = input.lines().flat_map(|l| l.trim().chars()).collect_vec();

    let mut x = 0;
    let mut y = 0;

    let mut answer = 0;

    loop {
        x += right;
        y += down;

        let mut cursor= y * width;

        cursor += x % width;


        if cursor >= grid.len()  {
            return answer;
        }

        if grid[cursor] == '#' {
            answer += 1;
        }
    }
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

    assert_eq!(slope(input.to_string(), 3, 1), 7);
}
