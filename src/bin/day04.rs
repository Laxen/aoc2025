use std::fs;
use aoc2025::map::{Map, NeighborType};

fn remove_rolls(map: &Map<char>) -> Map<char> {
    let mut ret_map = map.clone();

    let rows = map.grid.len();
    let cols = map.grid[0].len();

    for y in 0..rows {
        for x in 0..cols {
            if let Some('.') = map.get(x, y) {
                continue;
            }

            let neighbors = map.get_neighbors(x, y, NeighborType::Eight);

            if neighbors.iter().filter(|&&c| *c == '@').count() < 4 {
                ret_map.set(x, y, '.');
            }
        }
    }

    return ret_map;
}

fn main() {
    let contents =
        fs::read_to_string("inputs/day04/input.txt").expect("Failed to read example.txt");

    let mut map = Map::from_string(&contents);

    let start_rolls = map.count('@');

    loop {
        let new_map = remove_rolls(&map);

        if new_map == map {
            break;
        }

        map = new_map;
    }

    let end_rolls = map.count('@');
    println!("Rolls removed: {}", start_rolls - end_rolls);
}
