use std::collections::{HashMap, HashSet};

use crate::utils::string_from_file;

pub fn run() {
    let input = string_from_file("src/17input");

    println!("{}", solve(input, 6));
}

pub fn solve(input: String, cycles: usize) -> usize {
    let mut cube: HashMap<(isize, isize, isize), bool> = HashMap::new();

    let mut min_x: isize = 0;
    let mut min_y: isize = 0;
    let mut min_z: isize = 0;
    let mut max_x: isize = 0;
    let mut max_y: isize = 0;
    let mut max_z: isize = 0;

    input.lines().enumerate().for_each(|(y, l)| {
        l.trim().chars().enumerate().for_each(|(x, r)| {
            min_x = min_x.min(x as isize - 1);
            min_y = min_y.min(y as isize - 1);
            min_z = min_z.min(-1);

            max_x = max_x.max(x as isize + 1);
            max_y = max_y.max(y as isize + 1);
            max_z = max_z.max(1);

            println!("initialising cube {} {} {}, active={}", x, y, 0, r == '#');
            cube.insert((x as isize, y as isize, 0), r == '#');
        })
    });

    let mut active_count = 0;
    for c in 0..cycles {
        // Insert inactive neighbours into the cube before iterating
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    cube.entry((x, y, z)).or_insert_with(|| false);
                }
            }
        }

        let mut changes: HashMap<(isize, isize, isize), bool> = HashMap::new();

        let cube_copy = cube.clone();
        cube_copy.iter().for_each(|(cube, active)| {
            let (x, y, z) = cube.clone();
            min_x = min_x.min(x - 1);
            min_y = min_y.min(y - 1);
            min_z = min_z.min(z - 1);

            max_x = max_x.max(x + 1);
            max_y = max_y.max(y + 1);
            max_z = max_z.max(z + 1);

            let active_neighbours = neighbours(*cube)
                .iter()
                .filter_map(|c| match cube_copy.get(&c) {
                    Some(active_neighbour) => {
                        if *active_neighbour {
                            Some(c)
                        } else {
                            None
                        }
                    }
                    None => None,
                })
                .count();

            if *active && active_neighbours != 2 && active_neighbours != 3 {
                // println!("cube {:?} changing to inactive", cube);
                changes.insert(*cube, false);
            } else if !*active && active_neighbours == 3 {
                // println!("cube {:?} changing to active", cube);
                changes.insert(*cube, true);
            }
        });

        changes.iter().for_each(|(k, v)| {
            cube.insert(*k, *v);
        });

        active_count = cube.iter().filter(|(_, &a)| a).count();

        println!("cycle: {} active: {}", c, active_count);
    }

    active_count
}

fn neighbours(origin: (isize, isize, isize)) -> Vec<(isize, isize, isize)> {
    let (ox, oy, oz) = origin;

    let mut neighbours: Vec<(isize, isize, isize)> = vec![];

    for x in (ox - 1)..=(ox + 1) {
        for y in (oy - 1)..=(oy + 1) {
            for z in (oz - 1)..=(oz + 1) {
                if !(x == ox && y == oy && z == oz) {
                    neighbours.push((x, y, z));
                }
            }
        }
    }

    neighbours
}

#[test]
fn example_1() {
    let input = ".#.
    ..#
    ###";

    assert_eq!(solve(input.to_string(), 3), 38);
}
