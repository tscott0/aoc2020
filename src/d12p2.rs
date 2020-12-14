use crate::utils::string_from_file;

pub fn run() {
    let input = string_from_file("src/12input");

    println!("{}", solve(input));
}

enum Instruction {
    N(usize),
    S(usize),
    E(usize),
    W(usize),
    L(usize),
    R(usize),
    F(usize),
}

pub fn solve(input: String) -> usize {
    let mut waypoint: (isize, isize) = (10, 1);
    let xy = input
        .lines()
        .map(|l| -> Instruction {
            let chars = l.trim().chars().collect::<Vec<char>>();
            let dir = chars[0];
            let magnitude = chars[1..]
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
            match dir {
                'N' => Instruction::N(magnitude),
                'S' => Instruction::S(magnitude),
                'E' => Instruction::E(magnitude),
                'W' => Instruction::W(magnitude),
                'L' => Instruction::L(magnitude),
                'R' => Instruction::R(magnitude),
                'F' => Instruction::F(magnitude),
                _ => panic!("Unexpected instruction {}", dir),
            }
        })
        .filter_map(|i| match i {
            Instruction::N(m) => {
                println!("North {}", m);
                waypoint.1 += m as isize;
                None
            }
            Instruction::S(m) => {
                println!("South {}", m);
                waypoint.1 -= m as isize;
                None
            }
            Instruction::E(m) => {
                println!("East {}", m);
                waypoint.0 += m as isize;
                None
            }
            Instruction::W(m) => {
                println!("West {}", m);
                waypoint.0 -= m as isize;
                None
            }
            Instruction::L(m) => {
                rotate_left(&mut waypoint, m);
                None
            }
            Instruction::R(m) => {
                rotate_right(&mut waypoint, m);
                None
            }
            Instruction::F(m) => {
                let m = m as isize;
                Some((m * waypoint.0, m * waypoint.1))
            }
        })
        .fold((0, 0), |(accx, accy), (x, y)| {
            println!("x:{} y:{}", accx + x, accy + y);

            (accx + x, accy + y)
        });

    let (x, y) = xy;
    (x.abs() + y.abs()) as usize
}

fn rotate_right(wp: &mut (isize, isize), d: usize) {
    let (ref mut x, ref mut y) = wp;

    let rotations = (d / 90) % 4;
    for _ in 0..rotations {
        let t = *x * -1 as isize;
        *x = *y;
        *y = t;
    }
}

fn rotate_left(wp: &mut (isize, isize), d: usize) {
    rotate_right(wp, 360 - (d % 360))
}

#[test]
fn example_1() {
    let input = "F10
    N3
    F7
    R90
    F11";

    assert_eq!(solve(input.to_string()), 286);
}
