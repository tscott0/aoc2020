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
#[derive(Copy, Clone, Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn rotate_right(&mut self, d: usize) -> Direction {
        let new_dir = match (d / 90) % 4 {
            0 => *self,
            1 => match self {
                Direction::N => Direction::E,
                Direction::E => Direction::S,
                Direction::S => Direction::W,
                Direction::W => Direction::N,
            },
            2 => match self {
                Direction::N => Direction::S,
                Direction::E => Direction::W,
                Direction::S => Direction::N,
                Direction::W => Direction::E,
            },
            3 => match self {
                Direction::N => Direction::W,
                Direction::E => Direction::N,
                Direction::S => Direction::E,
                Direction::W => Direction::S,
            },
            _ => {
                panic!("{} not divisible by 90", d);
            }
        };

        println!(
            "current_direction: {:?} rotating {} degrees, now pointing {:?}",
            self, d, new_dir
        );

        return new_dir;
    }

    fn rotate_left(&mut self, d: usize) -> Direction {
        self.rotate_right(360 - (d % 360))
    }
}

pub fn solve(input: String) -> usize {
    let mut current_direction = Direction::E;
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
        .map(|i| match i {
            Instruction::N(m) => {
                println!("North {}", m);
                // current_direction = Direction::N;
                (0, m as isize)
            }
            Instruction::S(m) => {
                println!("South {}", m);
                // current_direction = Direction::S;
                (0, -(m as isize))
            }
            Instruction::E(m) => {
                println!("East {}", m);
                // current_direction = Direction::E;
                (m as isize, 0)
            }
            Instruction::W(m) => {
                println!("West {}", m);
                // current_direction = Direction::W;
                (-(m as isize), 0)
            }
            Instruction::L(m) => {
                current_direction = current_direction.rotate_left(m);
                (0, 0)
            }
            Instruction::R(m) => {
                current_direction = current_direction.rotate_right(m);
                (0, 0)
            }
            Instruction::F(m) => match current_direction {
                Direction::N => {
                    println!("Forward North {}", m);
                    (0, m as isize)
                }
                Direction::S => {
                    println!("Forward South {}", m);
                    (0, -(m as isize))
                }
                Direction::E => {
                    println!("Forward East {}", m);
                    (m as isize, 0)
                }
                Direction::W => {
                    println!("Forward West {}", m);
                    (-(m as isize), 0)
                }
            },
        })
        .fold((0, 0), |(accx, accy), (x, y)| {
            println!("x:{} y:{}", accx + x, accy + y);

            (accx + x, accy + y)
        });

    let (x, y) = xy;

    (x.abs() + y.abs()) as usize
}

#[test]
fn example_1() {
    let input = "F10
    N3
    F7
    R90
    F11";

    assert_eq!(solve(input.to_string()), 25);
}
