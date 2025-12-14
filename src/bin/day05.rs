use std::fs;

fn main() {
    let contents =
        fs::read_to_string("inputs/day05/input.txt").expect("Failed to read example.txt");

    let mut ranges = Vec::<(u64, u64)>::new();
    let mut ids = Vec::<u64>::new();

    for line in contents.lines() {
        if line.is_empty() {
            break;
        }

        let dash = line.find('-').unwrap();
        ranges.push((
            line[0..dash].parse().unwrap(),
            line[dash + 1..].parse().unwrap(),
        ));
    }

    for line in contents.lines().skip(ranges.len() + 1) {
        ids.push(line.parse().unwrap());
    }

    let fresh: Vec<&u64> = ids
        .iter()
        .filter(|&&id| ranges.iter().any(|&x| id >= x.0 && id <= x.1))
        .collect();

    println!("Total fresh IDs: {}", fresh.len());
}
