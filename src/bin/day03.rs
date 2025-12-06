use std::fs;

fn main() {
    let contents =
        fs::read_to_string("inputs/day03/input.txt").expect("Failed to read example.txt");

    let mut total_joltage = 0;

    for bank in contents.lines() {
        let mut max_joltage = 0;

        println!("Bank: {}", bank);
        for (i, c) in bank.chars().enumerate() {
            for j in bank[i + 1..].chars() {
                let joltage = format!("{}{}", c, j).parse::<u32>().unwrap();

                if joltage > max_joltage {
                    max_joltage = joltage;
                }
            }
        }
        println!("Max joltage: {}", max_joltage);
        total_joltage += max_joltage;
    }

    println!("Total joltage: {}", total_joltage);
}
