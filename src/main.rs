use std::env;

mod utils;
mod d1p1;
mod d1p2;
mod d2p1;
mod d2p2;
mod d3p1;
mod d3p2;
mod d4p1;
mod d4p2;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1..].len() != 2 {
        println!("Usage: aoc2020 <day> <part>");
    }

    let day = args[1].parse::<i32>().unwrap();
    let part = args[2].parse::<i32>().unwrap();

    if day < 1 || day > 25 || part < 1 || part > 2 {
        println!("Day must be 1-25 and part must be 1 or 2");
        std::process::exit(1)
    }

    println!("Day {:?} Part {:?}", day, part);

    match (day, part) {
        (1, 1) => d1p1::run(),
        (1, 2) => d1p2::run(),
        (2, 1) => d2p1::run(),
        (2, 2) => d2p2::run(),
        (3,1) => d3p1::run(),
        (3,2) => d3p2::run(),
        (4,1) => d4p1::run(),
        (4,2) => d4p2::run(),
        _ => {
            println!("Not yet implemented");
            std::process::exit(2);
        },
    }
}


