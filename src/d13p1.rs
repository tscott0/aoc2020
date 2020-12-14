use crate::utils::string_from_file;

pub fn run() {
    let input = string_from_file("src/13input");

    println!("{}", solve(input));
}

pub fn solve(input: String) -> usize {
    let mut lines_iter = input.lines().map(|l| l.trim());
    let start_time = lines_iter.next().unwrap().parse::<usize>().unwrap();
    let busses = lines_iter
        .next()
        .unwrap()
        .split(',')
        .filter(|&c| c != "x")
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut ts = start_time;
    loop {
        if let Some(bus) = busses.iter().find_map(|b| {
            println!("start: {} ts: {} b: {}", start_time, ts, b);

            if ts % b == 0 {
                Some(b)
            } else {
                None
            }
        }) {
            return (ts - start_time) * bus;
        }

        ts = ts + 1;
    }
}

#[test]
fn example_1() {
    let input = "939
    7,13,x,x,59,x,31,19";

    assert_eq!(solve(input.to_string()), 295);
}
