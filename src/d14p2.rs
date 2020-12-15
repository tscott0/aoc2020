use std::collections::HashMap;

use crate::utils::string_from_file;

pub fn run() {
    let input = string_from_file("src/14input");

    println!("{}", solve(input));
}

pub fn solve(input: String) -> u64 {
    let mut mask_one = 0b0u64;
    let mut floating: Vec<usize> = vec![];
    let mut memory: HashMap<u64, u64> = HashMap::new();

    input.lines().map(|l| l.trim()).for_each(|l| {
        let t = l.split(" = ").collect::<Vec<_>>();
        if t[0] == "mask" {
            let ones = t[1].replace('X', "0");
            mask_one = u64::from_str_radix(ones.as_str(), 2).unwrap();

            floating = t[1]
                .chars()
                .rev()
                .enumerate()
                .filter_map(|(i, c)| if c == 'X' { Some(i) } else { None })
                .collect::<Vec<usize>>();
        } else {
            let address = t[0]
                .replace("mem[", "")
                .replace("]", "")
                .parse::<u64>()
                .unwrap();

            let value = t[1].parse::<u64>().unwrap();

            let max = (2 as u32).pow(floating.len() as u32);

            // Loop up to 2^n where n is the number of floating bits
            (0..max)
                .map(|p| {
                    let mut masked_address = address | mask_one;
                    // Loop and shift by the position of each floating bit
                    for i in 0..floating.len() {
                        let bit_is_1 = p & (1 << i) != 0; // is bit at i non-zero?
                        let mask = 1 << floating[i];

                        // Mask with OR for 1 and NAND for 0
                        if bit_is_1 {
                            masked_address = masked_address | mask
                        } else {
                            masked_address = masked_address & !mask
                        }
                    }
                    masked_address
                })
                .for_each(|a| {
                    memory.insert(a, value);
                });
        }
    });

    memory.values().map(|&v| v).sum::<u64>()
}

#[test]
fn example_1() {
    let input = "mask = 000000000000000000000000000000X1001X
    mem[42] = 100
    mask = 00000000000000000000000000000000X0XX
    mem[26] = 1";

    assert_eq!(solve(input.to_string()), 208);
}
