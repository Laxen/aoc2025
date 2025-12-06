use std::fs;

fn get_max(numbers: &str, length: u32) -> (u32, usize) {
    let mut max_digit = 0;
    let mut digit_idx = 0;

    println!(
        "Getting max digit for length {} in numbers {}",
        length,
        numbers,
    );

    for (idx, c) in numbers.chars().enumerate() {
        let digit = c.to_digit(10).unwrap();

        if digit > max_digit && numbers.len() - idx >= length as usize {
            max_digit = digit;
            digit_idx = idx;
        }
    }

    println!("  Max digit: {} at index {}", max_digit, digit_idx);

    return (max_digit, digit_idx);
}

fn main() {
    let contents =
        fs::read_to_string("inputs/day03/input.txt").expect("Failed to read example.txt");

    let mut total_joltage = 0;

    for bank in contents.lines() {
        println!("Bank: {}", bank);

        let mut remaining_bank = bank;
        let mut joltage = 0;
        for i in (0..12).rev() {
            let (max_digit, digit_idx) = get_max(remaining_bank, i + 1);

            joltage += max_digit as u64 * 10u64.pow(i as u32);
            remaining_bank = &remaining_bank[digit_idx + 1..];
        }

        total_joltage += joltage;

        println!("---");
    }

    println!("Total joltage: {}", total_joltage);
}
