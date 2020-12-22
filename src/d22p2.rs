use std::{
    collections::{hash_map::DefaultHasher, HashSet, VecDeque},
    hash::{Hash, Hasher},
};

use crate::utils::string_from_file;

pub fn run() {
    let input = string_from_file("src/22input");

    println!("{}", solve(input));
}

#[derive(Hash)]
struct GameState {
    p1_deck: VecDeque<usize>,
    p2_deck: VecDeque<usize>,
}

pub fn solve(input: String) -> usize {
    let mut decks = input.split("\n\n");
    let mut p1_deck = decks
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| l.trim().parse::<u8>().unwrap())
        .collect::<VecDeque<_>>();

    let mut p2_deck = decks
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| l.trim().parse::<u8>().unwrap())
        .collect::<VecDeque<_>>();

    println!("p1 {:?} \np2 {:?}", p1_deck, p2_deck);

    let p1_win = game(&mut p1_deck, &mut p2_deck);

    let winning_deck = if p1_win { p1_deck } else { p2_deck };
    let score = winning_deck
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, &card)| acc + card as usize * (i + 1));

    score
}

fn game(p1_deck: &mut VecDeque<u8>, p2_deck: &mut VecDeque<u8>) -> bool {
    let mut previous_round_hashes: HashSet<(u64, u64)> = HashSet::new();

    while !p1_deck.is_empty() && !p2_deck.is_empty() {
        // Infinite game check

        let mut hasher = DefaultHasher::new();
        p1_deck.hash(&mut hasher);
        let p1_deck_hash = hasher.finish();

        p2_deck.hash(&mut hasher);
        let p2_deck_hash = hasher.finish();

        match previous_round_hashes.get(&(p1_deck_hash, p2_deck_hash)) {
            Some(_) => return true,
            None => {}
        };

        previous_round_hashes.insert((p1_deck_hash, p2_deck_hash));

        // Draw
        let p1_draw = p1_deck.pop_front().unwrap() as usize;
        let p2_draw = p2_deck.pop_front().unwrap() as usize;

        // If both players have at least as many cards remaining in their deck
        // as the value of the card they just drew, the winner of the round is
        // determined by playing a new game of Recursive Combat
        let p1_win = if p1_deck.len() >= p1_draw && p2_deck.len() >= p2_draw {
            let mut p1_subdeck = p1_deck.clone();
            p1_subdeck.truncate(p1_draw);

            let mut p2_subdeck = p2_deck.clone();
            p2_subdeck.truncate(p2_draw);

            // println!("p1_subdeck: {:?}\np2_subdeck: {:?}", p1_subdeck, p2_subdeck);
            game(&mut p1_subdeck, &mut p2_subdeck)
        } else {
            p1_draw > p2_draw
        };

        if p1_win {
            p1_deck.push_back(p1_draw as u8);
            p1_deck.push_back(p2_draw as u8);
        } else {
            p2_deck.push_back(p2_draw as u8);
            p2_deck.push_back(p1_draw as u8);
        }
    }

    !p1_deck.is_empty()
}

#[test]
fn example_1() {
    let input = "Player 1:
43
19

Player 2:
2
29
14";

    // Expected solution unknown. Expect to not be infinite
    solve(input.to_string());
}

#[test]
fn example_2() {
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

    assert_eq!(solve(input.to_string()), 291);
}
