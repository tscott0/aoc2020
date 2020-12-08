use regex::Regex;

use crate::utils::string_from_file;
use std::collections::{HashMap, HashSet};

pub fn run() {
    let input = string_from_file("src/07input");

    println!("{}", solve(input));
}

#[derive(Debug)]
struct Bag {
    count: usize,
    name: String,
}

struct ContainBag {
    contains: Vec<Bag>,
    name: String,
}

pub fn solve(input: String) -> usize {
    let clean_re = Regex::new(r" (bag[s]?[,.]?|contain|no other)").unwrap();

    let line_re = Regex::new(r"([a-z ]+) (([0-9]+) ([a-z ]+))*").unwrap();
    let bag_re = Regex::new(r"([0-9]+) ([a-z ]+)").unwrap();

    let mut unique_bags: HashSet<String> = HashSet::new();

    let bags: HashMap<String, Vec<Bag>> = input
        .lines()
        .map(|l| clean_re.replace_all(l.trim(), "").to_string())
        .map(|l| {
            // println!("\nLINE: {}", l);

            // let captures = re.captures(l.as_str());
            // println!("{:?}", captures);
            let bag = line_re
                .captures(l.as_str())
                .unwrap()
                .iter()
                .skip(1)
                .next()
                .unwrap()
                .unwrap()
                .as_str()
                .to_string();

            println!("  BAG: {:?}", bag);

            unique_bags.insert(bag.to_string());

            let captures = bag_re.captures_iter(l.as_str());
            let contains_bags = captures
                .map(|c| {
                    // println!("    captures: {:?}", c);

                    let b = c[2].trim().to_string();
                    unique_bags.insert(b.clone());
                    Bag {
                        count: c[1].parse::<usize>().unwrap(),
                        name: b,
                    }
                })
                .collect::<Vec<Bag>>();

            contains_bags.iter().for_each(|b| {
                println!("    {:?}", b);
            });

            (bag, contains_bags)
        })
        .collect();

    bags.iter()
        .filter(|(k, v)| v.iter().any(|b| holds_gold(&bags, b)))
        .count()
}

fn holds_gold(all_bags: &HashMap<String, Vec<Bag>>, current_bag: &Bag) -> bool {
    if "shiny gold" == current_bag.name.as_str() {
        return true;
    }

    match all_bags.get(&current_bag.name) {
        Some(v) => v.iter().any(|b| holds_gold(all_bags, b)),
        None => false,
    }
}

#[test]
fn example_1() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    assert_eq!(solve(input.to_string()), 4);
}
