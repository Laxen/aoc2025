use std::{cmp::max, fs};

fn area(a: (i32, i32), b: (i32, i32)) -> u64 {
    let x = (a.0 - b.0).abs() as u64;
    let y = (a.1 - b.1).abs() as u64;

    return (x + 1) * (y + 1);
}

fn main() {
    let contents =
        fs::read_to_string("inputs/day09/input.txt").expect("Failed to read example.txt");

    let mut coords = Vec::new();

    for line in contents.lines() {
        let mut parts = line.split(',');
        let (x, y) = (
            parts.next().unwrap().parse::<i32>().unwrap(),
            parts.next().unwrap().parse::<i32>().unwrap(),
        );

        coords.push((x, y));
    }

    let mut max_area = 0;
    for a in 0..coords.len() {
        for b in (a + 1)..coords.len() {
            let area = area(coords[a], coords[b]);

            max_area = max(max_area, area)
        }
    }

    println!("{}", max_area);
}