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

            (bag, contains_bags)
        })
        .collect();

    let mut calc_string: String = "".to_string();

    count_bags(
        &bags,
        &Bag {
            count: 1,
            name: "shiny gold".to_string(),
        },
        0,
        &mut calc_string,
    )
}

fn count_bags(
    all_bags: &HashMap<String, Vec<Bag>>,
    parent_bag: &Bag,
    level: usize,
    calc: &mut String,
) -> usize {
    // let sum = current_bag.count;

    // print!("-> {} ", &parent_bag.name);

    let result = match all_bags.get(&parent_bag.name) {
        Some(v) => {
            let mut direct_children: usize = 0;
            let contains = v
                .iter()
                .map(|b| {
                    println!(
                        "(+{}) {} contains {} {} bag(s) directly",
                        b.count, &parent_bag.name, b.count, b.name,
                    );
                    direct_children += b.count;

                    // println!("  {} {} * {}", "++".repeat(level), b.count, b.name);

                    // calc.push_str(format!("+ {} * {}", b.count, b.name).as_str());
                    let c = count_bags(all_bags, b, level + 1, calc);
                    // println!("+({} * {})", c, parent_bag.count);
                    c * b.count
                })
                .sum::<usize>();

            contains + direct_children
        }
        None => {
            // println!("NONE {}", &current_bag.name);
            0
        }
    };

    println!("| {} contains {} bags", &parent_bag.name, result);
    // println!("calc={}", calc);
    result
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

    assert_eq!(solve(input.to_string()), 32);
}

#[test]
fn example_2() {
    let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    assert_eq!(solve(input.to_string()), 126);
}
