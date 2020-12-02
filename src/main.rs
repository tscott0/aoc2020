use std::env;

mod d1p1;
mod d1p2;
mod utils;

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
        _ => {
            println!("Not yet implemented");
            std::process::exit(2);
        },
    }
}


