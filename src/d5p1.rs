use crate::utils::string_from_file;

pub fn run() {
    let input = string_from_file("src/05input");

    println!("{}", solve(input));
}

pub fn solve(input: String) -> usize {
    input
        .lines()
        .map(|l| {
            let row_binary = l
                .trim()
                .replace("B", "1")
                .replace("F", "0")
                .chars()
                .take(7)
                .collect::<String>();
            let row_num = usize::from_str_radix(row_binary.as_str(), 2).unwrap();

            let col_binary = l
                .trim()
                .replace("R", "1")
                .replace("L", "0")
                .chars()
                .skip(7)
                .take(3)
                .collect::<String>();
            let col_num = usize::from_str_radix(col_binary.as_str(), 2).unwrap();

            let coefficient = 8;
            let answer = row_num * coefficient + col_num;
            println!("{} * {} + {} = {}", row_num, coefficient, col_num, answer);

            answer
        })
        .max()
        .unwrap()
}

#[test]
fn example_1() {
    let input = "BFFFBBFRRR
                 FFFBBBFRRR
                 BBFFBBFRLL";

    assert_eq!(solve(input.to_string()), 820);
}
