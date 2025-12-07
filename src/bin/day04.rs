use std::fs;

fn print_map(map: &Vec<Vec<char>>) {
    for row in map {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn main() {
    let contents =
        fs::read_to_string("inputs/day04/input.txt").expect("Failed to read example.txt");

    let mut map: Vec<Vec<char>> = Vec::new();

    for line in contents.lines() {
        let mut row = Vec::new();

        for c in line.chars() {
            row.push(c);
        }

        map.push(row);
    }

    let rows = map.len();
    let cols = map[0].len();

    let mut accessible = 0;

    for y in 0..rows {
        for x in 0..cols {
            if map[y][x] == '.' { continue; }

            let min_y = if y > 0 { y - 1 } else { 0 };
            let max_y = (y + 1).min(rows - 1);
            let min_x = if x > 0 { x - 1 } else { 0 };
            let max_x = (x + 1).min(cols - 1);

            let mut neighbors = 0;

            for ny in min_y..=max_y {
                for nx in min_x..=max_x {
                    if ny == y && nx == x { continue; }

                    if map[ny][nx] == '@' {
                        neighbors += 1;
                    }
                }
            }

            if neighbors < 4 {
                accessible += 1;
            }
        }
    }

    print_map(&map);
    println!("Accessible positions: {}", accessible);
}
