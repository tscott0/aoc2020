use crate::utils::string_from_file;

pub fn run() {
    let input = string_from_file("src/05input");

    println!("{}", solve(input));
}

pub fn solve(input: String) -> usize {
    let mut min_possible = 999;
    let mut max_possible = 0;
    let mut boarding_passes = input
        .lines()
        .filter_map(|l| {
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

            // Filter out the invalid IDs
            if row_num == 0 || row_num == 127 {
                return None;
            }

            if answer < min_possible {
                min_possible = answer
            }

            if answer > max_possible {
                max_possible = answer
            }

            Some(answer)
        })
        .collect::<Vec<usize>>();
    boarding_passes.sort();

    println!(
        "My seat ID must be between {} and {} inclusive",
        min_possible, max_possible
    );
    (min_possible..=max_possible)
        .filter(|p| !boarding_passes.contains(p))
        .next()
        .unwrap()
}
