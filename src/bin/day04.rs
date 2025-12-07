use std::fmt::Display;
use std::fs;

enum NeighborType {
    Four,  /* The 4 orthogonal cells */
    Eight, /* The 8 surrounding cells */
}

#[derive(Clone, PartialEq)]
struct Map<T> {
    grid: Vec<Vec<T>>,
}

impl<T: Display + PartialEq> Map<T> {
    fn new() -> Self {
        return Map { grid: Vec::new() };
    }

    fn from_vec(vec: Vec<Vec<T>>) -> Self {
        return Map { grid: vec };
    }

    fn print(&self) {
        for row in &self.grid {
            for elem in row {
                print!("{}", elem);
            }
            println!();
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<&T> {
        if y >= self.grid.len() || x >= self.grid[y].len() {
            return None;
        }
        return Some(&self.grid[y][x]);
    }

    fn set(&mut self, x: usize, y: usize, value: T) {
        if y < self.grid.len() && x < self.grid[y].len() {
            self.grid[y][x] = value;
        }
    }

    fn get_neighbors(&self, x: usize, y: usize, neighbor_type: NeighborType) -> Vec<&T> {
        let mut neighbors = Vec::new();

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dy == 0 && dx == 0 {
                    continue;
                }

                /* Skip diagonals for 4-neighbor mode */
                if matches!(neighbor_type, NeighborType::Four) && dy != 0 && dx != 0 {
                    continue;
                }

                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx < 0 || ny < 0 {
                    continue;
                }

                if let Some(neighbor) = self.get(nx as usize, ny as usize) {
                    neighbors.push(neighbor);
                }
            }
        }

        return neighbors;
    }

    fn count(&self, target: T) -> usize
    where
        T: Copy,
    {
        let mut count = 0;

        for row in &self.grid {
            for elem in row {
                if *elem == target {
                    count += 1;
                }
            }
        }

        return count;
    }
}

impl Map<char> {
    fn from_string(s: &str) -> Self {
        return Map::from_vec(s.lines().map(|line| line.chars().collect()).collect());
    }
}

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
        fs::read_to_string("inputs/day04/example.txt").expect("Failed to read example.txt");

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
