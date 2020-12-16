use std::collections::HashMap;

use crate::utils::string_from_file;

pub fn run() {
    let input = string_from_file("src/16input");

    println!("{}", solve(input));
}

pub fn solve(input: String) -> usize {
    let mut rules: HashMap<String, Vec<(usize, usize)>> = HashMap::new();

    let mut sections = input.split("\n\n");

    sections.next().unwrap().lines().for_each(|l| {
        if l.trim() == "" {
            return;
        }

        let mut parts = l.split(":");

        let name = parts.next().unwrap().to_string();
        let bound_pairs = parts
            .next()
            .unwrap()
            .split(" or ")
            .map(|b| {
                let bounds = b
                    .split("-")
                    .map(|v| {
                        v.trim().parse::<usize>().unwrap()
                    })
                    .collect::<Vec<_>>();
                (bounds[0], bounds[1])
            })
            .collect::<Vec<(usize, usize)>>();
        rules.insert(name, bound_pairs);
    });

    // let your_ticket = sections
    //     .next()
    //     .unwrap()
    //     .lines()
    //     .skip(1)
    //     .next()
    //     .unwrap()
    //     .split(",")
    //     .map(|v| v.trim().parse::<usize>().unwrap())
    //     .collect::<Vec<_>>();

    let nearby_tickets = sections
        .skip(1)
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| {
            l.trim()
                .split(",")
                .map(|v| v.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    nearby_tickets
        .iter()
        .map(|nb| {
            nb.iter().filter(|&field| {
                let field_ok = rules.iter().any(|(_, rule_bounds)| {
                    rule_bounds
                        .iter()
                        .any(|(lower, upper)| *field >= *lower && *field <= *upper)
                });
                !field_ok
            })
        })
        .flatten()
        .map(|v| *v)
        .sum::<usize>()
}

#[test]
fn example_1() {
    let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    assert_eq!(solve(input.to_string()), 71);
}
