use std::collections::VecDeque;

use crate::utils::string_from_file;

pub fn run() {
    let input = string_from_file("src/22input");

    println!("{}", solve(input));
}

pub fn solve(input: String) -> usize {
    let mut decks = input.split("\n\n");
    let mut p1_deck = decks
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| l.trim().parse::<usize>().unwrap())
        .collect::<VecDeque<_>>();

    let mut p2_deck = decks
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| l.trim().parse::<usize>().unwrap())
        .collect::<VecDeque<_>>();

    println!("p1 {:?} p2 {:?}", p1_deck, p2_deck);

    while !p1_deck.is_empty() && !p2_deck.is_empty() {
        let p1_draw = p1_deck.pop_front().unwrap();
        let p2_draw = p2_deck.pop_front().unwrap();

        if p1_draw > p2_draw {
            p1_deck.push_back(p1_draw);
            p1_deck.push_back(p2_draw);
        } else {
            p2_deck.push_back(p2_draw);
            p2_deck.push_back(p1_draw);
        }
    }

    let winning_deck = if p1_deck.is_empty() { p2_deck } else { p1_deck };

    let score = winning_deck
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, card)| acc + card * (i + 1));

    score
}

#[test]
fn example_1() {
    let input = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    assert_eq!(solve(input.to_string()), 306);
}
