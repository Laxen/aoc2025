use std::fs;

fn brute(start: &str, end: &str) -> Vec<u64> {
    let start_num: u64 = start.parse().unwrap();
    let end_num: u64 = end.parse().unwrap();

    let mut ret = Vec::<u64>::new();

    println!("Brute forcing from {} to {}", start_num, end_num);

    for num in start_num..=end_num {
        let s = num.to_string();

        if s.len() % 2 != 0 {
            continue;
        }

        let mid = s.len() / 2;
        let first_half = &s[0..mid];
        let second_half = &s[mid..];

        if first_half == second_half {
            println!("  Found invalid ID: {}", s);
            ret.push(num);
        }
    }

    return ret;
}

fn main() {
    let contents = fs::read_to_string("inputs/day02/input.txt").expect("Failed to read example.txt");
    let line = contents.lines().next().unwrap();

    let mut sum = 0;
    for range in line.split(",") {
        let (start, end) = range.split_once("-").unwrap();
        let invalid_ids = brute(start, end);

        sum += invalid_ids.iter().sum::<u64>();
    }

    println!("Total sum: {}", sum);
}