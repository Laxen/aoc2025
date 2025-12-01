use std::fs;

fn main() {
    let contents = fs::read_to_string("inputs/day01/input.txt").expect("Failed to read example.txt");

    let mut position = 50;
    let mut n_zeroes = 0;
    for line in contents.lines() {
        let direction = line.chars().next().expect("Empty line");
        let amount: i32 = line[1..].parse().expect("Failed to parse number");

        if direction == 'R' {
            position += amount;
        } else if direction == 'L' {
            position -= amount;
        } else {
            panic!("Invalid direction in line: {}", line);
        }

        /* Ensure position stays within 0-99 */
        position = ((position % 100) + 100) % 100;

        if position == 0 {
            n_zeroes += 1;
        }
    }

    println!("{}", n_zeroes);
}
