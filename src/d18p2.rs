use crate::utils::string_from_file;
use regex::Regex;

pub fn run() {
    let input = string_from_file("src/18input");

    println!("{}", solve(input));
}

pub fn solve(input: String) -> usize {
    let perens_re = Regex::new(r"\([^()]+\)").unwrap();
    let add_re = Regex::new(r"[0-9]+ \+ [0-9]+").unwrap();
    let mul_re = Regex::new(r"[0-9]+ \* [0-9]+").unwrap();

    input
        .lines()
        .map(|l| solve_equation(l, &perens_re, &add_re, &mul_re))
        .sum()
}

fn solve_equation(l: &str, perens_re: &Regex, add_re: &Regex, mul_re: &Regex) -> usize {

    let brackets = Regex::new(r"^\(([^()]*)\)$").unwrap();
    let l = if brackets.is_match(l.trim()) {
        brackets.captures(l).unwrap().get(1).unwrap().as_str()
    } else {
        l
    };

    // println!("l: {} captures: {:?}", l, perens_re.captures(l));
    match perens_re.captures(l) {
        Some(perens_captures) => {
            let to_replace = perens_captures.get(0).unwrap().as_str();

            let answer = solve_equation(to_replace, perens_re, add_re, mul_re);
            // println!("substituting {} with {}", to_replace, answer);

            let new_equation = l.replacen(to_replace, answer.to_string().as_str(), 1);
            solve_equation(new_equation.as_str(), perens_re, add_re, mul_re)
        }
        None => match add_re.captures(l) {
            Some(add_captures) => {
                let first_add = add_captures.get(0).unwrap().as_str();

                let answer = first_add
                    .split(" + ")
                    .map(|n| n.parse::<usize>().unwrap())
                    .sum::<usize>();

                // println!("adding {} and replacing with {}", first_add, answer);
                let new_l = add_re.replacen(l, 1, answer.to_string().as_str());

                solve_equation(&new_l, perens_re, add_re, mul_re)
            }
            None => match mul_re.captures(l) {
                Some(mul_captures) => {
                    let first_mul = mul_captures.get(0).unwrap().as_str();

                    let answer = first_mul
                        .split(" * ")
                        .map(|n| n.parse::<usize>().unwrap())
                        .product::<usize>();

                    // println!("mutliplying {} and replacing with {}", first_mul, answer);
                    let new_l = mul_re.replacen(l, 1, answer.to_string().as_str());

                    solve_equation(&new_l, perens_re, add_re, mul_re)
                }
                None => {
                    l.parse::<usize>().unwrap()
                }
            },
        },
    }
}

#[test]
fn example_1() {
    let input = "1 + 2 * 3 + 4 * 5 + 6";

    assert_eq!(solve(input.to_string()), 231);
}

#[test]
fn example_2() {
    let input = "1 + (2 * 3) + (4 * (5 + 6))";

    assert_eq!(solve(input.to_string()), 51);
}

#[test]
fn example_3() {
    let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    assert_eq!(solve(input.to_string()), 23340);
}
