use std::collections::HashMap;

pub fn run() {
    println!("{}", solve("2,0,1,9,5,19".to_string(), 2020));
}

pub fn solve(input: String, round: usize) -> usize {
    let mut spoken_numbers: HashMap<usize, Vec<usize>> = HashMap::new();
    let starting_numbers = input
        .trim()
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut game: Vec<usize> = Vec::new();

    starting_numbers.iter().enumerate().for_each(|(i, &n)| {
        spoken_numbers.entry(n).or_insert(vec![]).push(i);
        game.push(n);
    });

    loop {
        let current_turn_index = game.len();
        let last_turn_index = current_turn_index - 1;
        let last_turn_num = game[last_turn_index];

        if current_turn_index == round {
            return last_turn_num;
        }

        let number_to_speak: usize = match spoken_numbers.get(&last_turn_num).clone() {
            Some(occurrences) => {
                // If that was the first time the number has been spoken, the current player says 0.
                if occurrences.len() == 1 {
                    println!("never seen, 0");
                    0
                // Otherwise, the number had been spoken before; the current player announces how
                // many turns apart the number is from when it was previously spoken.
                } else {
                    let previous_at = occurrences.get(occurrences.len() - 2).unwrap();
                    println!(
                        "already seen, {} - {} = {}",
                        last_turn_index,
                        previous_at,
                        last_turn_index - previous_at
                    );
                    last_turn_index - previous_at
                }
            }
            None => {
                panic!("Shouldn't happen!")
            }
        };

        spoken_numbers
            .entry(number_to_speak)
            .or_insert(vec![current_turn_index])
            .push(current_turn_index);

        game.push(number_to_speak);
    }
}

#[test]
fn example_1() {
    let input = "0,3,6";

    assert_eq!(solve(input.to_string(), 10), 0);
}

#[test]
fn example_2() {
    let input = "0,3,6";

    assert_eq!(solve(input.to_string(), 2020), 436);
}

#[test]
fn example_3() {
    let input = "1,3,2";

    assert_eq!(solve(input.to_string(), 2020), 1);
}
