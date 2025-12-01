use std::fs;

fn main() {
    let contents = fs::read_to_string("inputs/day01/input.txt").expect("Failed to read example.txt");

    println!("{:<10} {:<10} {:<15} {:<10} {:<10}", "Position", "Move", "New Position", "Final Pos", "Rotations");
    println!("{:-<10} {:-<10} {:-<15} {:-<10} {:-<10}", "", "", "", "", "");

    let mut position = 50;
    let mut total_rotations = 0;

    for line in contents.lines() {
        let direction = line.chars().next().expect("Empty line");
        let amount: i32 = line[1..].parse().expect("Failed to parse number");

        let new_position;
        if direction == 'R' {
            new_position = position + amount;
        } else if direction == 'L' {
            new_position = position - amount;
        } else {
            panic!("Invalid direction in line: {}", line);
        }

        let mut rotations = new_position.abs() / 100;
        if new_position == 0 {
            rotations += 1;
        }

        /* Add one rotation if new_position has different sign than position */
        if position.signum() != new_position.signum() && position != 0 && new_position != 0 {
            rotations += 1;
        }

        let new_final_position = (new_position % 100 + 100) % 100;

        println!("{:<10} {:<10} {:<15} {:<10} {:<10}", position, line, new_position, new_final_position, rotations);

        position = new_final_position;
        total_rotations += rotations;
    }

    println!("Total rotations: {}", total_rotations);
}
