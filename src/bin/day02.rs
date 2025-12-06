use std::fs;

fn brute(start: &str, end: &str) -> Vec<u64> {
    let start_num: u64 = start.parse().unwrap();
    let end_num: u64 = end.parse().unwrap();

    let mut ret = Vec::<u64>::new();

    // println!("Brute forcing from {} to {}", start_num, end_num);

    for num in start_num..=end_num {
        let s = num.to_string();

        // println!("{}", s);
        for i in 0..s.len() / 2 {
            let pattern = &s[..i + 1];
            let repeat_pattern = pattern.repeat(s.len() / pattern.len());

            // println!("Pattern: {}", repeat_pattern);

            if repeat_pattern == s {
                // println!("Found invalid number: {}", num);
                ret.push(num);
                break;
            }
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