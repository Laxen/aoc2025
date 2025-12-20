use std::fs;
use aoc2025::map;

fn main() {
    let contents =
        fs::read_to_string("inputs/day07/input.txt").expect("Failed to read example.txt");

    let mut map = map::Map::from_string(&contents);
    let mut splits = 0;

    for y in 0..map.grid.len() {
        for x in 0..map.grid[y].len() {
            let e = *map.get(x, y).unwrap();

            if y == 0 {
                if e == 'S' {
                    map.set(x, y, '|');
                }
                continue;
            }

            let e_up = *map.get(x, y - 1).unwrap();
            if e_up == '|' {
                match e {
                    '.' => {
                        map.set(x, y, '|');
                    }
                    '^' => {
                        map.set(x - 1, y, '|');
                        map.set(x + 1, y, '|');
                        splits += 1;
                    }
                    _ => {}
                }
            }
        }
    }

    map.print();
    println!("{}", splits);
}