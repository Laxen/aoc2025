use aoc2025::map;
use std::{collections::HashMap, fs};

fn main() {
    let contents =
        fs::read_to_string("inputs/day07/input.txt").expect("Failed to read example.txt");

    let mut map = map::Map::from_string(&contents);
    let mut splits: HashMap<(usize, usize), u64> = HashMap::new();

    for y in 0..map.grid.len() {
        for x in 0..map.grid[y].len() {
            let e = *map.get(x, y).unwrap();

            if y == 0 {
                if e == 'S' {
                    map.set(x, y, '|');
                    *splits.entry((x, y)).or_insert(0) += 1;
                }
                continue;
            }

            let e_up = *map.get(x, y - 1).unwrap();
            if e_up == '|' {
                let timelines = *splits.entry((x, y - 1)).or_insert(0);

                match e {
                    '.' => {
                        map.set(x, y, '|');
                        *splits.entry((x, y)).or_insert(0) += timelines;
                    }
                    '^' => {
                        map.set(x - 1, y, '|');
                        map.set(x + 1, y, '|');

                        *splits.entry((x - 1, y)).or_insert(0) += timelines;
                        *splits.entry((x + 1, y)).or_insert(0) += timelines;
                    }
                    '|' => {
                        *splits.entry((x, y)).or_insert(0) += timelines;
                    }
                    _ => {}
                }
            }
        }
    }

    let sum: u64 = splits
        .iter()
        .filter(|(&(_, y), &_)| y == map.grid.len() - 1)
        .map(|(_, v)| v)
        .sum();
    println!("{}", sum);
}
