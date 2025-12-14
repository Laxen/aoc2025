use std::fs;
use std::cmp;

fn merge_range(a: (u64, u64), b: (u64, u64)) -> Option<(u64, u64)> {
    let mut start_range = &b;
    let mut end_range = &a;

    if a.0 <= b.0 {
        start_range = &a;
        end_range = &b;
    }

    if end_range.0 >= start_range.0 && end_range.0 <= start_range.1 + 1 {
        return Some((start_range.0, cmp::max(start_range.1, end_range.1)));
    }

    return None;
}

fn merge_loop(ranges: &Vec<(u64, u64)>) -> Option<Vec<(u64, u64)>> {
    let mut merged_ranges = ranges.clone();

    for a in 0..ranges.len() {
        for b in (a + 1)..ranges.len() {
            if let Some(merged) = merge_range(ranges[a], ranges[b]) {
                merged_ranges[a] = merged;
                merged_ranges.remove(b);
                return Some(merged_ranges);
            }
        }
    }

    return None;
}

fn main() {
    let contents =
        fs::read_to_string("inputs/day05/input.txt").expect("Failed to read example.txt");

    let mut ranges = Vec::<(u64, u64)>::new();

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

    while let Some(merged) = merge_loop(&ranges) {
        ranges = merged;
    }

    println!("Merged ranges: {:?}", ranges);

    let mut count = 0;
    for range in ranges {
        count += range.1 - range.0 + 1;
    }
    println!("Count: {}", count);
}