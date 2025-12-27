use std::{fs, vec};

struct Region {
    size: u32,
    presents: Vec<u32>,
}

fn main() {
    let contents = fs::read_to_string("inputs/day12/input.txt").expect("Failed to read input.txt");

    let shapes = vec![
        vec![
            vec!['#', '#', '#'],
            vec!['#', '.', '#'],
            vec!['#', '.', '#'],
        ],
        vec![
            vec!['#', '.', '.'],
            vec!['#', '#', '.'],
            vec!['#', '#', '#'],
        ],
        vec![
            vec!['#', '#', '#'],
            vec!['#', '#', '#'],
            vec!['.', '.', '#'],
        ],
        vec![
            vec!['.', '#', '#'],
            vec!['#', '#', '.'],
            vec!['#', '#', '#'],
        ],
        vec![
            vec!['#', '#', '#'],
            vec!['.', '#', '.'],
            vec!['#', '#', '#'],
        ],
        vec![
            vec!['.', '#', '#'],
            vec!['#', '#', '.'],
            vec!['#', '.', '.'],
        ],
    ];

    let mut regions = Vec::new();

    for line in contents.lines() {
        let parts = line.split(": ").collect::<Vec<&str>>();
        let size: u32 = parts[0]
            .split('x')
            .map(|x| x.parse::<u32>().unwrap())
            .product();
        let presents: Vec<u32> = parts[1]
            .split(' ')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        regions.push(Region { size, presents });
    }

    let mut shape_size = Vec::new();

    for shape in shapes {
        let mut size = 0;
        for row in shape {
            for cell in row {
                if cell == '#' {
                    size += 1;
                }
            }
        }
        shape_size.push(size);
    }

    let mut fit_regions = 0;
    for region in regions {
        let mut smallest_size = 0;
        let mut biggest_size = 0;

        for (i, n) in region.presents.iter().enumerate() {
            smallest_size += shape_size[i] * n;
            biggest_size += 3 * 3 * n;
        }

        if region.size >= biggest_size {
            fit_regions += 1;
        } else if region.size >= smallest_size {
            println!("Maybe fits");
        }
    }

    println!("{}", fit_regions);
}
