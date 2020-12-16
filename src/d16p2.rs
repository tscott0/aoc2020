use std::collections::{HashMap, HashSet};

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
                    .map(|v| v.trim().parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                (bounds[0], bounds[1])
            })
            .collect::<Vec<(usize, usize)>>();
        rules.insert(name, bound_pairs);
    });

    let your_ticket = sections
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .map(|v| v.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let field_count = your_ticket.len();

    let nearby_tickets = sections
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

    let mut valid_tickets = nearby_tickets
        .iter()
        .filter(|&nearby| {
            nearby.iter().all(|&field| {
                rules.iter().any(|(_, rule_bounds)| {
                    rule_bounds
                        .iter()
                        .any(|(lower, upper)| field >= *lower && field <= *upper)
                })
            })
        })
        .collect::<Vec<_>>();

    // including your ticket
    valid_tickets.push(&your_ticket);

    let mut rule_positions: HashMap<String, usize> = HashMap::new();
    let mut remaining_positions: Vec<usize> = (0..field_count).collect();

    while rules.len() > 0 {
        remaining_positions.clone().iter().for_each(|i| {
            println!("{}", i);
            let mut rc = rules.clone();
            let matching_rules = rc
                .iter_mut()
                .filter(|(r, bound_pairs)| {
                    valid_tickets.iter().map(|&values| values[*i]).all(|v| {
                        let in_bounds = bound_pairs
                            .iter()
                            .any(|(lower, upper)| v >= *lower && v <= *upper);

                        in_bounds
                    })
                })
                .collect::<Vec<_>>();

            if matching_rules.len() == 1 {
                let (rule_name, _) = matching_rules[0];
                rule_positions.insert(rule_name.clone(), *i);

                let index = remaining_positions.iter().position(|x| *x == *i).unwrap();
                remaining_positions.remove(index);

                rules.remove(rule_name);
            }
        });
    }

    rule_positions
        .iter()
        .filter(|(r, _)| r.contains("departure"))
        .map(|(r, _)| {
            let p = rule_positions.get(r).unwrap();

            println!("{}", your_ticket[*p]);
            your_ticket[*p]
        })
        .product::<usize>()
}

#[test]
fn example_1() {
    let input = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

    solve(input.to_string());
}
