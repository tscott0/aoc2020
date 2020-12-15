use crate::utils::string_from_file;

pub fn run() {
    let input = string_from_file("src/13input");

    println!("{}", solve(input));
}

#[derive(PartialEq)]
enum Bus {
    Bus(usize),
    X,
}

pub fn solve(input: String) -> i64 {
    let mut lines_iter = input.lines().map(|l| l.trim());
    lines_iter.next();
    let busses = lines_iter
        .next()
        .unwrap()
        .split(',')
        .map(|b| {
            if b == "x" {
                Bus::X
            } else {
                Bus::Bus(b.parse::<usize>().unwrap())
            }
        })
        .collect::<Vec<Bus>>();

    let mods = busses
        .iter()
        .filter_map(|b| match b {
            Bus::Bus(bus_num) => Some(*bus_num as i64),
            Bus::X => None,
        })
        .collect::<Vec<_>>();

    let res = busses
        .iter()
        .enumerate()
        .filter_map(|(i, b)| match b {
            Bus::Bus(bus_num) => Some(*bus_num as i64 - i as i64),
            Bus::X => None,
        })
        .collect::<Vec<_>>();

    chinese_remainder(&res, &mods).unwrap()
}

// https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

// Keeping this solution as it works for smaller examples
#[allow(dead_code)]
pub fn solve_brute(input: String) -> usize {
    let mut lines_iter = input.lines().map(|l| l.trim());
    lines_iter.next();
    let busses = lines_iter
        .next()
        .unwrap()
        .split(',')
        .map(|b| {
            if b == "x" {
                Bus::X
            } else {
                Bus::Bus(b.parse::<usize>().unwrap())
            }
        })
        .collect::<Vec<Bus>>();

    let first_bus = match busses[0] {
        Bus::Bus(b) => b,
        Bus::X => 1,
    };
    println!("first_bus: {}", first_bus);
    let mut ts = first_bus;

    loop {
        if busses.iter().enumerate().all(|(i, b)| match b {
            Bus::Bus(bus_num) => (ts + i) % bus_num == 0,
            Bus::X => true,
        }) {
            return ts;
        }

        ts = ts + first_bus;
    }
}

#[test]
fn example_1() {
    let input = "XXX
    7,13,x,x,59,x,31,19";

    assert_eq!(solve(input.to_string()), 1068781);
}

#[test]
fn example_2() {
    let input = "XXX
    17,x,13,19";

    assert_eq!(solve(input.to_string()), 3417);
}

#[test]
fn example_3() {
    let input = "XXX
    67,7,59,61";

    assert_eq!(solve(input.to_string()), 754018);
}
