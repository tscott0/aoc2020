use std::env;

mod d10p1;
mod d10p2;
mod d11p1;
mod d11p2;
mod d12p1;
mod d12p2;
mod d13p1;
mod d13p2;
mod d14p1;
mod d14p2;
mod d15p1;
mod d15p2;
mod d16p1;
mod d16p2;
mod d17p1;
mod d17p2;
mod d18p1;
mod d18p2;
mod d21p1;
mod d21p2;
mod d1p1;
mod d1p2;
mod d2p1;
mod d2p2;
mod d3p1;
mod d3p2;
mod d4p1;
mod d4p2;
mod d5p1;
mod d5p2;
mod d6p1;
mod d6p2;
mod d7p1;
mod d7p2;
mod d8p1;
mod d8p2;
mod d9p1;
mod d9p2;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1..].len() != 2 {
        println!("Usage: aoc2020 <day> <part>");
        std::process::exit(1)
    }

    let day = args[1].parse::<i32>().unwrap();
    let part = args[2].parse::<i32>().unwrap();

    if day < 1 || day > 25 || part < 1 || part > 2 {
        println!("Day must be 1-25 and part must be 1 or 2");
        std::process::exit(2)
    }

    println!("Day {:?} Part {:?}", day, part);

    match (day, part) {
        (1, 1) => d1p1::run(),
        (1, 2) => d1p2::run(),
        (2, 1) => d2p1::run(),
        (2, 2) => d2p2::run(),
        (3, 1) => d3p1::run(),
        (3, 2) => d3p2::run(),
        (4, 1) => d4p1::run(),
        (4, 2) => d4p2::run(),
        (5, 1) => d5p1::run(),
        (5, 2) => d5p2::run(),
        (6, 1) => d6p1::run(),
        (6, 2) => d6p2::run(),
        (7, 1) => d7p1::run(),
        (7, 2) => d7p2::run(),
        (8, 1) => d8p1::run(),
        (8, 2) => d8p2::run(),
        (9, 1) => d9p1::run(),
        (9, 2) => d9p2::run(),
        (10, 1) => d10p1::run(),
        (10, 2) => d10p2::run(),
        (11, 1) => d11p1::run(),
        (11, 2) => d11p2::run(),
        (12, 1) => d12p1::run(),
        (12, 2) => d12p2::run(),
        (13, 1) => d13p1::run(),
        (13, 2) => d13p2::run(),
        (14, 1) => d14p1::run(),
        (14, 2) => d14p2::run(),
        (15, 1) => d15p1::run(),
        (15, 2) => d15p2::run(),
        (16, 1) => d16p1::run(),
        (16, 2) => d16p2::run(),
        (17, 1) => d17p1::run(),
        (17, 2) => d17p2::run(),
        (18, 1) => d18p1::run(),
        (18, 2) => d18p2::run(),
        // 
        (21, 1) => d21p1::run(),
        (21, 2) => d21p2::run(),
        _ => {
            println!("Not yet implemented");
            std::process::exit(2);
        }
    }
}
