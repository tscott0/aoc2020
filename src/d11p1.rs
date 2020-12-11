use std::fmt;

use crate::utils::string_from_file;

pub fn run() {
    let input = string_from_file("src/11input");

    println!("{}", solve(input));
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Seat {
    Empty,
    Occupied,
    Floor,
}

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        let c = match self {
            Seat::Empty => 'L',
            Seat::Occupied => '#',
            Seat::Floor => '.',
        };
        write!(f, "{}", c)
    }
}

pub fn solve(input: String) -> usize {
    let mut seats = input
        .lines()
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| match c {
                    'L' => Seat::Empty,
                    '#' => Seat::Occupied,
                    '.' => Seat::Floor,
                    _ => panic!("Unexpected seat state"),
                })
                .collect::<Vec<Seat>>()
        })
        .collect::<Vec<_>>();

    loop {
        if !rounds(&mut seats) {
            break;
        };
        // seats.iter().for_each(|r| {
        //     r.iter().for_each(|s| print!("{}", *s));
        //     println!();
        // });
        // println!();
    }

    seats_occupied(&seats)
}

fn rounds(layout: &mut Vec<Vec<Seat>>) -> bool {
    let mut changes: Vec<(usize, usize, Seat)> = Vec::new();

    for (y, row) in layout.iter().enumerate() {
        for (x, seat) in row.iter().enumerate() {
            match seat {
                Seat::Empty => {
                    if adjacents_occupied(layout, x, y) == 0 {
                        changes.push((x, y, Seat::Occupied));
                    }
                }
                Seat::Occupied => {
                    if adjacents_occupied(layout, x, y) >= 4 {
                        changes.push((x, y, Seat::Empty));
                    }
                }
                Seat::Floor => {}
            }
        }
    }

    // No changes
    if changes.is_empty() {
        return false;
    }

    println!("{} changes\n", changes.len());

    // Apply the queued changes simultaneously
    changes.iter().for_each(|(x, y, s)| {
        layout[*y][*x] = *s;
    });

    true
}

fn seats_occupied(layout: &Vec<Vec<Seat>>) -> usize {
    layout
        .iter()
        .map(|r| r.iter().filter(|&s| *s == Seat::Occupied).count())
        .sum()
}

fn adjacents_occupied(layout: &Vec<Vec<Seat>>, column: usize, row: usize) -> usize {
    let row_count = layout.len();
    let column_count = layout.iter().next().unwrap().len();

    let attempts: Vec<(i32, i32)> = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    let adjacent_seat_coords: Vec<(usize, usize)> = attempts
        .iter()
        .filter_map(|&(x, y)| {
            if column == 0 && x == -1 {
                return None;
            }

            if column + 1 == column_count && x == 1 {
                return None;
            }

            if row == 0 && y == -1 {
                return None;
            }

            if row + 1 == row_count && y == 1 {
                return None;
            }

            Some((
                x.checked_add(column as i32).unwrap() as usize,
                y.checked_add(row as i32).unwrap() as usize,
            ))
        })
        .collect();

    adjacent_seat_coords
        .iter()
        .filter(|&c| {
            let (x, y) = *c;

            // println!("x: {} y: {} {:?}", x, y, layout[y][x]);
            layout[y][x] == Seat::Occupied
        })
        .count()
}

#[test]
fn example_1() {
    let input = "L.LL.LL.LL
    LLLLLLL.LL
    L.L.L..L..
    LLLL.LL.LL
    L.LL.LL.LL
    L.LLLLL.LL
    ..L.L.....
    LLLLLLLLLL
    L.LLLLLL.L
    L.LLLLL.LL";

    // 7 * 5 = 35
    assert_eq!(solve(input.to_string()), 37);
}
