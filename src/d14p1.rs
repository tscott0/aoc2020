use std::collections::HashMap;

use crate::utils::string_from_file;

pub fn run() {
    let input = string_from_file("src/14input");

    println!("{}", solve(input));
}

pub fn solve(input: String) -> usize {
    let mut mask_one = 0b0u64;
    let mut mask_zero = 0b0u64;

    let mut memory: HashMap<u64, u64> = HashMap::new();
    input
        .lines()
        .map(|l| l.trim())
        .filter_map(|l| {
            let t = l.split(" = ").collect::<Vec<_>>();
            if t[0] == "mask" {
                let ones = t[1].replace('X', "0");
                let zeroes = t[1]
                    .replace('1', "X") // leave only 0 or X
                    .replace('0', "1") // swap 0 to 1
                    .replace('X', "0"); // swap X to 0 (0X1 should become 100)

                mask_one = u64::from_str_radix(ones.as_str(), 2).unwrap();
                mask_zero = u64::from_str_radix(zeroes.as_str(), 2).unwrap();
                None
            } else {
                let address = t[0]
                    .replace("mem[", "")
                    .replace("]", "")
                    .parse::<u64>()
                    .unwrap();

                let mut value = t[1].parse::<u64>().unwrap();

                value = value | mask_one;
                value = value & !mask_zero;

                Some((address, value))
            }
        })
        .for_each(|(a, v)| {
            memory.insert(a, v);
        });

    memory.iter().fold(0, |acc, (_, &v)| acc + v as usize)
}

#[test]
fn example_1() {
    let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
    mem[8] = 11
    mem[7] = 101
    mem[8] = 0";

    assert_eq!(solve(input.to_string()), 165);
}
