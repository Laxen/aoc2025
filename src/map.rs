pub enum NeighborType {
    Four,  /* The 4 orthogonal cells */
    Eight, /* The 8 surrounding cells */
}

#[derive(Clone, PartialEq)]
pub struct Map<T> {
    pub grid: Vec<Vec<T>>,
}

impl<T: std::fmt::Display + PartialEq> Map<T> {
    pub fn new() -> Self {
        return Map { grid: Vec::new() };
    }

    pub fn from_vec(vec: Vec<Vec<T>>) -> Self {
        return Map { grid: vec };
    }

    pub fn print(&self) {
        for row in &self.grid {
            for elem in row {
                print!("{}", elem);
            }
            println!();
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if y >= self.grid.len() || x >= self.grid[y].len() {
            return None;
        }
        return Some(&self.grid[y][x]);
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if y < self.grid.len() && x < self.grid[y].len() {
            self.grid[y][x] = value;
        }
    }

    pub fn get_neighbors(&self, x: usize, y: usize, neighbor_type: NeighborType) -> Vec<&T> {
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

    pub fn count(&self, target: T) -> usize
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
    pub fn from_string(s: &str) -> Self {
        return Map::from_vec(s.lines().map(|line| line.chars().collect()).collect());
    }
}
