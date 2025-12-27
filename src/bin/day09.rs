use std::fs;

fn check_intersection(a: (i32, i32), b: (i32, i32), coords: &Vec<(i32, i32)>) -> bool {
    let n = coords.len();

    let x_min = a.0.min(b.0);
    let y_min = a.1.min(b.1);
    let x_max = a.0.max(b.0);
    let y_max = a.1.max(b.1);

    for i in 0..n {
        let (x0, y0) = coords[i];
        let (x1, y1) = coords[(i + 1) % n];

        let edge_x_min = x0.min(x1);
        let edge_y_min = y0.min(y1);
        let edge_x_max = x0.max(x1);
        let edge_y_max = y0.max(y1);

        if x_max > edge_x_min && x_min < edge_x_max && y_max > edge_y_min && y_min < edge_y_max {
            return true;
        }
    }

    return false;
}

fn check_inside(point: (i32, i32), coords: &Vec<(i32, i32)>) -> bool {
    let n = coords.len();
    let mut inside = false;

    for i in 0..n {
        let (xi, yi) = coords[i];
        let (xj, yj) = coords[(i + 1) % n];

        if ((yi > point.1) != (yj > point.1))
            && (point.0 < (xj - xi) * (point.1 - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
    }

    return inside;
}

fn check_area(a: (i32, i32), b: (i32, i32), coords: &Vec<(i32, i32)>) -> u64 {
    if check_intersection(a, b, coords) {
        return 0;
    }

    let center = ((a.0 + b.0) / 2, (a.1 + b.1) / 2);

    if !check_inside(center, coords) {
        return 0;
    }

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

    let mut max_area: u64 = 0;
    let n = coords.len();
    for i in 0..n {
        for j in (i + 1)..n {
            max_area = max_area.max(check_area(coords[i], coords[j], &coords));
        }
    }

    println!("{}", max_area);
}
