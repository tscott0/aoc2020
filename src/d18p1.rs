use crate::utils::string_from_file;
use regex::Regex;

pub fn run() {
    let input = string_from_file("src/18input");

    println!("{}", solve(input));
}

pub fn solve(input: String) -> usize {
    let re = Regex::new(r"(.*) ?(\([^()]*\)){1} ?(.*)").unwrap();

    input.lines().map(|l| solve_equation(l, &re)).sum()
}

fn solve_equation(l: &str, re: &Regex) -> usize {
    println!("solve_equation \"{}\"", l);
    let brackets = Regex::new(r"^\(([^()]*)\)$").unwrap();

    let l = if brackets.is_match(l.trim()) {
        let inside = brackets
            .captures(l)
            .unwrap()
            .iter()
            .skip(1)
            .next()
            .unwrap()
            .unwrap()
            .as_str();
        inside
    } else {
        l
    };

    println!("l: {} captures: {:?}", l, re.captures(l));

    match re.captures(l) {
        None => {
            let mut tokens = l.split(" ");
            let mut total = tokens.next().unwrap().parse::<usize>().unwrap();

            for chunk in tokens.collect::<Vec<_>>().chunks(2) {
                println!("chunk: {:?}", chunk);

                let operator = chunk[0];
                let operand = chunk[1].trim().parse::<usize>().unwrap();

                match operator {
                    "+" => total += operand,
                    "*" => total *= operand,
                    _ => {
                        panic!("Unexpected operator")
                    }
                }
            }

            total
        }
        Some(c) => {
            let to_replace = c.get(2).unwrap().as_str();
            let replacement = solve_equation(to_replace, re);
            println!("substituting {} with {}", to_replace, replacement);
            let new_equation = l.replacen(to_replace, replacement.to_string().as_str(), 1);

            solve_equation(new_equation.as_str(), re)
        }
    }
}

#[test]
fn example_1() {
    let input = "1 + 2 * 3 + 4 * 5 + 6";

    assert_eq!(solve(input.to_string()), 71);
}

#[test]
fn example_2() {
    let input = "2 * 3 + (4 * 5)";

    assert_eq!(solve(input.to_string()), 26);
}

#[test]
fn example_3() {
    let input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";

    assert_eq!(solve(input.to_string()), 12240);
}
